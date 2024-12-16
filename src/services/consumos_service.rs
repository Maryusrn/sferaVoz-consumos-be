use mongodb::Collection;
use crate::models::consumos_model::Consumos;
use futures::stream::StreamExt;
use std::error::Error;

pub async fn get_all_consumos_hour(collection: &Collection<Consumos>) -> Result<Vec<Consumos>, Box<dyn Error>> {
    let mut cursor = collection.find(None, None).await?;
    let mut consumos: Vec<Consumos> = Vec::new();
    
    while let Some(consumo) = cursor.next().await {
        match consumo {
            Ok(consumo_data) => consumos.push(consumo_data),
            Err(e) => eprintln!("Error al obtener consumo: {}", e),
        }
    }
    Ok(consumos)
}