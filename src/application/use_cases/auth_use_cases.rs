use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::domain::entities::auth::{AuthUser, Claims, TokenResponse};
use crate::domain::repositories::auth_repository::AuthRepository;

pub struct LoginUseCase<T: AuthRepository> {
    auth_repository: T,
    jwt_secret: String,
}

impl<T: AuthRepository> LoginUseCase<T> {
    pub fn new(auth_repository: T, jwt_secret: String) -> Self {
        Self {
            auth_repository,
            jwt_secret,
        }
    }

    pub async fn execute(&self, auth: AuthUser) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let user = self.auth_repository.authenticate(auth).await?;

        let now = Utc::now();
        let exp = (now + Duration::hours(24)).timestamp();
        let claims = Claims {
            sub: user.id,
            exp,
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: exp - now.timestamp(),
        })
    }
}