use async_trait::async_trait;
use crate::domain::entities::auth::AuthUser;
use crate::domain::entities::user::User;

#[async_trait]
pub trait AuthRepository {
    async fn authenticate(&self, auth: AuthUser) -> Result<User, Box<dyn std::error::Error>>;
}
