use actix_web::{web, HttpResponse, Responder};
use mongodb::{Client, Collection};
use mongodm::doc;
use mongodm::prelude::ObjectId;
use serde::Serialize;
use crate::services::user_service::{find_user_by_id_service, get_all_users};
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

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    name: String,
    email: String,
}

pub async fn get_user_by_id_handler(
    path: web::Path<String>, 
    db_client: web::Data<Client>,
) -> impl Responder {
    let user_id_str = path.into_inner();

    let user_collection: Collection<User> = db_client.database("test").collection("users");

    match ObjectId::parse_str(&user_id_str) {
        Ok(user_id) => {
            match find_user_by_id_service(&user_id, &user_collection).await {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(error) => HttpResponse::NotFound().body(error),
            }
        }
        Err(_) => {
            HttpResponse::BadRequest().body("ID de usuario no válido")
        }
    }
}
