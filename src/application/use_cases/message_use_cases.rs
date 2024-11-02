use async_trait::async_trait;
use crate::domain::entities::message::DatabaseMessage;
use crate::domain::entities::message::WebSocketMessage;
use crate::domain::repositories::message_repository::MessageRepository;

pub struct SendMessageUseCase<T: MessageRepository> {
    message_repository: T,
}

impl<T: MessageRepository> SendMessageUseCase<T> {
    pub fn new(message_repository: T) -> Self {
        Self { message_repository }
    }

    pub async fn execute(&self, sender_id: i32, receiver_id: i32, content: String) -> Result<DatabaseMessage, String> {
        let message = DatabaseMessage {
            id: 0, // Will be set by the database
            sender_id,
            receiver_id,
            content,
            is_read: false,
            created_at: chrono::Utc::now().naive_utc(),
        };
        self.message_repository.save_message(message).await
    }
}

pub struct GetMessagesUseCase<T: MessageRepository> {
    message_repository: T,
}

impl<T: MessageRepository> GetMessagesUseCase<T> {
    pub fn new(message_repository: T) -> Self {
        Self { message_repository }
    }

    pub async fn execute(&self, user1_id: i32, user2_id: i32) -> Result<Vec<DatabaseMessage>, String> {
        self.message_repository.get_messages(user1_id, user2_id).await
    }
}