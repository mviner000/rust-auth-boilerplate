use async_trait::async_trait;
use crate::domain::entities::message::DatabaseMessage;

#[async_trait]
pub trait MessageRepository {
    async fn save_message(&self, message: DatabaseMessage) -> Result<DatabaseMessage, String>;
    async fn get_messages(&self, user1_id: i32, user2_id: i32) -> Result<Vec<DatabaseMessage>, String>;
    async fn mark_as_read(&self, message_id: i32) -> Result<(), String>;
}