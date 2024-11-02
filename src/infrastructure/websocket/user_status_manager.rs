use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use actix::Addr;
use crate::presentation::handlers::ws_handlers::WebSocketActor;

#[derive(Clone)]
pub struct UserStatusManager {
    connections: Arc<RwLock<HashMap<i32, Addr<WebSocketActor>>>>,
}

impl UserStatusManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_connection(&self, user_id: i32, addr: Addr<WebSocketActor>) {
        let mut connections = self.connections.write().await;
        // Since Addr<T> implements Clone when T: Actor, this will now work
        connections.insert(user_id, addr.clone());
        drop(connections);

        // Broadcast new user's online status
        self.broadcast_status_update(user_id, true).await.ok();

        // Send existing users' status to new user
        let online_users = self.get_online_status().await;
        for (existing_user_id, is_online) in online_users {
            if existing_user_id != user_id {
                self.send_status_to_user(addr.clone(), existing_user_id, is_online).await.ok();
            }
        }
    }

    pub async fn remove_connection(&self, user_id: i32) {
        let mut connections = self.connections.write().await;
        connections.remove(&user_id);
        drop(connections);
        self.broadcast_status_update(user_id, false).await.ok();
    }

    pub async fn get_online_status(&self) -> HashMap<i32, bool> {
        let connections = self.connections.read().await;
        let mut status_map = HashMap::new();
        for (user_id, _) in connections.iter() {
            status_map.insert(*user_id, true);
        }
        status_map
    }

    async fn broadcast_status_update(&self, user_id: i32, online: bool) -> Result<(), String> {
        let connections = self.connections.read().await;
        for (conn_user_id, addr) in connections.iter() {
            if *conn_user_id != user_id {
                self.send_status_to_user(addr.clone(), user_id, online).await?;
            }
        }
        Ok(())
    }

    pub async fn send_status_to_user(&self, addr: Addr<WebSocketActor>, user_id: i32, online: bool) -> Result<(), String> {
        use crate::domain::entities::message::WebSocketMessage;
        addr.try_send(WebSocketMessage::Status { user_id, online })
            .map_err(|e| format!("Failed to send status: {}", e))
    }

    pub async fn get_connection(&self, user_id: i32) -> Option<Addr<WebSocketActor>> {
        let connections = self.connections.read().await;
        connections.get(&user_id).cloned()
    }
}
