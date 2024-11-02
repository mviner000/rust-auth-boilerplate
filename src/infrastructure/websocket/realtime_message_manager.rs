use std::sync::Arc;
use crate::infrastructure::websocket::user_status_manager::UserStatusManager;
use crate::domain::entities::message::WebSocketMessage;

#[derive(Clone)]
pub struct RealtimeMessageManager {
    user_status_manager: Arc<UserStatusManager>,
}

impl RealtimeMessageManager {
    pub fn new(user_status_manager: Arc<UserStatusManager>) -> Self {
        Self {
            user_status_manager
        }
    }

    pub async fn send_message(&self, _from_user_id: i32, to_user_id: i32, content: String) -> Result<(), String> {
        if let Some(addr) = self.user_status_manager.get_connection(to_user_id).await {
            let message = WebSocketMessage::Chat {
                to_user_id,
                content,
            };
            addr.try_send(message)
                .map_err(|e| format!("Failed to send message: {}", e))
        } else {
            Err(format!("User {} is not connected", to_user_id))
        }
    }

    pub async fn broadcast_to_all(&self, message: WebSocketMessage) -> Result<(), String> {
        let connections = self.user_status_manager.get_online_status().await;
        for (user_id, _) in connections {
            if let Some(addr) = self.user_status_manager.get_connection(user_id).await {
                addr.try_send(message.clone())
                    .map_err(|e| format!("Failed to broadcast to user {}: {}", user_id, e))?;
            }
        }
        Ok(())
    }
}