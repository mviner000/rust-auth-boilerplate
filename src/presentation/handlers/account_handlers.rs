use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use tracing::info;
use crate::domain::repositories::account_repository::AccountRepository;
use crate::application::use_cases::account_use_cases::{GetAccountUseCase, UpdateAccountUseCase, UploadAvatarUseCase};
use crate::domain::entities::account::UpdateAccountDto;

pub struct AccountHandlers<T: AccountRepository> {
    get_account_use_case: GetAccountUseCase<T>,
    update_account_use_case: UpdateAccountUseCase<T>,
    upload_avatar_use_case: UploadAvatarUseCase<T>,
}

impl<T: AccountRepository> AccountHandlers<T> {
    pub fn new(
        get_account_use_case: GetAccountUseCase<T>,
        update_account_use_case: UpdateAccountUseCase<T>,
        upload_avatar_use_case: UploadAvatarUseCase<T>,
    ) -> Self {
        Self {
            get_account_use_case,
            update_account_use_case,
            upload_avatar_use_case,
        }
    }

    pub async fn get_account(&self, user_id: web::Path<i32>) -> impl Responder {
        match self.get_account_use_case.execute(user_id.into_inner()).await {
            Ok(account) => HttpResponse::Ok().json(account),
            Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get account",
                "message": e.to_string()
            })),
        }
    }

    pub async fn update_account(&self, user_id: web::Path<i32>, account_dto: web::Json<UpdateAccountDto>) -> impl Responder {
        match self.update_account_use_case.execute(user_id.into_inner(), account_dto.into_inner()).await {
            Ok(account) => HttpResponse::Ok().json(account),
            Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update account",
                "message": e.to_string()
            })),
        }
    }

    pub async fn upload_avatar(&self, user_id: web::Path<i32>, mut payload: Multipart) -> impl Responder {
        // Process multipart form data
        while let Ok(Some(mut field)) = payload.try_next().await {
            if field.name() == "avatar" {
                let mut image_data = Vec::new();
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => image_data.extend_from_slice(&data),
                        Err(e) => {
                            return HttpResponse::BadRequest().json(serde_json::json!({
                                "error": "Failed to process upload",
                                "message": e.to_string()
                            }));
                        }
                    }
                }

                return match self.upload_avatar_use_case.execute(user_id.into_inner(), image_data).await {
                    Ok(response) => HttpResponse::Ok().json(response),
                    Err(e) => {
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Failed to process avatar",
                            "message": e.to_string()
                        }))
                    }
                }
            }
        }

        HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No avatar file provided",
            "message": "Please provide an avatar file"
        }))
    }
}

pub fn configure<T: AccountRepository + 'static>(
    cfg: &mut web::ServiceConfig,
    handlers: web::Data<AccountHandlers<T>>,
) {
    cfg.service(
        web::scope("/account")
            .route("/{id}", web::get().to(move |handlers: web::Data<AccountHandlers<T>>, id: web::Path<i32>| async move {
                handlers.get_account(id).await
            }))
            .route("/{id}", web::put().to(move |handlers: web::Data<AccountHandlers<T>>, id: web::Path<i32>, account_dto: web::Json<UpdateAccountDto>| async move {
                handlers.update_account(id, account_dto).await
            }))
            .route("/{id}/avatar", web::post().to(move |handlers: web::Data<AccountHandlers<T>>, id: web::Path<i32>, payload: Multipart| async move {
                info!("Received avatar upload request for user {}", id);
                handlers.upload_avatar(id, payload).await
            }))
    );
}