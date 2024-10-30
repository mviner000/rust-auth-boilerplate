use reqwest::{Client, multipart};
use std::{error::Error, fs, path::Path};

#[tokio::test]
async fn test_avatar_upload() -> Result<(), Box<dyn Error>> {
    let account_id = "12"; // Ensure this is a valid account ID
    let file_path = "/media/ssd-ubuntu/ntfs-drive/master-rust/new1/src/tests/upload_avatar_test/test_avatar.jpg";
    let bearer_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjEyLCJleHAiOjE3MzAzODIwMzIsImlhdCI6MTczMDI5NTYzMn0.6JqGMxrZFr9faubnK8PWlCSoWEfmhLSJJcsa9WBYeDs";

    // Check if file exists before proceeding
    let file_path = Path::new(file_path);
    if !file_path.exists() {
        return Err(format!(
            "Test file does not exist at path: {}",
            file_path.display()
        )
            .into());
    }

    // Validate file is actually a file (not a directory)
    if !file_path.is_file() {
        return Err(format!(
            "Path exists but is not a file: {}",
            file_path.display()
        )
            .into());
    }

    // Read the file bytes and prepare the multipart form
    let file_bytes = fs::read(file_path)?;
    let part = multipart::Part::bytes(file_bytes)
        .file_name(file_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("avatar.jpg")
            .to_string());
    let form = multipart::Form::new().part("avatar", part);

    // Upload the file
    let client = Client::new();
    // Fixed URL path to match the actual API endpoint
    let upload_url = format!("http://localhost:8080/api/v1/account/{}/avatar", account_id);

    println!("Sending request to: {}", upload_url);

    let response = client
        .post(&upload_url)
        .multipart(form)
        .header("Authorization", format!("Bearer {}", bearer_token))
        .send()
        .await?;

    // Enhanced error reporting
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!(
            "Failed to upload avatar. Status: {}, Error: {}\nEndpoint: {}",
            status, error_text, upload_url
        )
            .into());
    }

    println!("Upload successful!");
    Ok(())
}