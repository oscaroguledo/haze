// src/operations/table/tb.rs

use sqlx::{Pool, Postgres, Error, Row};
use serde_json::{json, Value};
use crate::operations::database::db::{get_database_name};

/// Creates a table with the specified name and columns.
pub async fn create_table(
    pool: &Pool<Postgres>,
    schema_name: &str,
    table_name: &str,
    columns: &str, // Example: "id SERIAL PRIMARY KEY, name VARCHAR(100)"
) -> Result<(), Error> {
    let query = format!("CREATE TABLE {}.{} ({})", schema_name, table_name, columns);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

/// Fetches all tables in the specified schema, including their columns and other metadata.
pub async fn fetch_all_tables(
    pool: &Pool<Postgres>,
    schema_name: &str,
) -> Result<Vec<Row>, Error> {
    // Fetch all tables in the schema
    let rows = sqlx::query(
        r#"
        SELECT table_name, table_type, is_insertable_into, table_schema
        FROM information_schema.tables
        WHERE table_schema = $1
        "#,
    )
    .bind(schema_name)
    .fetch_all(pool)
    .await?;

    Ok(rows) // Return the rows as Vec<Row>
}

/// Fetches table metadata by its name in the specified schema.
pub async fn fetch_table_by_name(
    pool: &Pool<Postgres>,
    schema_name: &str,
    table_name: &str,
) -> Result<Option<Row>, Error> {
    // Query to check if the table exists and fetch its metadata
    let row = sqlx::query(
        r#"
        SELECT table_name, table_type, is_insertable_into
        FROM information_schema.tables
        WHERE table_schema = $1 AND table_name = $2
        "#,
    )
    .bind(schema_name)
    .bind(table_name)
    .fetch_optional(pool)
    .await?;

    Ok(row) // Return the row as Option<Row>, or None if no table is found
}

/// Deletes a table by name within a schema.
pub async fn delete_table_by_name(
    pool: &Pool<Postgres>,
    schema_name: &str,
    table_name: &str,
) -> Result<(), Error> {
    let query = format!("DROP TABLE IF EXISTS {}.{} CASCADE", schema_name, table_name);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

/// Deletes all tables within a schema.
pub async fn delete_all_tables(pool: &Pool<Postgres>, schema_name: &str) -> Result<(), Error> {
    let rows = sqlx::query(
        r#"
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = $1
        "#,
    )
    .bind(schema_name)
    .fetch_all(pool)
    .await?;

    // Iterate over all the table names and delete them one by one
    for row in rows {
        let table_name: String = row.get("table_name");
        // Call delete_table_by_name for each table
        delete_table_by_name(pool, schema_name, &table_name).await?;
    }

    Ok(())
}