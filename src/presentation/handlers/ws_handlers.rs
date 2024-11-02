use actix::{Actor, StreamHandler, ActorContext, Running, AsyncContext, Handler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crate::infrastructure::websocket::connection_manager::ConnectionManager;
use crate::domain::entities::message::WebSocketMessage;

pub struct WebSocketActor {
    user_id: i32,
    connection_manager: ConnectionManager,
}

impl WebSocketActor {
    pub fn new(user_id: i32, connection_manager: ConnectionManager) -> Self {
        Self {
            user_id,
            connection_manager,
        }
    }
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let connection_manager = self.connection_manager.clone();
        let user_id = self.user_id;
        let addr = ctx.address();

        actix::spawn(async move {
            connection_manager.add_connection(user_id, addr).await;
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        let connection_manager = self.connection_manager.clone();
        let user_id = self.user_id;

        actix::spawn(async move {
            connection_manager.remove_connection(user_id).await;
        });
        Running::Stop
    }
}

impl Handler<WebSocketMessage> for WebSocketActor {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: WebSocketMessage, ctx: &mut Self::Context) -> Self::Result {
        match serde_json::to_string(&msg) {
            Ok(msg_str) => {
                ctx.text(msg_str);
                Ok(())
            }
            Err(e) => Err(format!("Failed to serialize message: {}", e))
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                log::debug!("Received WebSocket message: {}", text);

                match serde_json::from_str::<WebSocketMessage>(&text) {
                    Ok(ws_message) => {
                        log::debug!("Parsed message successfully: {:?}", ws_message);

                        match ws_message {
                            WebSocketMessage::Status { user_id, online } => {
                                let connection_manager = self.connection_manager.clone();
                                log::debug!("Processing status update for user {}: online = {}", user_id, online);

                                // Only broadcast if this is the user's own status
                                if user_id == self.user_id {
                                    actix::spawn(async move {
                                        if let Err(e) = connection_manager.broadcast_status_update(user_id, online).await {
                                            log::error!("Failed to broadcast status update: {}", e);
                                        }
                                    });
                                }
                            },
                            WebSocketMessage::Chat { to_user_id, content } => {
                                let connection_manager = self.connection_manager.clone();
                                let message = WebSocketMessage::Chat {
                                    to_user_id,
                                    content: content.clone(),
                                };
                                actix::spawn(async move {
                                    if let Err(e) = connection_manager.broadcast_to_user(to_user_id, message).await {
                                        log::error!("Failed to broadcast message: {}", e);
                                    }
                                });
                            },
                            WebSocketMessage::CallOffer { to_user_id, sdp } => {
                                let connection_manager = self.connection_manager.clone();
                                let message = WebSocketMessage::CallOffer {
                                    to_user_id,
                                    sdp: sdp.clone(),
                                };
                                actix::spawn(async move {
                                    if let Err(e) = connection_manager.broadcast_to_user(to_user_id, message).await {
                                        log::error!("Failed to broadcast call offer: {}", e);
                                    }
                                });
                            },
                            WebSocketMessage::CallAnswer { to_user_id, sdp } => {
                                let connection_manager = self.connection_manager.clone();
                                let message = WebSocketMessage::CallAnswer {
                                    to_user_id,
                                    sdp: sdp.clone(),
                                };
                                actix::spawn(async move {
                                    if let Err(e) = connection_manager.broadcast_to_user(to_user_id, message).await {
                                        log::error!("Failed to broadcast call answer: {}", e);
                                    }
                                });
                            },
                            WebSocketMessage::IceCandidate { to_user_id, candidate } => {
                                let connection_manager = self.connection_manager.clone();
                                let message = WebSocketMessage::IceCandidate {
                                    to_user_id,
                                    candidate: candidate.clone(),
                                };
                                actix::spawn(async move {
                                    if let Err(e) = connection_manager.broadcast_to_user(to_user_id, message).await {
                                        log::error!("Failed to broadcast ICE candidate: {}", e);
                                    }
                                });
                            },
                            WebSocketMessage::EndCall { to_user_id } => {
                                let connection_manager = self.connection_manager.clone();
                                let message = WebSocketMessage::EndCall { to_user_id };
                                actix::spawn(async move {
                                    if let Err(e) = connection_manager.broadcast_to_user(to_user_id, message).await {
                                        log::error!("Failed to broadcast end call: {}", e);
                                    }
                                });
                            },
                            WebSocketMessage::Error { message } => {
                                ctx.text(serde_json::to_string(&WebSocketMessage::Error {
                                    message: message.clone(),
                                }).unwrap());
                            }
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to parse WebSocket message: {}", e);
                        let error_msg = WebSocketMessage::Error {
                            message: format!("Invalid message format: {}", e),
                        };
                        if let Ok(error_string) = serde_json::to_string(&error_msg) {
                            ctx.text(error_string);
                        }
                    }
                }
            },
            Ok(ws::Message::Close(reason)) => {
                log::debug!("WebSocket close request received for user {}", self.user_id);
                ctx.close(reason);
                ctx.stop();
            },
            _ => (),
        }
    }
}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    user_id: web::Path<i32>,
    connection_manager: web::Data<ConnectionManager>,
) -> Result<HttpResponse, Error> {
    let actor = WebSocketActor::new(
        user_id.into_inner(),
        connection_manager.get_ref().clone(),
    );
    ws::start(actor, &req, stream)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ws/{user_id}")
            .route(web::get().to(ws_index))
    );
}