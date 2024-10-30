use std::path::PathBuf;
use uuid::Uuid;
use image::{ImageFormat, imageops};
use tokio::fs;
use tracing::info;
use crate::domain::repositories::account_repository::AccountRepository;
use crate::domain::entities::account::{Account, UpdateAccountDto, AvatarUploadResponse};

pub struct GetAccountUseCase<T: AccountRepository> {
    account_repository: T,
}

impl<T: AccountRepository> GetAccountUseCase<T> {
    pub fn new(account_repository: T) -> Self {
        Self { account_repository }
    }

    pub async fn execute(&self, user_id: i32) -> Result<Account, Box<dyn std::error::Error>> {
        self.account_repository.find_by_user_id(user_id).await
    }
}

pub struct UpdateAccountUseCase<T: AccountRepository> {
    account_repository: T,
}

impl<T: AccountRepository> UpdateAccountUseCase<T> {
    pub fn new(account_repository: T) -> Self {
        Self { account_repository }
    }

    pub async fn execute(&self, user_id: i32, account_dto: UpdateAccountDto) -> Result<Account, Box<dyn std::error::Error>> {
        self.account_repository.update(user_id, account_dto).await
    }
}

pub struct UploadAvatarUseCase<T: AccountRepository> {
    account_repository: T,
    upload_dir: PathBuf,
}

impl<T: AccountRepository> UploadAvatarUseCase<T> {
    pub fn new(account_repository: T, upload_dir: PathBuf) -> Self {
        Self {
            account_repository,
            upload_dir,
        }
    }

    pub async fn execute(&self, user_id: i32, image_data: Vec<u8>) -> Result<AvatarUploadResponse, Box<dyn std::error::Error>> {
        info!("Processing avatar upload for user {}", user_id);

        // Create upload directory if it doesn't exist
        fs::create_dir_all(&self.upload_dir).await?;
        info!("Upload directory created/verified");

        // Generate unique filename
        let filename = format!("{}.avif", Uuid::new_v4());
        info!("Generated filename: {}", filename);

        // Process images
        let img = image::load_from_memory(&image_data)?;
        info!("Image loaded from memory");

        // Create 300x300 version
        let large = imageops::resize(&img, 300, 300, imageops::FilterType::Lanczos3);
        let large_path = self.upload_dir.join(format!("300_{}", &filename));
        large.save_with_format(&large_path, ImageFormat::Avif)?;
        info!("Large image saved: {:?}", large_path);

        // Create 40x40 version
        let small = imageops::resize(&img, 40, 40, imageops::FilterType::Lanczos3);
        let small_path = self.upload_dir.join(format!("40_{}", &filename));
        small.save_with_format(&small_path, ImageFormat::Avif)?;
        info!("Small image saved: {:?}", small_path);

        // Generate URLs
        let large_url = format!("/uploads/300_{}", filename);
        let small_url = format!("/uploads/40_{}", filename);

        // Update database
        info!("Updating database with new avatar URLs");
        self.account_repository.update_avatar(user_id, large_url.clone(), small_url.clone()).await?;
        info!("Database updated successfully");

        Ok(AvatarUploadResponse {
            avatar_300x300_url: large_url,
            avatar_40x40_url: small_url,
            message: "Avatar uploaded successfully".to_string(),
        })
    }
}