use actix_web::{HttpResponse, Responder};
use crate::services::calls_service::get_all_calls;
use crate::config::database::establish_connection;
use crate::models::calls_model::Calls;

pub async fn get_calls_handler() -> impl Responder {
    let client = match establish_connection().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error al conectar con la base de datos: {}", e);
            return HttpResponse::InternalServerError().json("Error al conectar con la base de datos");
        }
    };

    let db = client.database("consumos");
    let collection = db.collection::<Calls>("calls");
    
    match get_all_calls(&collection).await {
        Ok(calls) => HttpResponse::Ok().json(calls), 
        Err(e) => {
            eprintln!("Error al obtener registro de llamadas: {}", e);
            HttpResponse::NotFound().json("Registros no encontrados")
        }
    }
}
