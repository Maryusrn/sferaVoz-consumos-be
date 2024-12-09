use actix_web::{web, HttpResponse, Responder};
use mongodm::doc;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    email: String,
}

pub async fn get_user_by_email_handler(query: web::Query<SearchQuery>) -> impl Responder {
    let client_result = establish_connection().await;
    match client_result {
        Ok(client) => {
            let db = client.database("test"); // Cambia "mydb" por tu base de datos
            let users_collection = db.collection::<User>("users");

            let filter = doc! { "email": &query.email };
            let user = users_collection.find_one(filter, None).await;

            match user {
                Ok(Some(u)) => {
                    HttpResponse::Ok().json(UserResponse {
                        id: u.id.unwrap_or_default().to_hex(),
                        name: u.name,
                        email: u.email,
                    })
                }
                Ok(None) => {
                    HttpResponse::NotFound().json("Usuario no encontrado")
                }
                Err(_) => {
                    HttpResponse::InternalServerError().json("Error al obtener el usuario")
                }
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().json("Error al conectar con la base de datos")
        }
    }
}