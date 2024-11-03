use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, dev::ServiceRequest, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::domain::entities::auth::Claims;

#[allow(dead_code)]  // Added because the compiler can't detect usage through middleware configuration
pub async fn validator(req: ServiceRequest, credentials: BearerAuth)
                       -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let token = credentials.token();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            // Add claims to request extensions for use in handlers
            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        },
        Err(_) => Err((ErrorUnauthorized("Invalid token"), req)),
    }
}