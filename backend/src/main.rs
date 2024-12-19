mod operations;
mod controllers;
mod websocket;
mod db;
mod models;
mod routes;

use dotenv::dotenv;
use std::env;
use operations::data::dt::{create_table, fetch_all_tables, fetch_table_by_name, delete_all_tables, delete_table_by_name};
use operations::database::db::{create_pool, get_database_name};
use serde_json::json; // For JSON output

use actix_web::{App, HttpServer};
// use actix_web::web::Data;

// use actix_files::Files;

use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_pool = establish_connection().await;
    let _server_addr = "0.0.0.0";
    let _server_port = 8080;
    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:3000")
        //     .allowed_origin("http://localhost:8080")
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            // .wrap(cors)
            .configure(routes::index::configure_routes)
            // .route("/ws/", web::get().to(websocket::start_ws))
    })
    .bind((_server_addr, _server_port))?
    // .workers(3)
    .run()
    .await
}
