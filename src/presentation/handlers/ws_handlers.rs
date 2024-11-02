use actix::{Actor, StreamHandler, ActorContext, Running, AsyncContext, Handler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::sync::Arc;
use crate::infrastructure::websocket::{
    user_status_manager::UserStatusManager,
    realtime_message_manager::RealtimeMessageManager
};
use crate::domain::entities::message::WebSocketMessage;

pub struct WebSocketActor {
    user_id: i32,
    user_status_manager: Arc<UserStatusManager>,
    realtime_message_manager: Arc<RealtimeMessageManager>, // Changed to Arc
}

impl Clone for WebSocketActor {
    fn clone(&self) -> Self {
        Self {
            user_id: self.user_id,
            user_status_manager: Arc::clone(&self.user_status_manager),
            realtime_message_manager: Arc::clone(&self.realtime_message_manager),
        }
    }
}

impl WebSocketActor {
    pub fn new(
        user_id: i32,
        user_status_manager: Arc<UserStatusManager>,
        realtime_message_manager: RealtimeMessageManager,
    ) -> Self {
        Self {
            user_id,
            user_status_manager,
            realtime_message_manager: Arc::new(realtime_message_manager),
        }
    }
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let user_status_manager = Arc::clone(&self.user_status_manager);
        let user_id = self.user_id;
        let addr = ctx.address();

        actix::spawn(async move {
            user_status_manager.add_connection(user_id, addr).await;
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        let user_status_manager = Arc::clone(&self.user_status_manager);
        let user_id = self.user_id;

        actix::spawn(async move {
            user_status_manager.remove_connection(user_id).await;
        });
        Running::Stop
    }
}

impl Handler<WebSocketMessage> for WebSocketActor {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: WebSocketMessage, ctx: &mut Self::Context) -> Self::Result {
        let message = serde_json::to_string(&msg)
            .map_err(|e| format!("Failed to serialize message: {}", e))?;

        ctx.text(message);
        Ok(())
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<WebSocketMessage>(&text) {
                    Ok(websocket_msg) => {
                        match websocket_msg {
                            WebSocketMessage::Chat { to_user_id, content } => {
                                let realtime_manager = Arc::clone(&self.realtime_message_manager);
                                let from_user_id = self.user_id;
                                actix::spawn(async move {
                                    realtime_manager.send_message(from_user_id, to_user_id, content).await.ok();
                                });
                            },
                            WebSocketMessage::CallOffer { to_user_id, sdp } => {
                                let realtime_manager = Arc::clone(&self.realtime_message_manager);
                                let from_user_id = self.user_id;
                                // Handle WebRTC offer
                            },
                            WebSocketMessage::CallAnswer { to_user_id, sdp } => {
                                let realtime_manager = Arc::clone(&self.realtime_message_manager);
                                let from_user_id = self.user_id;
                                // Handle WebRTC answer
                            },
                            WebSocketMessage::IceCandidate { to_user_id, candidate } => {
                                let realtime_manager = Arc::clone(&self.realtime_message_manager);
                                let from_user_id = self.user_id;
                                // Handle ICE candidate
                            },
                            WebSocketMessage::EndCall { to_user_id } => {
                                let realtime_manager = Arc::clone(&self.realtime_message_manager);
                                let from_user_id = self.user_id;
                                // Handle call end
                            },
                            WebSocketMessage::Status { .. } | WebSocketMessage::Error { .. } => {
                                ctx.text(serde_json::json!({
                                    "type": "error",
                                    "message": "Invalid message type for client"
                                }).to_string());
                            }
                        }
                    },
                    Err(e) => {
                        let error_msg = WebSocketMessage::Error {
                            message: format!("Invalid message format: {}", e)
                        };
                        if let Ok(error_string) = serde_json::to_string(&error_msg) {
                            ctx.text(error_string);
                        }
                    }
                }
            },
            Ok(ws::Message::Binary(_)) => (),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            _ => (),
        }
    }
}

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<i32>,
    user_status_manager: web::Data<Arc<UserStatusManager>>,
    realtime_message_manager: web::Data<RealtimeMessageManager>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    let actor = WebSocketActor::new(
        user_id,
        user_status_manager.get_ref().clone(),
        realtime_message_manager.get_ref().clone(),
    );
    let resp = ws::start(actor, &req, stream)?;
    Ok(resp)
}


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ws/{user_id}")
            .route(web::get().to(ws_route))
    );
}