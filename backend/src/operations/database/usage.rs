// src/main.rs
mod operations;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use std::env;
use operations::database::db::{create_database, drop_all_databases, drop_database_by_name, fetch_all_system_databases, fetch_system_database_by_name,fetch_all_databases, fetch_database_by_name};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env
    dotenv().ok();

    // Get the database connection string from the .env file
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Set up the connection pool to the PostgreSQL instance
    let pool = PgPoolOptions::new()
        .max_connections(5) // Set the max number of concurrent connections
        .connect(&database_url) // Connect to the PostgreSQL database
        .await?;
    // Create a new database with optional password
    //     let db_name = "new_database";
    //     let password = Some("password123"); // Optional password (set to None if no password is provided)
    //     create_database(&pool, db_name, password).await?;

    //     println!("Database '{}' created successfully!", db_name);
    // Drop all databases except the default system ones
    fetch_all_databases(&pool).await?;
    // Fetch all non-system databases
    let databases = fetch_all_databases(&pool).await?;

    // Print the names of all fetched databases
    for db in databases {
        println!("Found database: {}", db);
    }
    // Fetch database details
    // let db_name ="admin_db";
    // if let Some(db_details) = fetch_database_by_name(&pool, db_name).await? {
    //     println!("Database Details: {}", db_details);
    // } else {
    //     println!("Database not found or is a system database.");
    // }

    

    Ok(())
}