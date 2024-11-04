use std::path::PathBuf;
use image::{DynamicImage, imageops};
use uuid::Uuid;
use webp::Encoder;
use crate::domain::entities::avatar::{Avatar, AvatarUploadResponse};
use crate::domain::repositories::avatar_repository::AvatarRepository;
use crate::domain::repositories::account_repository::AccountRepository;

pub struct UploadAvatarUseCase<T: AvatarRepository, U: AccountRepository> {
    avatar_repository: T,
    account_repository: U,
    upload_dir: PathBuf,
}


impl<T: AvatarRepository, U: AccountRepository> UploadAvatarUseCase<T, U> {
    pub fn new(avatar_repository: T, account_repository: U, upload_dir: PathBuf) -> Self {
        Self {
            avatar_repository,
            account_repository,
            upload_dir,
        }
    }

    async fn convert_to_webp(img: &DynamicImage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let rgba = img.to_rgba8();
        let width = rgba.width() as u32;
        let height = rgba.height() as u32;

        let encoder = Encoder::from_rgba(&rgba, width, height);
        let encoded = encoder.encode(80.0);
        Ok(encoded.to_vec())
    }

    pub async fn execute(&self, account_id: i32, image_data: Vec<u8>) -> Result<AvatarUploadResponse, Box<dyn std::error::Error>> {
        tokio::fs::create_dir_all(&self.upload_dir).await?;

        let large_webp = format!("300_{}.webp", Uuid::new_v4());
        let small_webp = format!("40_{}.webp", Uuid::new_v4());

        let large_webp_path = self.upload_dir.join(&large_webp);
        let small_webp_path = self.upload_dir.join(&small_webp);

        let img = image::load_from_memory(&image_data)?;

        // Create 300x300 version
        let large = imageops::resize(&img, 300, 300, imageops::FilterType::Lanczos3);
        let large_img = DynamicImage::ImageRgba8(large);
        let large_webp_data = Self::convert_to_webp(&large_img).await?;
        tokio::fs::write(&large_webp_path, large_webp_data).await?;

        // Create 40x40 version
        let small = imageops::resize(&img, 40, 40, imageops::FilterType::Lanczos3);
        let small_img = DynamicImage::ImageRgba8(small);
        let small_webp_data = Self::convert_to_webp(&small_img).await?;
        tokio::fs::write(&small_webp_path, small_webp_data).await?;

        let large_url = format!("/uploads/{}", large_webp);
        let small_url = format!("/uploads/{}", small_webp);

        // Create new avatar record
        let avatar = self.avatar_repository.create(
            account_id,
            large_url.clone(),
            small_url.clone(),
        ).await?;

        // Update account's default avatar
        self.account_repository.set_default_avatar(account_id, avatar.id).await?;

        Ok(AvatarUploadResponse {
            avatar_300x300_url: large_url,
            avatar_40x40_url: small_url,
            message: "Avatar uploaded successfully".to_string(),
        })
    }
}