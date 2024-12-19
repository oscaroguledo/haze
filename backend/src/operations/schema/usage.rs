// src/main.rs
mod operations;
use dotenv::dotenv;
use std::env;
// use operations::database::db::{create_database, drop_all_databases, drop_database_by_name, fetch_all_system_databases, fetch_system_database_by_name,fetch_all_databases, fetch_database_by_name};
use operations::schema::sc::{fetch_all_schema, create_schema,fetch_schema_by_name};
use operations::database::db::{create_pool,get_database_name,fetch_all_privileges, revoke_privileges};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env
    dotenv().ok();
    // Retrieve database URL from an environment variable or configuration
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = create_pool(&db_url.to_string()).await?;
    let db_name = get_database_name(&pool).await?;
    println!("db_name: {}", db_name);
    // List schemas
    let schemas = fetch_all_schema(&pool).await.unwrap();
    println!("Schemas: {:?}", schemas);

    let schema = fetch_schema_by_name(&pool, "public").await.unwrap();
    println!("Schema: {:?}", schema);
    
    // // Delete a schema by name
    // if let Err(e) = delete_schema_by_name(&pool, "public").await {
    //     println!("Error deleting schema: {}", e);
    // }

    // // Delete all user-defined schemas
    // if let Err(e) = delete_all_schemas(&pool).await {
    //     println!("Error deleting all schemas: {}", e);
    // }
    // match fetch_all_privileges(&pool, &db_name).await {
    //     Ok(privileges) => {
    //         for (grantee, schema, privilege) in privileges {
    //             println!("Grantee: {}, Schema: {}, Privilege: {}", grantee, schema, privilege);
    //         }
    //     },
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    Ok(())
}