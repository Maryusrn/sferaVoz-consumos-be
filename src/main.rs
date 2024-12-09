use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use routes::calls_routes::calls_routes; // Importa Cors
use crate::routes::{users_routes::user_routes, auth_routes::auth_routes};
use crate::config::database::establish_connection; // Importa conexión a la base de datos

mod config;
mod models;
mod services;
mod handlers;
mod routes;
mod utils;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    println!("Server is listening on http://127.0.0.1:3000");

    std::env::set_var("RUST_LOG", "actix_web=debug");

    let db_client = establish_connection().await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Permite solicitudes desde cualquier origen
            .allow_any_method() // Permite todos los métodos HTTP (GET, POST, etc.)
            .allow_any_header() // Permite cualquier encabezado
            .supports_credentials(); // Incluye credenciales si es necesario

        App::new()
            .wrap(Logger::default())
            .wrap(cors) // Aplica CORS como middleware
            .app_data(web::Data::new(db_client.clone()))
            .configure(|cfg| {
                cfg.service(user_routes());
                cfg.service(auth_routes());
                cfg.service(calls_routes());
            })
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}