mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

use infrastructure::{
    config::database,
    repositories::{
        user_repository::UserRepositoryImpl,
        auth_repository::AuthRepositoryImpl,
    },
};

use application::use_cases::{
    user_use_cases::{GetUserByIdUseCase, CreateUserUseCase, ListUsersUseCase, DeleteUserUseCase, UpdateUserUseCase},
    auth_use_cases::LoginUseCase,
};

use presentation::{
    handlers::{
        user_handlers::{UserHandlers, configure as user_configure},
        auth_handlers::{AuthHandlers, configure as auth_configure},
    },
    middleware::auth::validator,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .init();

    debug!("Starting application...");

    dotenvy::dotenv().ok();
    let pool = database::establish_connection();

    debug!("Database connection established");

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let user_repository = UserRepositoryImpl::new(pool.clone());
    let auth_repository = AuthRepositoryImpl::new(pool);

    let get_user_use_case = GetUserByIdUseCase::new(user_repository.clone());
    let create_user_use_case = CreateUserUseCase::new(user_repository.clone());
    let list_users_use_case = ListUsersUseCase::new(user_repository.clone());
    let update_user_use_case = UpdateUserUseCase::new(user_repository.clone());
    let delete_user_use_case = DeleteUserUseCase::new(user_repository);

    let login_use_case = LoginUseCase::new(auth_repository, jwt_secret);

    let user_handlers = web::Data::new(UserHandlers::new(
        get_user_use_case,
        create_user_use_case,
        list_users_use_case,
        update_user_use_case,
        delete_user_use_case,
    ));

    let auth_handlers = web::Data::new(AuthHandlers::new(login_use_case));

    let auth = HttpAuthentication::bearer(validator);

    HttpServer::new(move || {
        App::new()
            .app_data(user_handlers.clone())
            .app_data(auth_handlers.clone())
            .service(
                web::scope("/api/v1")
                    // Configure auth routes WITHOUT authentication
                    .configure(|cfg| auth_configure(cfg, auth_handlers.clone()))
                    // Protected routes under /user with authentication
                    .service(
                        web::scope("")
                            .wrap(auth.clone())
                            .configure(|cfg| user_configure(cfg, user_handlers.clone()))
                    )
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}