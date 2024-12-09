use actix_web::{web, Scope};
use crate::handlers::calls_handler::get_calls_handler;

pub fn calls_routes() -> Scope {
    web::scope("/calls") 
        .route("", web::get().to(get_calls_handler))
}