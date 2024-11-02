use actix_web::{web, HttpResponse};
use crate::application::use_cases::message_use_cases::{SendMessageUseCase, GetMessagesUseCase};
use crate::infrastructure::websocket::realtime_message_manager::RealtimeMessageManager;
use crate::domain::repositories::message_repository::MessageRepository;

pub struct MessageHandlers<T: MessageRepository> {
    send_message_use_case: SendMessageUseCase<T>,
    get_messages_use_case: GetMessagesUseCase<T>,
    realtime_message_manager: RealtimeMessageManager,
}

impl<T: MessageRepository> MessageHandlers<T> {
    pub fn new(
        send_message_use_case: SendMessageUseCase<T>,
        get_messages_use_case: GetMessagesUseCase<T>,
        realtime_message_manager: RealtimeMessageManager,
    ) -> Self {
        Self {
            send_message_use_case,
            get_messages_use_case,
            realtime_message_manager,
        }
    }

    // Example handler methods
    pub async fn send_message(
        &self,
        sender_id: i32,
        receiver_id: i32,
        content: String,
    ) -> Result<HttpResponse, actix_web::Error> {
        // Save to database
        let message = self.send_message_use_case
            .execute(sender_id, receiver_id, content.clone())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // Send real-time message
        self.realtime_message_manager
            .send_message(sender_id, receiver_id, content)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        Ok(HttpResponse::Ok().json(message))
    }

    pub async fn get_messages(
        &self,
        user1_id: i32,
        user2_id: i32,
    ) -> Result<HttpResponse, actix_web::Error> {
        let messages = self.get_messages_use_case
            .execute(user1_id, user2_id)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        Ok(HttpResponse::Ok().json(messages))
    }
}

// Add configuration function for routes
pub fn configure<T: MessageRepository + 'static>(
    cfg: &mut web::ServiceConfig,
    _handlers: web::Data<MessageHandlers<T>>,
) {
    cfg.service(
        web::scope("/messages")
            .route("", web::post().to(move |
                sender_id: web::Path<i32>,
                receiver_id: web::Path<i32>,
                content: web::Json<String>,
                handlers: web::Data<MessageHandlers<T>>,
            | async move {
                handlers.send_message(
                    sender_id.into_inner(),
                    receiver_id.into_inner(),
                    content.into_inner(),
                ).await
            }))
            .route("/{user1_id}/{user2_id}", web::get().to(move |
                path: web::Path<(i32, i32)>,
                handlers: web::Data<MessageHandlers<T>>,
            | async move {
                let (user1_id, user2_id) = path.into_inner();
                handlers.get_messages(user1_id, user2_id).await
            }))
    );
}