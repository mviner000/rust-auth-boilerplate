use async_trait::async_trait;
use crate::domain::entities::user::User;

#[async_trait]
pub trait UserRepository {
    async fn find_by_name(&self, name: &str) -> Result<User, Box<dyn std::error::Error>>;
}