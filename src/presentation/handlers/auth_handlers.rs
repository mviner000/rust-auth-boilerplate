use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::application::use_cases::auth_use_cases::LoginUseCase;
use crate::domain::repositories::auth_repository::AuthRepository;
use crate::domain::entities::auth::AuthUser;
use tracing::debug;

pub struct AuthHandlers<T: AuthRepository> {
    login_use_case: LoginUseCase<T>,
}

#[allow(dead_code)]
impl<T: AuthRepository> AuthHandlers<T> {
    pub fn new(login_use_case: LoginUseCase<T>) -> Self {
        Self { login_use_case }
    }

    pub async fn login(&self, auth: web::Json<AuthUser>) -> impl Responder {
        let username = auth.username.clone();
        debug!("Login attempt for user: {}", username);

        match self.login_use_case.execute(auth.into_inner()).await {
            Ok(token) => {
                debug!("Login successful for user: {}", username);
                HttpResponse::Ok().json(token)
            }
            Err(e) => {
                debug!("Login failed: {}", e);
                HttpResponse::Unauthorized().json(json!({
                    "error": "Authentication failed",
                    "message": e.to_string()
                }))
            }
        }
    }
}

#[allow(dead_code)]
pub fn configure<T: AuthRepository + 'static>(
    cfg: &mut web::ServiceConfig,
    _handlers: web::Data<AuthHandlers<T>>,  // Removed underscore
) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(
                |handlers: web::Data<AuthHandlers<T>>, auth: web::Json<AuthUser>| async move {
                    handlers.login(auth).await
                }
            ))
    );
}