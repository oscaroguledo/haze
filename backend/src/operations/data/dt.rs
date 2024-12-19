// src/operations/data/dt.rs

use sqlx::{Pool, Postgres, Error, Row};
use std::collections::HashMap;
use serde_json::{json, Value};
use std::result::Result; // Result type for returning errors
use crate::operations::database::db::{get_database_name};
use sqlx::{Pool, Postgres, Error, Row};
use serde_json::{json, Value};

/// Inserts data into a table with dynamically provided fields.
pub async fn insert_data(
    pool: &Pool<Postgres>,
    schema_name: &str,
    table_name: &str,
    fields: &HashMap<String, Value>,
) -> Result<(), Error> {
    // Extract column names and values
    let columns: Vec<&str> = fields.keys().map(|key| key.as_str()).collect();
    let placeholders: Vec<String> = (1..=fields.len()).map(|i| format!("${}", i)).collect();
    let values: Vec<&Value> = fields.values().collect();

    // Build the query dynamically
    let query = format!(
        "INSERT INTO {}.{} ({}) VALUES ({})",
        schema_name,
        table_name,
        columns.join(", "),
        placeholders.join(", ")
    );

    // Execute the query with the provided values
    sqlx::query_with(&query, values).execute(pool).await?;
    Ok(())
}

/// Fetches all data from a table within a schema.
pub async fn fetch_all_data(
    pool: &Pool<Postgres>,
    schema_name: &str,
    table_name: &str,
    ) -> Result<Value, Error> {
        // Query to fetch all rows
        let query = format!("SELECT * FROM {}.{}", schema_name, table_name);

        // Fetch all rows
        let rows = sqlx::query(&query).fetch_all(pool).await?;

        // Map rows into JSON objects
        let data: Vec<Value> = rows
            .into_iter()
            .map(|row| {
                let json_object: serde_json::Map<String, Value> = row
                    .columns()
                    .iter()
                    .map(|col| {
                        let column_name = col.name().to_string();
                        let value: Value = row.try_get::<Value, &str>(column_name.as_str()).unwrap_or(json!(null));
                        (column_name, value)
                    })
                    .collect();
                json!(json_object)
            })
            .collect();

        Ok(json!(data))
}


/// Fetches a single item by ID from a table within a schema.
pub async fn fetch_item_by_id(
    pool: &Pool<Postgres>,
    schema_name: &str,
    table_name: &str,
    id_column: &str,
    id_value: &Value,
) -> Result<Option<Value>, Error> {
    let query = format!(
        "SELECT * FROM {}.{} WHERE {} = $1",
        schema_name, table_name, id_column
    );

    // Fetch the row
    if let Some(row) = sqlx::query(&query).bind(id_value).fetch_optional(pool).await? {
        let json_object: serde_json::Map<String, Value> = row
            .columns()
            .iter()
            .map(|col| {
                let column_name = col.name().to_string();
                let value: Value = row.try_get::<Value, &str>(column_name.as_str()).unwrap_or(json!(null));
                (column_name, value)
            })
            .collect();
        Ok(Some(json!(json_object)))
    } else {
        Ok(None)
    }
}

/// Deletes a single item by ID from a table within a schema.
pub async fn delete_by_id(
    pool: &Pool<Postgres>,
    schema_name: &str,
    table_name: &str,
    id_column: &str,
    id_value: &Value,
) -> Result<(), Error> {
    let query = format!(
        "DELETE FROM {}.{} WHERE {} = $1",
        schema_name, table_name, id_column
    );

    sqlx::query(&query).bind(id_value).execute(pool).await?;
    Ok(())
}

/// Deletes all records from a table within a schema.
pub async fn delete_all_records(
    pool: &Pool<Postgres>,
    schema_name: &str,
    table_name: &str,
) -> Result<(), Error> {
    let query = format!("DELETE FROM {}.{}", schema_name, table_name);

    sqlx::query(&query).execute(pool).await?;
    Ok(())
}