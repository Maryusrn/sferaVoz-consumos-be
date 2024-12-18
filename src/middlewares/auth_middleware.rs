use jsonwebtoken::{decode, Validation, DecodingKey};
use actix_web::{dev::ServiceRequest, Error, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub async fn auth_middleware(
    req: ServiceRequest,
    secret: &str,
) -> Result<ServiceRequest, (Error, HttpResponse)> {
    if let Some(header) = req.headers().get("Authorization") {
        if let Ok(token) = header.to_str() {
            match validate_jwt(token, secret) {
                Ok(_) => Ok(req),
                Err(_) => Err((
                    actix_web::error::ErrorUnauthorized("Invalid token"),
                    HttpResponse::Unauthorized().finish(),
                )),
            }
        } else {
            Err((
                actix_web::error::ErrorUnauthorized("Invalid token format"),
                HttpResponse::Unauthorized().finish(),
            ))
        }
    } else {
        Err((
            actix_web::error::ErrorUnauthorized("Missing Authorization header"),
            HttpResponse::Unauthorized().finish(),
        ))
    }
}