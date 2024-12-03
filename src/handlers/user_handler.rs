use actix_web::{HttpResponse, Responder};
use crate::services::user_service::get_all_users;
use crate::config::database::establish_connection;
use crate::models::user_model::User;

pub async fn get_users_handler() -> impl Responder {
    let client = match establish_connection().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error al conectar con la base de datos: {}", e);
            return HttpResponse::InternalServerError().json("Error al conectar con la base de datos");
        }
    };

    let db = client.database("test");
    let collection = db.collection::<User>("users");
    
    match get_all_users(&collection).await {
        Ok(users) => HttpResponse::Ok().json(users), 
        Err(e) => {
            eprintln!("Error al obtener usuarios: {}", e);
            HttpResponse::NotFound().json("Usuarios no encontrados")
        }
    }
}
