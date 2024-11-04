use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use mime_guess::from_path;
use crate::application::use_cases::avatar_use_cases::UploadAvatarUseCase;
use crate::domain::repositories::avatar_repository::AvatarRepository;
use crate::domain::repositories::account_repository::AccountRepository;

pub struct AvatarHandlers<T: AvatarRepository, U: AccountRepository> {
    upload_avatar_use_case: UploadAvatarUseCase<T, U>,
}

impl<T: AvatarRepository, U: AccountRepository> AvatarHandlers<T, U> {
    pub fn new(upload_avatar_use_case: UploadAvatarUseCase<T, U>) -> Self {
        Self {
            upload_avatar_use_case,
        }
    }

    pub async fn upload_avatar(&self, account_id: web::Path<i32>, mut payload: Multipart) -> impl Responder {
        while let Ok(Some(mut field)) = payload.try_next().await {
            if field.name() == "avatar" {
                // Get content type from filename
                if let Some(filename) = field.content_disposition().get_filename() {
                    let content_type = from_path(filename).first_or_octet_stream();

                    // Validate content type
                    if !content_type.type_().eq(&mime::IMAGE) {
                        return HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Invalid file type",
                            "message": "Only image files are allowed"
                        }));
                    }

                    // Process valid image file
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

                    return match self.upload_avatar_use_case.execute(account_id.into_inner(), image_data).await {
                        Ok(response) => HttpResponse::Ok().json(response),
                        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Failed to process avatar",
                            "message": e.to_string()
                        })),
                    };
                }
            }
        }

        HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No avatar file provided",
            "message": "Please provide an avatar file"
        }))
    }
}

pub fn configure<T: AvatarRepository + 'static, U: AccountRepository + 'static>(
    cfg: &mut web::ServiceConfig,
    handlers: web::Data<AvatarHandlers<T, U>>,
) {
    cfg.service(
        web::scope("/avatars")
            .route("/{account_id}", web::post().to(move |handlers: web::Data<AvatarHandlers<T, U>>, account_id: web::Path<i32>, payload: Multipart| async move {
                handlers.upload_avatar(account_id, payload).await
            }))
    );
}
