use async_trait::async_trait;
use crate::domain::entities::account::{Account, UpdateAccountDto};

#[async_trait]
pub trait AccountRepository {
    async fn find_by_user_id(&self, user_id: i32) -> Result<Account, Box<dyn std::error::Error>>;
    async fn update(&self, user_id: i32, account: UpdateAccountDto) -> Result<Account, Box<dyn std::error::Error>>;
    async fn set_default_avatar(&self, user_id: i32, avatar_id: i32) -> Result<Account, Box<dyn std::error::Error>>;
    async fn load_default_avatar(&self, account: &mut Account) -> Result<(), Box<dyn std::error::Error>>;
}