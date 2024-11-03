use actix_web::{web, HttpResponse, Responder, dev::Payload, FromRequest, HttpMessage};
use serde_json::json;
use std::future::{ready, Ready};
use futures::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use crate::domain::entities::auth::Claims;
use crate::application::use_cases::user_use_cases::{CreateUserUseCase, ListUsersUseCase, GetUserByIdUseCase, UpdateUserUseCase, DeleteUserUseCase};
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::entities::user::{CreateUserDto, UpdateUserDto};

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        ready(match req.extensions().get::<Claims>() {
            Some(claims) => Ok((*claims).clone()),  // This now requires Clone implementation
            None => Err(actix_web::error::ErrorUnauthorized("No claims found")),
        })
    }
}

pub struct UserHandlers<T: UserRepository> {
    get_user_use_case: GetUserByIdUseCase<T>,
    create_user_use_case: CreateUserUseCase<T>,
    list_users_use_case: ListUsersUseCase<T>,
    update_user_use_case: UpdateUserUseCase<T>,
    delete_user_use_case: DeleteUserUseCase<T>,
}

impl<T: UserRepository> UserHandlers<T> {
    pub fn new(
        get_user_use_case: GetUserByIdUseCase<T>,
        create_user_use_case: CreateUserUseCase<T>,
        list_users_use_case: ListUsersUseCase<T>,
        update_user_use_case: UpdateUserUseCase<T>,
        delete_user_use_case: DeleteUserUseCase<T>,
    ) -> Self {
        Self {
            get_user_use_case,
            create_user_use_case,
            list_users_use_case,
            update_user_use_case,
            delete_user_use_case,
        }
    }

    pub async fn get_user(&self, user_id: web::Path<i32>) -> impl Responder {
        match self.get_user_use_case.execute(user_id.into_inner()).await {
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

    pub async fn list_users(&self) -> impl Responder {
        match self.list_users_use_case.execute().await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    pub async fn update_user(&self, user_id: web::Path<i32>, user_dto: web::Json<UpdateUserDto>) -> impl Responder {
        match self.update_user_use_case.execute(user_id.into_inner(), user_dto.into_inner()).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().finish(),
        }
    }

    pub async fn delete_user(&self, user_id: web::Path<i32>) -> impl Responder {
        match self.delete_user_use_case.execute(user_id.into_inner()).await {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(_) => HttpResponse::NotFound().finish(),
        }
    }

    pub async fn get_profile(&self, claims: Claims) -> impl Responder {
        match self.get_user_use_case.execute(claims.sub).await {
            Ok(user) => HttpResponse::Ok().json(json!({
                "id": user.id,
                "username": user.username,
                "email": user.email,
            })),
            Err(_) => HttpResponse::NotFound().finish(),
        }
    }
}

pub fn configure<T: UserRepository + 'static>(
    cfg: &mut web::ServiceConfig,
    _handlers: web::Data<UserHandlers<T>>,
) {
    cfg.service(
        web::scope("/user")
            // First, define the static routes
            .route("/me", web::get().to(move |handlers: web::Data<UserHandlers<T>>, claims: Claims| async move {
                handlers.get_profile(claims).await
            }))
            .route("", web::get().to(move |handlers: web::Data<UserHandlers<T>>| async move {
                handlers.list_users().await
            }))
            .route("", web::post().to(move |handlers: web::Data<UserHandlers<T>>, user_dto: web::Json<CreateUserDto>| async move {
                handlers.create_user(user_dto).await
            }))
            // Then, define the routes with parameters
            .route("/{id}", web::get().to(move |handlers: web::Data<UserHandlers<T>>, id: web::Path<i32>| async move {
                handlers.get_user(id).await
            }))
            .route("/{id}", web::put().to(move |handlers: web::Data<UserHandlers<T>>, id: web::Path<i32>, user_dto: web::Json<UpdateUserDto>| async move {
                handlers.update_user(id, user_dto).await
            }))
            .route("/{id}", web::delete().to(move |handlers: web::Data<UserHandlers<T>>, id: web::Path<i32>| async move {
                handlers.delete_user(id).await
            })),
    );
}