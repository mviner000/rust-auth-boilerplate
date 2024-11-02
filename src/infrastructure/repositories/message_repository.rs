use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, RunQueryDsl};
use diesel::prelude::*;
use async_trait::async_trait;
use crate::domain::entities::message::DatabaseMessage;
use crate::domain::repositories::message_repository::MessageRepository;
use crate::schema::messages;

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
        let mut conn = self.pool.get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?;

        // Using tokio::task::spawn_blocking for diesel sync operations
        let result = tokio::task::spawn_blocking(move || {
            diesel::insert_into(messages::table)
                .values((
                    messages::sender_id.eq(message.sender_id),
                    messages::receiver_id.eq(message.receiver_id),
                    messages::content.eq(message.content),
                    messages::is_read.eq(message.is_read),
                    messages::created_at.eq(message.created_at),
                ))
                .get_result::<DatabaseMessage>(&mut conn)
        }).await
            .map_err(|e| format!("Task failed: {}", e))?
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(result)
    }

    async fn get_messages(&self, user1_id: i32, user2_id: i32) -> Result<Vec<DatabaseMessage>, String> {
        let mut conn = self.pool.get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?;

        let result = tokio::task::spawn_blocking(move || {
            messages::table
                .filter(
                    messages::sender_id.eq(user1_id)
                        .and(messages::receiver_id.eq(user2_id))
                        .or(messages::sender_id.eq(user2_id)
                            .and(messages::receiver_id.eq(user1_id)))
                )
                .order(messages::created_at.asc())
                .load::<DatabaseMessage>(&mut conn)
        }).await
            .map_err(|e| format!("Task failed: {}", e))?
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(result)
    }

    async fn mark_as_read(&self, message_id: i32) -> Result<(), String> {
        let mut conn = self.pool.get()
            .map_err(|e| format!("Failed to get DB connection: {}", e))?;

        let result = tokio::task::spawn_blocking(move || {
            diesel::update(messages::table.find(message_id))
                .set(messages::is_read.eq(true))
                .execute(&mut conn)
        }).await
            .map_err(|e| format!("Task failed: {}", e))?
            .map_err(|e| format!("Database error: {}", e))?;

        if result == 0 {
            return Err("Message not found".to_string());
        }

        Ok(())
    }
}