use bcrypt::{hash, DEFAULT_COST};
use mongodb::Client;
use mongodm::prelude::ObjectId;
use crate::models::user_model::User;
use dotenv::dotenv;
use std::env;

pub async fn register_service(
    nombre: &str,
    email: &str,
    password: &str,
) -> Result<(), String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(|e| e.to_string())?;
    let client = Client::with_uri_str(&database_url).await.map_err(|e| e.to_string())?;
    let database = client.database("consumos");
    let collection = database.collection::<User>("users");

    let hashed_password = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;

    let new_user = User {
        id: Some(ObjectId::new()),
        name: nombre.to_string(),
        email: email.to_string(),
        password: hashed_password,
        rol: false,
    };

    collection.insert_one(new_user, None).await.map_err(|e| e.to_string())?;

    Ok(())
}