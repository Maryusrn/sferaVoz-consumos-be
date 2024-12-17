use actix_web::{web, Scope};
use crate::handlers::consumos_handler::{get_consumos_hour, get_consumos_month, get_consumos_year};

pub fn consumos_routes() -> Scope {
    web::scope("/getConsumos") 
        .route("/hour", web::get().to(get_consumos_hour))
        .route("/month", web::get().to(get_consumos_month))
        .route("/year", web::get().to(get_consumos_year))
}