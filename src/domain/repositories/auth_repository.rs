use async_trait::async_trait;
use crate::domain::entities::{
    auth::{AuthUser, RegisterUserDto},
    user::User,
};

#[async_trait]
pub trait AuthRepository {
    async fn authenticate(&self, auth: AuthUser) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;
    async fn register(&self, register_dto: RegisterUserDto) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;
}