use std::path::PathBuf;
use uuid::Uuid;
use image::{imageops, DynamicImage};
use tokio::fs;
use tracing::info;
use webp::Encoder;
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

    async fn convert_to_webp(img: &DynamicImage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let rgba = img.to_rgba8();
        let width = rgba.width() as u32;
        let height = rgba.height() as u32;

        let encoder = Encoder::from_rgba(&rgba, width, height);
        // Use encode_simple with quality parameter directly
        let encoded = encoder.encode(80.0); // quality parameter between 0-100
        Ok(encoded.to_vec())
    }

    pub async fn execute(&self, user_id: i32, image_data: Vec<u8>) -> Result<AvatarUploadResponse, Box<dyn std::error::Error>> {
        info!("Processing avatar upload for user {}", user_id);

        // Create upload directory if it doesn't exist
        fs::create_dir_all(&self.upload_dir).await?;
        info!("Upload directory created/verified");

        // Generate filenames with .webp extension
        let large_webp = format!("300_{}.webp", Uuid::new_v4());
        let small_webp = format!("40_{}.webp", Uuid::new_v4());

        let large_webp_path = self.upload_dir.join(&large_webp);
        let small_webp_path = self.upload_dir.join(&small_webp);

        // Process images
        let img = image::load_from_memory(&image_data)?;
        info!("Image loaded from memory");

        // Create and convert 300x300 version
        let large = imageops::resize(&img, 300, 300, imageops::FilterType::Lanczos3);
        let large_img = DynamicImage::ImageRgba8(large);
        let large_webp_data = Self::convert_to_webp(&large_img).await?;
        tokio::fs::write(&large_webp_path, large_webp_data).await?;
        info!("Large image saved: {:?}", large_webp_path);

        // Create and convert 40x40 version
        let small = imageops::resize(&img, 40, 40, imageops::FilterType::Lanczos3);
        let small_img = DynamicImage::ImageRgba8(small);
        let small_webp_data = Self::convert_to_webp(&small_img).await?;
        tokio::fs::write(&small_webp_path, small_webp_data).await?;
        info!("Small image saved: {:?}", small_webp_path);

        // Generate URLs
        let large_url = format!("/uploads/{}", large_webp);
        let small_url = format!("/uploads/{}", small_webp);

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