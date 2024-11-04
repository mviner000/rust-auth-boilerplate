// File: src/tests/upload_avatar_test/upload_avatar_test.rs

use reqwest::{Client, multipart, StatusCode};
use std::{error::Error, path::PathBuf};
use serde_json::Value;

const API_BASE_URL: &str = "http://127.0.0.1:8080/api/v1";
const TEST_ACCOUNT_ID: i32 = 1;
const TEST_BEARER_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjEsImV4cCI6MTczMDgwODQ2NCwiaWF0IjoxNzMwNzIyMDY0fQ.Cx09DlbQ1GXO1Dia_1ba6IWyU8E-eQjisccGVXUGHNE";

struct TestClient {
    client: Client,
    base_url: String,
    bearer_token: String,
}

impl TestClient {
    fn new(base_url: &str, bearer_token: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            bearer_token: bearer_token.to_string(),
        }
    }

    async fn upload_avatar(&self, account_id: i32, file_path: &PathBuf, mime_type: &str) -> Result<reqwest::Response, Box<dyn Error>> {
        let file_bytes = tokio::fs::read(file_path).await?;
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("avatar")
            .to_string();

        let part = multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str(mime_type)?;

        let form = multipart::Form::new().part("avatar", part);

        let upload_url = format!("{}/avatars/{}", self.base_url, account_id);
        let response = self.client
            .post(&upload_url)
            .multipart(form)
            .header("Authorization", format!("Bearer {}", self.bearer_token))
            .send()
            .await?;

        Ok(response)
    }

    async fn get_account(&self, account_id: i32) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/account/{}", self.base_url, account_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.bearer_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

#[tokio::test]
async fn test_avatar_upload_success() -> Result<(), Box<dyn Error>> {
    let test_client = TestClient::new(API_BASE_URL, TEST_BEARER_TOKEN);

    // Get initial account state
    let initial_account = test_client.get_account(TEST_ACCOUNT_ID).await?;
    let initial_avatar_id = initial_account["default_avatar_id"].as_i64();

    // Use existing test image
    let test_image = PathBuf::from("src/tests/upload_avatar_test/test_avatar.jpg");

    // Upload avatar
    let response = test_client.upload_avatar(TEST_ACCOUNT_ID, &test_image, "image/jpeg").await?;

    assert_eq!(response.status(), StatusCode::OK,
               "Upload failed with response: {:?}", response.text().await?);

    // Verify account was updated
    let updated_account = test_client.get_account(TEST_ACCOUNT_ID).await?;
    let new_avatar_id = updated_account["default_avatar_id"].as_i64();

    assert!(new_avatar_id.is_some(), "Avatar ID should be set");
    assert_ne!(new_avatar_id, initial_avatar_id, "Avatar ID should have changed");

    Ok(())
}

#[tokio::test]
async fn test_avatar_upload_invalid_mime_type() -> Result<(), Box<dyn Error>> {
    let test_client = TestClient::new(API_BASE_URL, TEST_BEARER_TOKEN);

    // Create test text file
    let test_file = PathBuf::from("src/tests/upload_avatar_test/not_an_image.txt");
    tokio::fs::write(&test_file, b"This is not an image").await?;

    // Attempt upload with text/plain mime type
    let response = test_client.upload_avatar(TEST_ACCOUNT_ID, &test_file, "text/plain").await?;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST,
               "Should reject invalid mime type. Response: {:?}", response.text().await?);

    // Cleanup
    tokio::fs::remove_file(test_file).await?;

    Ok(())
}

#[tokio::test]
async fn test_avatar_upload_unauthorized() -> Result<(), Box<dyn Error>> {
    let test_client = TestClient::new(API_BASE_URL, "invalid_token");
    let test_image = PathBuf::from("src/tests/upload_avatar_test/test_avatar.jpg");

    let response = test_client.upload_avatar(TEST_ACCOUNT_ID, &test_image, "image/jpeg").await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED,
               "Should reject invalid token. Response: {:?}", response.text().await?);

    Ok(())
}

#[tokio::test]
async fn test_avatar_upload_nonexistent_account() -> Result<(), Box<dyn Error>> {
    let test_client = TestClient::new(API_BASE_URL, TEST_BEARER_TOKEN);
    let test_image = PathBuf::from("src/tests/upload_avatar_test/test_avatar.jpg");

    let response = test_client.upload_avatar(999999, &test_image, "image/jpeg").await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND,
               "Should reject non-existent account. Response: {:?}", response.text().await?);

    Ok(())
}