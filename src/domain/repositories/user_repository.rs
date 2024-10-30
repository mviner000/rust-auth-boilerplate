use async_trait::async_trait;
use crate::domain::entities::user::{User, CreateUserDto};

#[async_trait]
pub trait UserRepository {

    async fn find_by_id(&self, user_id: i32) -> Result<User, Box<dyn std::error::Error>>;
    async fn create(&self, user: CreateUserDto) -> Result<User, Box<dyn std::error::Error>>;
    async fn find_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error>>;
}