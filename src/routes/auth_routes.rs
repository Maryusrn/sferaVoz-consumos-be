use actix_web::{web, Scope};
use crate::handlers::auth_handler::login_handler;
pub fn auth_routes() -> Scope {
    web::scope("/login")
        .route("", web::post().to(login_handler))
}