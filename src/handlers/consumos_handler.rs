use actix_web::{HttpResponse, Responder};
use crate::services::consumos_service::get_all_consumos_hour;
use crate::config::database::establish_connection;
use crate::models::consumos_model::Consumos;

pub async fn get_consumos_hour() -> impl Responder {
    let client = match establish_connection().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error al conectar con la base de datos: {}", e);
            return HttpResponse::InternalServerError().json("Error al conectar con la base de datos");
        }
    };

    let db = client.database("test");
    let collection = db.collection::<Consumos>("consumos");
    
    match get_all_consumos_hour(&collection).await {
        Ok(consumos) => HttpResponse::Ok().json(consumos), 
        Err(e) => {
            eprintln!("Error al obtener registro de consumos: {}", e);
            HttpResponse::NotFound().json("Consumos no encontrados")
        }
    }
}