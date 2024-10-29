use actix_web::{web, HttpResponse, Responder};

use crate::application::use_cases::user_use_cases::GetUserByNameUseCase;
use crate::domain::repositories::user_repository::UserRepository;

pub struct UserHandlers<T: UserRepository> {
    get_user_use_case: GetUserByNameUseCase<T>,
}

impl<T: UserRepository> UserHandlers<T> {
    pub fn new(get_user_use_case: GetUserByNameUseCase<T>) -> Self {
        Self { get_user_use_case }
    }

    pub async fn get_user(&self) -> impl Responder {
        match self.get_user_use_case.execute("jose").await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().finish(),
        }
    }
}

pub fn configure<T: UserRepository + 'static>(
    cfg: &mut web::ServiceConfig,
    _handlers: web::Data<UserHandlers<T>>,
) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(move |handlers: web::Data<UserHandlers<T>>| async move {
                handlers.get_user().await
            })),
    );
}