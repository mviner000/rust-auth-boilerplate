use actix_web::{web, HttpResponse, Responder};
use crate::domain::repositories::account_repository::AccountRepository;
use crate::application::use_cases::account_use_cases::{GetAccountUseCase, UpdateAccountUseCase};
use crate::domain::entities::account::UpdateAccountDto;

pub struct AccountHandlers<T: AccountRepository> {
    get_account_use_case: GetAccountUseCase<T>,
    update_account_use_case: UpdateAccountUseCase<T>,
}

impl<T: AccountRepository> AccountHandlers<T> {
    pub fn new(
        get_account_use_case: GetAccountUseCase<T>,
        update_account_use_case: UpdateAccountUseCase<T>,
    ) -> Self {
        Self {
            get_account_use_case,
            update_account_use_case,
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
    );
}