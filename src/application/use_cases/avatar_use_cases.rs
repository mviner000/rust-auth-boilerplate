use std::path::{Path, PathBuf};
use uuid::Uuid;
use image::{ImageOutputFormat};
use webp::Encoder;
use crate::domain::entities::avatar::{AvatarUploadResponse, AvatarUrls};
use crate::domain::repositories::avatar_repository::AvatarRepository;
use crate::domain::repositories::account_repository::AccountRepository;
use std::sync::Arc;

const LARGE_SIZE: u32 = 300;
const SMALL_SIZE: u32 = 40;

pub struct UploadAvatarUseCase<T, U>
where
    T: AvatarRepository + Send + Sync,
    U: AccountRepository + Send + Sync,
{
    avatar_repository: T,
    account_repository: Arc<U>,
    upload_dir: PathBuf,
}

impl<T, U> UploadAvatarUseCase<T, U>
where
    T: AvatarRepository + Send + Sync,
    U: AccountRepository + Send + Sync,
{
    pub fn new(avatar_repository: T, account_repository: Arc<U>, upload_dir: PathBuf) -> Self {
        Self {
            avatar_repository,
            account_repository,
            upload_dir,
        }
    }

    pub async fn get_avatar(&self, account_id: i32) -> Result<Option<AvatarUrls>, Box<dyn std::error::Error>> {
        let avatar = self.avatar_repository.find_latest_by_account_id(account_id).await?;

        Ok(avatar.and_then(|a| {
            match (a.avatar_300x300_url, a.avatar_40x40_url) {
                (Some(large_url), Some(small_url)) => Some(AvatarUrls {
                    avatar_300x300_url: large_url,
                    avatar_40x40_url: small_url,
                }),
                _ => None
            }
        }))
    }

    pub async fn execute(&self, account_id: i32, image_data: Vec<u8>) -> Result<AvatarUploadResponse, Box<dyn std::error::Error>> {
        // Create account-specific directory
        let account_dir = self.upload_dir.join(account_id.to_string());
        std::fs::create_dir_all(&account_dir)?;

        // Process images
        let img = image::load_from_memory(&image_data)?;

        // Generate UUIDs for filenames
        let large_uuid = Uuid::new_v4();
        let small_uuid = Uuid::new_v4();

        // Process large image (300x300)
        let large_image = img.resize(LARGE_SIZE, LARGE_SIZE, image::imageops::FilterType::Lanczos3);
        let large_filename = format!("300_{}.webp", large_uuid);
        let large_path = account_dir.join(&large_filename);
        let large_webp = self.create_webp(&large_image)?;
        std::fs::write(&large_path, large_webp)?;

        // Process small image (40x40)
        let small_image = img.resize(SMALL_SIZE, SMALL_SIZE, image::imageops::FilterType::Lanczos3);
        let small_filename = format!("40_{}.webp", small_uuid);
        let small_path = account_dir.join(&small_filename);
        let small_webp = self.create_webp(&small_image)?;
        std::fs::write(&small_path, small_webp)?;

        // Create URLs (relative to upload directory)
        let large_url = format!("/uploads/{}/{}", account_id, large_filename);
        let small_url = format!("/uploads/{}/{}", account_id, small_filename);

        // Save to database and set as default avatar
        let avatar = self.avatar_repository.create(
            account_id,
            large_url.clone(),
            small_url.clone(),
        ).await?;

        // Set as default avatar
        self.account_repository.set_default_avatar(account_id, avatar.id).await?;

        Ok(AvatarUploadResponse {
            avatar_300x300_url: large_url,
            avatar_40x40_url: small_url,
            message: "Avatar uploaded successfully".to_string(),
        })
    }

    fn create_webp(&self, img: &image::DynamicImage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let rgba = img.to_rgba8();
        let encoder = Encoder::from_rgba(&rgba, img.width(), img.height());
        let encoded = encoder.encode(75f32); // Quality factor of 75
        Ok(encoded.to_vec())
    }
}