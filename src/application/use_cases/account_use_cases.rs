use std::sync::Arc;
use crate::domain::repositories::account_repository::AccountRepository;
use crate::domain::entities::account::{Account, UpdateAccountDto};

pub struct GetAccountUseCase<T: AccountRepository> {
    account_repository: Arc<T>,
}

impl<T: AccountRepository> GetAccountUseCase<T> {
    pub fn new(account_repository: Arc<T>) -> Self {
        Self { account_repository }
    }

    pub async fn execute(&self, user_id: i32) -> Result<Account, Box<dyn std::error::Error>> {
        self.account_repository.find_by_user_id(user_id).await
    }
}

pub struct GetAllAccountsUseCase<T: AccountRepository> {
    repository: Arc<T>,
}

impl<T: AccountRepository> GetAllAccountsUseCase<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Account>, Box<dyn std::error::Error>> {
        self.repository.get_all().await
    }
}

pub struct UpdateAccountUseCase<T: AccountRepository> {
    account_repository: Arc<T>,
}

impl<T: AccountRepository> UpdateAccountUseCase<T> {
    pub fn new(account_repository: Arc<T>) -> Self {
        Self { account_repository }
    }

    pub async fn execute(&self, user_id: i32, account_dto: UpdateAccountDto) -> Result<Account, Box<dyn std::error::Error>> {
        self.account_repository.update(user_id, account_dto).await
    }
}