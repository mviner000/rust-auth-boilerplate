use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use tracing::debug;
use bcrypt::{hash_with_salt, DEFAULT_COST};

use crate::domain::entities::auth::{AuthUser, RegisterUserDto};
use crate::domain::entities::user::User;
use crate::domain::repositories::auth_repository::AuthRepository;
use crate::schema::{users, accounts};

#[derive(Clone)]
pub struct AuthRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
    salt: [u8; 16],
}

impl AuthRepositoryImpl {
    #[allow(dead_code)]
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>, secret_key: String) -> Self {
        let salt = secret_key.as_bytes()[..16]
            .try_into()
            .expect("SECRET_KEY must be at least 16 bytes");

        debug!("Initializing AuthRepositoryImpl with salt: {:?}", salt);
        Self { pool, salt }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn authenticate(&self, auth: AuthUser) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        use self::users::dsl::*;

        debug!("Starting authentication for user: {}", auth.username);

        let conn = &mut self.pool.get().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Get user with all fields
        let user_result = users
            .filter(username.eq(&auth.username))
            .select((id, username, email, password))
            .first::<(i32, String, String, String)>(conn)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        debug!("Found stored hash for user {}: {}", auth.username, user_result.3);

        // Hash the provided password with the same salt for comparison
        let hashed_input = hash_with_salt(
            auth.password.as_bytes(),
            DEFAULT_COST,
            self.salt
        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
            .to_string();

        debug!("Generated hash for login attempt: {}", hashed_input);
        debug!("Hash comparison for {}", auth.username);
        debug!("  Stored hash  : {}", user_result.3);
        debug!("  Login hash   : {}", hashed_input);
        debug!("  Match result : {}", user_result.3 == hashed_input);

        if user_result.3 == hashed_input {
            debug!("Password verification successful for user: {}", auth.username);
            Ok(User {
                id: user_result.0,
                username: user_result.1,
                email: user_result.2,
                password: user_result.3,
            })
        } else {
            debug!("Password verification failed for user: {}", auth.username);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid password"
            )))
        }
    }

    async fn register(&self, register_dto: RegisterUserDto) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        use self::users::dsl::*;
        debug!("Starting registration for user: {}", register_dto.username);

        let conn = &mut self.pool.get().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Check if username already exists
        let existing_user = users
            .filter(username.eq(&register_dto.username))
            .select((id, username, email, password))
            .first::<(i32, String, String, String)>(conn)
            .optional()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        if existing_user.is_some() {
            debug!("Registration failed: Username already exists: {}", register_dto.username);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Username already exists"
            )));
        }

        // Hash the password with our consistent salt
        let hashed_password = hash_with_salt(
            register_dto.password.as_bytes(),
            DEFAULT_COST,
            self.salt
        ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
            .to_string();

        debug!("Generated hash for registration: {}", hashed_password);

        // Start transaction
        conn.transaction(|conn| {
            // Create user with all fields selected
            let user_result = diesel::insert_into(users)
                .values((
                    username.eq(&register_dto.username),
                    email.eq(&register_dto.email),
                    password.eq(&hashed_password),
                ))
                .returning((id, username, email, password))
                .get_result::<(i32, String, String, String)>(conn);

            match user_result {
                Ok(user) => {
                    // Create associated account
                    diesel::insert_into(accounts::table)
                        .values((
                            accounts::user_id.eq(user.0),
                            accounts::first_name.eq(&register_dto.first_name),
                            accounts::middle_name.eq(&register_dto.middle_name),
                            accounts::last_name.eq(&register_dto.last_name),
                            accounts::created_at.eq(diesel::dsl::now),
                            accounts::updated_at.eq(diesel::dsl::now),
                        ))
                        .execute(conn)?;

                    debug!("User and account created successfully for: {}", register_dto.username);
                    debug!("Stored hash in database: {}", user.3);

                    Ok(User {
                        id: user.0,
                        username: user.1,
                        email: user.2,
                        password: user.3,
                    })
                }
                Err(e) => Err(e),
            }
        })
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}