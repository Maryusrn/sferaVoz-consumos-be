use mongodb::Collection;
use mongodm::{doc, prelude::ObjectId};
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

pub async fn find_user_by_id_service(
    user_id: &ObjectId,
    user_collection: &Collection<User>,
) -> Result<User, String> {
    match user_collection.find_one(doc! {"_id": user_id}, None).await {
        Ok(Some(user)) => Ok(user),  //
        Ok(None) => Err("Usuario no encontrado".to_string()), 
        Err(err) => Err(format!("Error al acceder a la base de datos: {}", err)),
    }
}