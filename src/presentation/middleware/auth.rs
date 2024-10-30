use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, dev::ServiceRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::domain::entities::auth::Claims;

pub async fn validator(req: ServiceRequest, credentials: BearerAuth)
                       -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = credentials.token();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(_) => Ok(req),
        Err(_) => Err((ErrorUnauthorized("Invalid token"), req)),
    }
}