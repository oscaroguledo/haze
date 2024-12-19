
// src/operations/schema/sc.rs
use sqlx::{Postgres, Error, Row};
use serde_json::{json, Value};
use std::result::Result; // Result type for returning errors

use crate::operations::database::db::{get_database_name};


pub async fn create_schema(pool: &sqlx::Pool<sqlx::Postgres>, schema_name: &str) -> Result<(), Error> {
    // List of system databases that should not be modified
    let system_databases = vec!["postgres", "template1", "template0"];

    // Get the current database name
    let current_db_name = get_database_name(pool).await?;

    if system_databases.contains(&current_db_name.as_str()) {
        return Err(sqlx::Error::Protocol("Cannot create schema in system database".to_string()));
    }

    // Execute the create schema query in the current database
    let query = format!("CREATE SCHEMA {}", schema_name);
    sqlx::query(&query)
        .execute(pool)
        .await?;

    Ok(())
}

// Assuming CustomError is a custom error type defined elsewhere
pub async fn fetch_all_schema(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Value, Error> {
    // Query to fetch schema properties (excluding system schemas)
    let rows = sqlx::query!(
        r#"
        SELECT 
            schema_name,
            schema_owner
        FROM information_schema.schemata
        WHERE schema_name NOT IN ('pg_catalog', 'information_schema')
        "#,
    )
    .fetch_all(pool)
    .await?;

    // Collect schemas and their properties into a Vec of JSON values
    let schemas: Vec<Value> = rows.into_iter().map(|row| {
        json!({
            "schema_name": row.schema_name,
            "owner": row.schema_owner
        })
    }).collect();

    // Wrap the result in a JSON object
    Ok(json!({ "schemas": schemas }))
}


pub async fn fetch_schema_by_name(pool: &sqlx::Pool<Postgres>, schema_name: &str) -> Result<Value, Error> {
    let row = sqlx::query!(
        r#"
        SELECT schema_name, schema_owner
        FROM information_schema.schemata
        WHERE schema_name = $1
        "#,
        schema_name
    )
    .fetch_one(pool)
    .await?;

    let schema = json!({
        "schema_name": row.schema_name,
        "owner": row.schema_owner,
    });

    Ok(schema)
}
pub async fn delete_schema_by_name(pool: &sqlx::Pool<Postgres>, schema_name: &str) -> Result<(), Error> {
    // Deleting schema with CASCADE to remove any objects inside it
    sqlx::query(
        r#"
        DROP SCHEMA IF EXISTS $1 CASCADE
        "#,
    )
    .bind(schema_name) // Bind the schema name as a parameter
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_all_schemas(pool: &sqlx::Pool<Postgres>) -> Result<(), Error> {
    // Fetching all user schemas (excluding system schemas)
    let rows = sqlx::query(
        r#"
        SELECT schema_name
        FROM information_schema.schemata
        WHERE schema_name NOT IN ('pg_catalog', 'information_schema')
        "#,
    )
    .fetch_all(pool)
    .await?;

    // Deleting each user schema
    for row in rows {
        // Get the schema_name from the first column (index 0)
        let schema_name: String = row.get("schema_name");  // This will correctly fetch the schema name


        sqlx::query(
            r#"
            DROP SCHEMA IF EXISTS $1 CASCADE
            "#,
        )
        .bind(schema_name)  // Bind the schema_name as a parameter
        .execute(pool)
        .await?;
    }

    Ok(())
}
