mod application;
mod domain;
mod infrastructure;
mod presentation;
mod schema;

use actix_files::Files;
use tracing::info;
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use tracing::{Level};
use tracing_subscriber::FmtSubscriber;
use std::path::PathBuf;
use std::sync::Arc;
use infrastructure::{
    config::database,
    repositories::{
        user_repository::UserRepositoryImpl,
        auth_repository::AuthRepositoryImpl,
        account_repository::AccountRepositoryImpl,
        avatar_repository::AvatarRepositoryImpl,
    },
};

use crate::application::use_cases::{
    account_use_cases::{GetAccountUseCase, UpdateAccountUseCase},
    avatar_use_cases::UploadAvatarUseCase,
    user_use_cases::{GetUserByIdUseCase, CreateUserUseCase, ListUsersUseCase, DeleteUserUseCase, UpdateUserUseCase},
    auth_use_cases::{LoginUseCase, RegisterUseCase},
};

use presentation::{
    handlers::{
        user_handlers::{UserHandlers, configure as user_configure},
        auth_handlers::{AuthHandlers, configure as auth_configure},
        account_handlers::{AccountHandlers, configure as account_configure},
        avatar_handlers::{AvatarHandlers, configure as avatar_configure},
    },
    middleware::auth::validator,
};
use presentation::handlers::ws_handlers;
use crate::application::use_cases::message_use_cases::{GetMessagesUseCase, SendMessageUseCase};
use crate::infrastructure::repositories::message_repository::MessageRepositoryImpl;
use crate::infrastructure::websocket::realtime_message_manager::RealtimeMessageManager;
use crate::infrastructure::websocket::user_status_manager::UserStatusManager;
use crate::presentation::handlers::message_handlers;
use crate::presentation::handlers::message_handlers::MessageHandlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Starting application...");

    dotenvy::dotenv().ok();
    let pool = database::establish_connection();

    info!("Database connection established");

    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    // Initialize WebSocket managers
    let user_status_manager = Arc::new(UserStatusManager::new());
    let realtime_message_manager = RealtimeMessageManager::new(user_status_manager.clone());

    // Create uploads directory if it doesn't exist
    let upload_dir = PathBuf::from("uploads");
    std::fs::create_dir_all(&upload_dir)?;
    info!("Upload directory ensured: {:?}", upload_dir);

    // Initialize repositories with properly cloned pools
    let user_repository = UserRepositoryImpl::new(pool.clone());
    let auth_repository = AuthRepositoryImpl::new(pool.clone(), secret_key);
    let account_repository = AccountRepositoryImpl::new(pool.clone());
    let avatar_repository = AvatarRepositoryImpl::new(pool.clone());
    let message_repository = MessageRepositoryImpl::new(pool.clone());

    // Initialize use cases
    let get_user_use_case = GetUserByIdUseCase::new(user_repository.clone());
    let create_user_use_case = CreateUserUseCase::new(user_repository.clone());
    let list_users_use_case = ListUsersUseCase::new(user_repository.clone());
    let update_user_use_case = UpdateUserUseCase::new(user_repository.clone());
    let delete_user_use_case = DeleteUserUseCase::new(user_repository);

    let send_message_use_case = SendMessageUseCase::new(message_repository.clone());
    let get_messages_use_case = GetMessagesUseCase::new(message_repository);

    let login_use_case = LoginUseCase::new(auth_repository.clone());
    let register_use_case = RegisterUseCase::new(auth_repository);

    let get_account_use_case = GetAccountUseCase::new(account_repository.clone());
    let update_account_use_case = UpdateAccountUseCase::new(account_repository.clone());
    let upload_avatar_use_case = UploadAvatarUseCase::new(
        avatar_repository.clone(),
        account_repository.clone(),
        upload_dir,
    );

    // Initialize handlers
    let user_handlers = web::Data::new(UserHandlers::new(
        get_user_use_case,
        create_user_use_case,
        list_users_use_case,
        update_user_use_case,
        delete_user_use_case,
    ));

    let auth_handlers = web::Data::new(AuthHandlers::new(
        login_use_case,
        register_use_case,
    ));

    let account_handlers = web::Data::new(AccountHandlers::new(
        get_account_use_case,
        update_account_use_case,
    ));

    let avatar_handlers = web::Data::new(AvatarHandlers::new(
        upload_avatar_use_case,
    ));

    let message_handlers = web::Data::new(MessageHandlers::new(
        send_message_use_case,
        get_messages_use_case,
        realtime_message_manager.clone(),
    ));

    let auth = HttpAuthentication::bearer(validator);

    let user_status_manager_data = web::Data::new(user_status_manager);
    let realtime_message_manager_data = web::Data::new(realtime_message_manager);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                matches!(
                    origin.as_bytes(),
                    b"http://localhost:3000" |
                    b"http://192.168.100.7:3000" |
                    b"http://0.0.0.0:3000"
                )
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(user_handlers.clone())
            .app_data(auth_handlers.clone())
            .app_data(account_handlers.clone())
            .app_data(avatar_handlers.clone())
            .app_data(user_status_manager_data.clone())
            .app_data(realtime_message_manager_data.clone())
            .configure(ws_handlers::configure)
            .service(Files::new("/uploads", "uploads").show_files_listing())
            .service(
                web::scope("/api/v1")
                    .configure(|cfg| auth_configure(cfg, auth_handlers.clone()))
                    .service(
                        web::scope("")
                            .wrap(auth.clone())
                            .configure(|cfg| user_configure(cfg, user_handlers.clone()))
                            .configure(|cfg| account_configure(cfg, account_handlers.clone()))
                            .configure(|cfg| avatar_configure(cfg, avatar_handlers.clone()))
                            .configure(|cfg| message_handlers::configure(cfg, message_handlers.clone()))
                    )
            )
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}