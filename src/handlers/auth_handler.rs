use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use mongodb::{Collection, Client};

use crate::config::config::jwt_secret;
use crate::services::auth_service::login_service;
use crate::models::user_model::User;
use crate::services::jwt_service::generate_jwt;
use crate::utils::validator::{validate_email, validate_password};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login_handler(
    login_request: web::Json<LoginRequest>,
    db_client: web::Data<Client>,  // Cambiado para recibir el cliente de MongoDB
) -> impl Responder {

    if let Err(e) = validate_email(&login_request.email) {
        return HttpResponse::BadRequest().json(e);
    }

    if let Err(e) = validate_password(&login_request.password) {
        return HttpResponse::BadRequest().json(e);
    }

    let user_collection: Collection<User> = db_client.database("test").collection("users");

    match login_service(&login_request.email, &login_request.password, &user_collection).await {
        Ok(true) => {
            let secret = jwt_secret();
            match generate_jwt(&login_request.email, &secret) {
                Ok(token) => HttpResponse::Ok().json(serde_json::json!({
                    "message": "Login exitoso",
                    "token": token
                })),
                Err(e) => HttpResponse::InternalServerError().json(format!("Error al generar el token: {}", e)),
            }
        }
        Ok(false) => HttpResponse::Unauthorized().json("Contraseña incorrecta"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}