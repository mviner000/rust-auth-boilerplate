use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::domain::entities::avatar::Avatar;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub default_avatar_id: Option<i32>,
    pub default_avatar: Option<Avatar>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountDto {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
}
