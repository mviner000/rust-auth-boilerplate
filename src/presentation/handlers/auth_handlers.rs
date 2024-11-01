use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::application::use_cases::auth_use_cases::{LoginUseCase, RegisterUseCase};
use crate::domain::repositories::auth_repository::AuthRepository;
use crate::domain::entities::auth::{AuthUser, RegisterUserDto};
use tracing::debug;

pub struct AuthHandlers<T: AuthRepository> {
    login_use_case: LoginUseCase<T>,
    register_use_case: RegisterUseCase<T>,
}

#[allow(dead_code)]
impl<T: AuthRepository> AuthHandlers<T> {
    pub fn new(login_use_case: LoginUseCase<T>, register_use_case: RegisterUseCase<T>) -> Self {
        Self {
            login_use_case,
            register_use_case,
        }
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

    pub async fn register(&self, register_dto: web::Json<RegisterUserDto>) -> impl Responder {
        match self.register_use_case.execute(register_dto.into_inner()).await {
            Ok(user) => {
                HttpResponse::Created().json(json!({
                    "status": "success",
                    "message": "User registered successfully",
                    "data": {
                        "id": user.id,
                        "username": user.username,
                        "email": user.email
                    }
                }))
            }
            Err(e) => {
                let error_message = e.to_string();
                HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Registration failed",
                    "error": error_message
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
            .route("/register", web::post().to(
                |handlers: web::Data<AuthHandlers<T>>, register_dto: web::Json<RegisterUserDto>| async move {
                    handlers.register(register_dto).await
                }
            ))
    );
}