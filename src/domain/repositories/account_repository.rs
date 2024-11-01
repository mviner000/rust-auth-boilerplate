use async_trait::async_trait;
use crate::domain::entities::account::{Account, UpdateAccountDto};

#[async_trait]
pub trait AccountRepository {
    async fn find_by_user_id(&self, user_id: i32) -> Result<Account, Box<dyn std::error::Error>>;
    async fn update(&self, user_id: i32, account: UpdateAccountDto) -> Result<Account, Box<dyn std::error::Error>>;
    async fn update_avatar(&self, user_id: i32, avatar_300x300_url: String, avatar_40x40_url: String) -> Result<Account, Box<dyn std::error::Error>>;

}
