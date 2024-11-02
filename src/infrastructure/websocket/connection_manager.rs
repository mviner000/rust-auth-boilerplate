use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use actix::Addr;
use crate::presentation::handlers::ws_handlers::WebSocketActor;
use crate::domain::entities::message::{Message, WebSocketMessage};

#[derive(Clone)]
pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<i32, Addr<WebSocketActor>>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }


    pub async fn get_online_status(&self) -> HashMap<i32, bool> {
        let connections = self.connections.read().await;
        let mut status_map = HashMap::new();
        for (user_id, _) in connections.iter() {
            status_map.insert(*user_id, true);
        }
        status_map
    }

    pub async fn add_connection(&self, user_id: i32, addr: Addr<WebSocketActor>) {
        let mut connections = self.connections.write().await;
        connections.insert(user_id, addr.clone());
        drop(connections); // Release the write lock before broadcasting

        // First, send the new user's online status to all existing connections
        self.broadcast_status_update(user_id, true).await.ok();

        // Then send the status of all existing users to the new connection
        let online_users = self.get_online_status().await;
        for (existing_user_id, is_online) in online_users {
            if existing_user_id != user_id {  // Don't send own status back
                let status_message = WebSocketMessage::Status {
                    user_id: existing_user_id,
                    online: is_online,
                };
                if let Err(e) = addr.try_send(status_message) {
                    log::error!("Failed to send initial status for user {}: {}", existing_user_id, e);
                }
            }
        }
    }

    pub async fn remove_connection(&self, user_id: i32) {
        let mut connections = self.connections.write().await;
        connections.remove(&user_id);
        drop(connections); // Release the write lock before broadcasting

        // Broadcast offline status to all remaining connections
        self.broadcast_status_update(user_id, false).await.ok();
    }

    pub async fn get_connection(&self, user_id: i32) -> Option<Addr<WebSocketActor>> {
        let connections = self.connections.read().await;
        connections.get(&user_id).cloned()
    }

    pub async fn broadcast_to_all(&self, message: WebSocketMessage) -> Result<(), String> {
        let connections = self.connections.read().await;
        for (_, addr) in connections.iter() {
            if let Err(e) = addr.try_send(message.clone()) {
                log::error!("Failed to send message: {}", e);
            }
        }
        Ok(())
    }

    pub async fn broadcast_to_user(&self, user_id: i32, message: WebSocketMessage) -> Result<(), String> {
        if let Some(connection) = self.get_connection(user_id).await {
            connection
                .try_send(message)
                .map_err(|e| format!("Failed to send message: {}", e))?;
        }
        Ok(())
    }

    pub async fn broadcast_status_update(&self, user_id: i32, online: bool) -> Result<(), String> {
        let status_message = WebSocketMessage::Status { user_id, online };
        let connections = self.connections.read().await;

            // Log the broadcast
            log::debug!(
            "Broadcasting status update - user_id: {}, online: {}, to {} connections",
            user_id,
            online,
            connections.len()
        );

        for (conn_user_id, addr) in connections.iter() {
            if *conn_user_id != user_id {  // Don't send status update to the user itself
                log::debug!("Sending status update to user {}", conn_user_id);
                if let Err(e) = addr.try_send(status_message.clone()) {
                    log::error!("Failed to send status update to user {}: {}", conn_user_id, e);
                }
            }
        }
        Ok(())
    }

    pub async fn get_all_online_users(&self) -> Vec<i32> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    }
}

// Add some error handling implementations
#[derive(Debug)]
pub enum WebSocketError {
    ConnectionNotFound(i32),
    MessageSendError(String),
    InternalError(String),
}

impl std::fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebSocketError::ConnectionNotFound(user_id) =>
                write!(f, "WebSocket connection not found for user {}", user_id),
            WebSocketError::MessageSendError(msg) =>
                write!(f, "Failed to send WebSocket message: {}", msg),
            WebSocketError::InternalError(msg) =>
                write!(f, "Internal WebSocket error: {}", msg),
        }
    }
}

impl std::error::Error for WebSocketError {}

// Helper methods for the ConnectionManager
impl ConnectionManager {
    pub async fn is_user_connected(&self, user_id: i32) -> bool {
        let connections = self.connections.read().await;
        connections.contains_key(&user_id)
    }

    pub async fn get_connected_users(&self) -> Vec<i32> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    }

    pub async fn get_connection_count(&self) -> usize {
        let connections = self.connections.read().await;
        connections.len()
    }

    // Safely send a message with proper error handling
    pub async fn safe_send_to_user(&self, user_id: i32, message: WebSocketMessage) -> Result<(), WebSocketError> {
        match self.get_connection(user_id).await {
            Some(connection) => {
                connection
                    .try_send(message)
                    .map_err(|e| WebSocketError::MessageSendError(e.to_string()))?;
                Ok(())
            }
            None => Err(WebSocketError::ConnectionNotFound(user_id))
        }
    }
}