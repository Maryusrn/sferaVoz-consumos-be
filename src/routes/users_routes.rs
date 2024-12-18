use actix_web::{web, Scope};
use crate::handlers::user_handler::{get_users_handler, get_user_by_id_handler}; // Importa el nuevo handler

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("", 
        web::get().to(get_users_handler))
        .route("/{id}", web::get().to(get_user_by_id_handler))
}