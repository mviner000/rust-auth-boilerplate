use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use actix::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseMessage {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
    pub is_read: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebSocketMessage {
    Chat {
        to_user_id: i32,
        content: String,
    },
    Status {
        user_id: i32,
        online: bool,
    },
    CallOffer {
        to_user_id: i32,
        sdp: String,
    },
    CallAnswer {
        to_user_id: i32,
        sdp: String,
    },
    IceCandidate {
        to_user_id: i32,
        candidate: String,
    },
    EndCall {
        to_user_id: i32,
    },
    Error {
        message: String,
    }
}

impl actix::Message for WebSocketMessage {
    type Result = Result<(), String>;
}

pub use self::DatabaseMessage as Message;