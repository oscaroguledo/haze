mod controllers;
mod db;
mod models;
mod routes;

use actix_web::{App, HttpServer};
use db::establish_connection;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_pool = establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(routes::index.configure_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
