use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::models::user_model::User;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    email: String,
}

pub fn generate_token(user: &User, secret: &str) -> Result<String, Box<dyn Error>> {

    let claims = Claims {
        id: user.id.clone().map_or_else(|| "unknown".to_string(), |oid| oid.to_hex()),
        email: user.email.clone(),
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;

    Ok(token)
}