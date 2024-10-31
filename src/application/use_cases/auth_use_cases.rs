use std::fmt;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::domain::entities::auth::{AuthUser, Claims, TokenResponse};
use crate::domain::repositories::auth_repository::AuthRepository;

pub struct LoginUseCase<T: AuthRepository> {
    auth_repository: T,
    secret_key: String,
}

impl<T: AuthRepository> fmt::Debug for LoginUseCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoginUseCase")
            .field("auth_repository", &"AuthRepository")
            .field("secret_key", &"[REDACTED]")
            .finish()
    }
}

impl<T: AuthRepository> LoginUseCase<T> {
    pub fn new(auth_repository: T, secret_key: String) -> Self {
        Self {
            auth_repository,
            secret_key,
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
            &EncodingKey::from_secret(self.secret_key.as_bytes()),
        )?;

        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: exp - now.timestamp(),
        })
    }
}
