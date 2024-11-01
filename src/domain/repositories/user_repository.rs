use async_trait::async_trait;
use crate::domain::entities::user::{User, CreateUserDto, UpdateUserDto};

#[async_trait]
pub trait UserRepository {

    async fn find_by_id(&self, user_id: i32) -> Result<User, Box<dyn std::error::Error>>;
    async fn create(&self, user: CreateUserDto) -> Result<User, Box<dyn std::error::Error>>;
    async fn find_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error>>;

    async fn update(&self, id: i32, user: UpdateUserDto) -> Result<User, Box<dyn std::error::Error>>;
    async fn delete(&self, user_id: i32) -> Result<(), Box<dyn std::error::Error>>;
}