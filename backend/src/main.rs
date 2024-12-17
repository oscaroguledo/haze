mod controllers;
mod websocket;
mod db;
mod models;
mod routes;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::web::Data;
use dotenv::dotenv;
use std::env;
use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_pool = establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(routes::index::configure_routes)
            .route("/ws/", web::get().to(websocket::start_ws))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
