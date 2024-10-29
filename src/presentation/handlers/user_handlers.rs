use actix_web::{web, HttpResponse, Responder};

use crate::application::use_cases::user_use_cases::{GetUserByNameUseCase, CreateUserUseCase};
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::entities::user::CreateUserDto;

pub struct UserHandlers<T: UserRepository> {
    get_user_use_case: GetUserByNameUseCase<T>,
    create_user_use_case: CreateUserUseCase<T>,
}

impl<T: UserRepository> UserHandlers<T> {
    pub fn new(
        get_user_use_case: GetUserByNameUseCase<T>,
        create_user_use_case: CreateUserUseCase<T>,
    ) -> Self {
        Self {
            get_user_use_case,
            create_user_use_case,
        }
    }

    pub async fn get_user(&self) -> impl Responder {
        match self.get_user_use_case.execute("jose").await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().finish(),
        }
    }

    pub async fn create_user(&self, user_dto: web::Json<CreateUserDto>) -> impl Responder {
        match self.create_user_use_case.execute(user_dto.into_inner()).await {
            Ok(user) => HttpResponse::Created().json(user),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

pub fn configure<T: UserRepository + 'static>(
    cfg: &mut web::ServiceConfig,
    handlers: web::Data<UserHandlers<T>>,
) {
    cfg.service(
        web::scope("/user")
            .route("", web::get().to(move |handlers: web::Data<UserHandlers<T>>| async move {
                handlers.get_user().await
            }))
            .route("", web::post().to(move |handlers: web::Data<UserHandlers<T>>, user_dto: web::Json<CreateUserDto>| async move {
                handlers.create_user(user_dto).await
            })),
    );
}
