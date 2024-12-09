use actix_web::{web, Scope};
use crate::handlers::user_handler::{get_users_handler, get_user_by_email_handler}; // Importa el nuevo handler

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("", 
        web::get().to(get_users_handler)) // Ruta para obtener todos los usuarios
        .route("/search", web::get().to(get_user_by_email_handler))
}