use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use tracing::debug;  // Add this import
use bcrypt::verify;  // Remove DEFAULT_COST

use crate::domain::entities::auth::AuthUser;
use crate::domain::entities::user::User;
use crate::domain::repositories::auth_repository::AuthRepository;
use crate::infrastructure::repositories::user_repository::users;

pub struct AuthRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AuthRepositoryImpl {

    #[allow(dead_code)]
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn authenticate(&self, auth: AuthUser) -> Result<User, Box<dyn std::error::Error>> {
        use self::users::dsl::*;

        debug!("Attempting to authenticate user: {}", auth.username);

        let conn = &mut self.pool.get()?;

        // First find the user by username only
        let user_result = users
            .filter(username.eq(&auth.username))
            .first::<(i32, String, String, String)>(conn);

        match user_result {
            Ok(user) => {
                // Verify the password using bcrypt
                if verify(&auth.password, &user.3)? {
                    debug!("Password verified successfully");
                    Ok(User {
                        id: user.0,
                        username: user.1,
                        email: user.2,
                        password: user.3,
                    })
                } else {
                    debug!("Password verification failed");
                    Err("Invalid password".into())
                }
            }
            Err(e) => {
                debug!("User not found: {}", e);
                Err(Box::new(e))
            }
        }
    }
}