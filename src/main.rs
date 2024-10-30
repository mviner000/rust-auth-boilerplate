mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::{web, App, HttpServer};
use infrastructure::{
    config::database,
    repositories::user_repository::UserRepositoryImpl,
};
use application::use_cases::user_use_cases::{GetUserByIdUseCase, CreateUserUseCase, ListUsersUseCase};
use presentation::handlers::user_handlers::{UserHandlers, configure};
use crate::application::use_cases::user_use_cases::{DeleteUserUseCase, UpdateUserUseCase};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::establish_connection();

    let user_repository = UserRepositoryImpl::new(pool);
    let get_user_use_case = GetUserByIdUseCase::new(user_repository.clone());
    let create_user_use_case = CreateUserUseCase::new(user_repository.clone());
    let list_users_use_case = ListUsersUseCase::new(user_repository.clone());
    let update_user_use_case = UpdateUserUseCase::new(user_repository.clone());
    let delete_user_use_case = DeleteUserUseCase
    ::new(user_repository);

    let user_handlers = web::Data::new(UserHandlers::new(
        get_user_use_case,
        create_user_use_case,
        list_users_use_case,
        update_user_use_case,
        delete_user_use_case,
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(user_handlers.clone())
            .configure(|cfg| configure(cfg, user_handlers.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}