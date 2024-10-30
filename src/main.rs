mod application;
mod domain;
mod infrastructure;
mod presentation;
mod schema;

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;
use std::path::PathBuf;

use infrastructure::{
    config::database,
    repositories::{
        user_repository::UserRepositoryImpl,
        auth_repository::AuthRepositoryImpl,
        account_repository::AccountRepositoryImpl, // Add this
    },
};

use application::use_cases::{
    account_use_cases::{UpdateAccountUseCase, UploadAvatarUseCase},
    user_use_cases::{GetUserByIdUseCase, CreateUserUseCase, ListUsersUseCase, DeleteUserUseCase, UpdateUserUseCase},
    auth_use_cases::LoginUseCase,
};

use presentation::{
    handlers::{
        user_handlers::{UserHandlers, configure as user_configure},
        auth_handlers::{AuthHandlers, configure as auth_configure},
        account_handlers::{AccountHandlers, configure as account_configure}, // Add this
    },
    middleware::auth::validator,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .init();

    debug!("Starting application...");

    dotenvy::dotenv().ok();
    let pool = database::establish_connection();

    debug!("Database connection established");

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Initialize repositories
    let user_repository = UserRepositoryImpl::new(pool.clone());
    let auth_repository = AuthRepositoryImpl::new(pool.clone());
    let account_repository = AccountRepositoryImpl::new(pool.clone());

    // Initialize use cases
    let get_user_use_case = GetUserByIdUseCase::new(user_repository.clone());
    let create_user_use_case = CreateUserUseCase::new(user_repository.clone());
    let list_users_use_case = ListUsersUseCase::new(user_repository.clone());
    let update_user_use_case = UpdateUserUseCase::new(user_repository.clone());
    let delete_user_use_case = DeleteUserUseCase::new(user_repository);

    let login_use_case = LoginUseCase::new(auth_repository, jwt_secret);

    let upload_dir = PathBuf::from("uploads");
    let update_account_use_case = UpdateAccountUseCase::new(account_repository.clone());
    let upload_avatar_use_case = UploadAvatarUseCase::new(account_repository, upload_dir);

    // Initialize handlers
    let user_handlers = web::Data::new(UserHandlers::new(
        get_user_use_case,
        create_user_use_case,
        list_users_use_case,
        update_user_use_case,
        delete_user_use_case,
    ));

    let auth_handlers = web::Data::new(AuthHandlers::new(login_use_case));

    let account_handlers = web::Data::new(AccountHandlers::new(
        update_account_use_case,
        upload_avatar_use_case,
    ));

    let auth = HttpAuthentication::bearer(validator);

    HttpServer::new(move || {
        App::new()
            .app_data(user_handlers.clone())
            .app_data(auth_handlers.clone())
            .app_data(account_handlers.clone())
            .service(
                web::scope("/api/v1")
                    .configure(|cfg| auth_configure(cfg, auth_handlers.clone()))
                    .service(
                        web::scope("")
                            .wrap(auth.clone())
                            .configure(|cfg| user_configure(cfg, user_handlers.clone()))
                            .configure(|cfg| account_configure(cfg, account_handlers.clone()))
                    )
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}