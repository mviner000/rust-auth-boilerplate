mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::{web, App, HttpServer};
use infrastructure::{
    config::database,
    repositories::user_repository::UserRepositoryImpl,
};
use application::use_cases::user_use_cases::GetUserByNameUseCase;
use presentation::handlers::user_handlers::{UserHandlers, configure};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::establish_connection();

    let user_repository = UserRepositoryImpl::new(pool);
    let get_user_use_case = GetUserByNameUseCase::new(user_repository);
    let user_handlers = web::Data::new(UserHandlers::new(get_user_use_case));

    HttpServer::new(move || {
        App::new()
            .app_data(user_handlers.clone())
            .configure(|cfg| configure(cfg, user_handlers.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}