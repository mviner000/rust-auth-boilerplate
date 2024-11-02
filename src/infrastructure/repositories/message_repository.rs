use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use async_trait::async_trait;
use crate::domain::entities::message::DatabaseMessage;
use crate::domain::repositories::message_repository::MessageRepository;

#[derive(Clone)]
pub struct MessageRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl MessageRepositoryImpl {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageRepository for MessageRepositoryImpl {
    async fn save_message(&self, message: DatabaseMessage) -> Result<DatabaseMessage, String> {
        // Implementation details here
        // You can use self.pool.get() to get a connection
        todo!("Implement save_message")
    }

    async fn get_messages(&self, user1_id: i32, user2_id: i32) -> Result<Vec<DatabaseMessage>, String> {
        // Implementation details here
        // You can use self.pool.get() to get a connection
        todo!("Implement get_messages")
    }

    async fn mark_as_read(&self, message_id: i32) -> Result<(), String> {
        // Implementation details here
        // You can use self.pool.get() to get a connection
        todo!("Implement mark_as_read")
    }
}