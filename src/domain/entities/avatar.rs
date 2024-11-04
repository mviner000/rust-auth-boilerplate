use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Avatar {
    pub id: i32,
    pub account_id: i32,
    pub avatar_300x300_url: Option<String>,
    pub avatar_40x40_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct AvatarUploadResponse {
    pub avatar_300x300_url: String,
    pub avatar_40x40_url: String,
    pub message: String,
}