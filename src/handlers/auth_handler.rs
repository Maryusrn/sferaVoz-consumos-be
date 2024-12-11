use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;
use mongodb::{Collection, Client};

use crate::config::config::jwt_secret;
use crate::services::auth_service::login_service;
use crate::models::user_model::User;
use crate::utils::validator::{validate_email, validate_password};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login_handler(
    login_request: web::Json<LoginRequest>,
    db_client: web::Data<Client>,
) -> impl Responder {

    if let Err(e) = validate_email(&login_request.email) {
        return HttpResponse::BadRequest().json(e);
    }

    if let Err(e) = validate_password(&login_request.password) {
        return HttpResponse::BadRequest().json(e);
    }

    let user_collection: Collection<User> = db_client.database("test").collection("users");
    let secret = jwt_secret();

    match login_service(
        &login_request.email,
        &login_request.password,
        &user_collection,
        &secret,
    )
    .await
    {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Login exitoso",
            "token": token
        })),
        Err(e) => {
            info!("Error en el login: {}", e);
            match e.as_str() {
                "Contraseña incorrecta" => HttpResponse::Unauthorized().json(e),
                "Usuario no encontrado" => HttpResponse::NotFound().json(e),
                _ => HttpResponse::InternalServerError().json(e),
            }
        }
    }
}