use mongodb::Collection;
use crate::models::calls_model::Calls;
use futures::stream::StreamExt;
use std::error::Error;

pub async fn get_all_calls(collection: &Collection<Calls>) -> Result<Vec<Calls>, Box<dyn Error>> {
    let mut cursor = collection.find(None, None).await?;
    let mut calls: Vec<Calls> = Vec::new();
    
    while let Some(call) = cursor.next().await {
        match call {
            Ok(call_data) => calls.push(call_data),
            Err(e) => eprintln!("Error al obtener llamada: {}", e),
        }
    }
    
    Ok(calls)
}