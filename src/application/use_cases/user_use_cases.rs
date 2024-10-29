use crate::domain::{
    entities::user::{User, CreateUserDto},
    repositories::user_repository::UserRepository,
};

pub struct GetUserByNameUseCase<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> GetUserByNameUseCase<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, name: &str) -> Result<User, Box<dyn std::error::Error>> {
        self.user_repository.find_by_name(name).await
    }
}

pub struct CreateUserUseCase<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> CreateUserUseCase<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, user_dto: CreateUserDto) -> Result<User, Box<dyn std::error::Error>> {
        self.user_repository.create(user_dto).await
    }
}
