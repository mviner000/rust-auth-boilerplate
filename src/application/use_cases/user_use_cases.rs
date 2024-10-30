use crate::domain::{
    entities::user::{User, CreateUserDto},
    repositories::user_repository::UserRepository,
};
use crate::domain::entities::user::UpdateUserDto;

pub struct GetUserByIdUseCase<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> GetUserByIdUseCase<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, user_id: i32) -> Result<User, Box<dyn std::error::Error>> {
        self.user_repository.find_by_id(user_id).await
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


pub struct ListUsersUseCase<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> ListUsersUseCase<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        self.user_repository.find_all().await
    }
}

pub struct UpdateUserUseCase<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> UpdateUserUseCase<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, id: i32, user_dto: UpdateUserDto) -> Result<User, Box<dyn std::error::Error>> {
        self.user_repository.update(id, user_dto).await
    }
}


pub struct DeleteUserUseCase<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> DeleteUserUseCase<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.user_repository.delete(user_id).await
    }
}