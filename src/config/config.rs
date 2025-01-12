use std::env;

pub fn jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string())
}