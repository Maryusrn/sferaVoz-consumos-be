use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::error::Error;
use chrono::{Utc, Duration};

use crate::models::user_model::User;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    email: String,
    rol: bool,
    exp: usize,
}

pub fn generate_token(user: &User, secret: &str, remember_me: bool) -> Result<String, Box<dyn Error>> {

    let expires_in = if remember_me {
        Utc::now() + Duration::days(4)
    } else {
        Utc::now() + Duration::hours(8)
    };

    let claims = Claims {
        id: user.id.clone().map_or_else(|| "unknown".to_string(), |oid| oid.to_hex()),
        email: user.email.clone(),
        rol: user.rol.clone(),
        exp: expires_in.timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;

    Ok(token)
}