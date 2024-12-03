use actix_web::{web, Scope};
use crate::handlers::user_handler::get_users_handler;

pub fn user_routes() -> Scope {
    web::scope("/users") 
        .route("", web::get().to(get_users_handler))
}