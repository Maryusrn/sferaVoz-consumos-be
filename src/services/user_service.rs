use mongodb::Collection;
use crate::models::user_model::User;
use futures::stream::StreamExt;
use std::error::Error;

pub async fn get_all_users(collection: &Collection<User>) -> Result<Vec<User>, Box<dyn Error>> {
    let mut cursor = collection.find(None, None).await?;
    let mut users: Vec<User> = Vec::new();
    
    while let Some(user) = cursor.next().await {
        match user {
            Ok(user_data) => users.push(user_data),
            Err(e) => eprintln!("Error al obtener usuario: {}", e),
        }
    }
    
    Ok(users)
}