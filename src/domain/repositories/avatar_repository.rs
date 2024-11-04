use async_trait::async_trait;
use crate::domain::entities::avatar::Avatar;

#[async_trait]
pub trait AvatarRepository {
    async fn create(&self, account_id: i32, avatar_300x300_url: String, avatar_40x40_url: String) -> Result<Avatar, Box<dyn std::error::Error>>;
    async fn find_by_account_id(&self, account_id: i32) -> Result<Vec<Avatar>, Box<dyn std::error::Error>>;
    async fn find_latest_by_account_id(&self, account_id: i32) -> Result<Option<Avatar>, Box<dyn std::error::Error>>;
}
