
// src/operations/database/db.rs

use sqlx::{Pool, PgPool, Postgres, Error};
use std::collections::HashMap;
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(db_url: &str) -> Result<sqlx::Pool<sqlx::Postgres>, Error> {
    // Create the connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    Ok(pool)
}

pub async fn get_database_name(pool: &PgPool) -> Result<String, Error> {
    let row = sqlx::query!("SELECT current_database()")
        .fetch_one(pool)
        .await?;
    Ok(row.current_database.expect("Database name not found"))
}
// The function is now public and can be used from other modules
pub async fn create_database(pool: &sqlx::Pool<sqlx::Postgres>, db_name: &str, password: Option<&str>) -> Result<(), Error> {
    // Base SQL query to create the database
    let mut query = format!("CREATE DATABASE {};", db_name);

    // If a password is provided, add the password to the query
    if let Some(pass) = password {
        query.push_str(&format!(" WITH OWNER = postgres ENCODING = 'UTF8' CONNECTION LIMIT = -1 TEMPLATE = template0;"));
        query.push_str(&format!(" CREATE USER {} WITH PASSWORD '{}';", db_name, pass));
    }

    // Execute the SQL query to create the database
    sqlx::query(&query)
        .execute(pool)
        .await?;

    Ok(())
}


pub async fn drop_all_databases(pool: &Pool<Postgres>) -> Result<(), Error> {
    // List of system databases that should not be dropped
    let system_databases = vec!["postgres", "template1", "template0","admin_db"];

    // Fetch the list of all databases
    let rows = sqlx::query!("SELECT datname FROM pg_database WHERE datistemplate = false")
        .fetch_all(pool)
        .await?;

    for row in rows {
        let db_name = row.datname;

        // Skip system databases
        if !system_databases.contains(&db_name.as_str()) {
            println!("Dropping database: {}", db_name);

            // Drop the database
            let query = format!("DROP DATABASE IF EXISTS {}; ", db_name);
            sqlx::query(&query)
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}
pub async fn drop_database_by_name(pool: &Pool<Postgres>,db_name: &str, ) -> Result<(), Error> {
    // List of system databases that should not be dropped
    let system_databases = vec!["postgres", "template1", "template0","admin_db"];

    
    // Skip system databases
    if !system_databases.contains(&db_name) {
        println!("Dropping database: {}", db_name);

        // Drop the database
        let query = format!("DROP DATABASE IF EXISTS {}; ", db_name);
        sqlx::query(&query)
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn fetch_all_databases(pool: &Pool<Postgres>) -> Result<Vec<Value>, Error> {
    // List of system databases that should not be fetched
    let system_databases = vec!["postgres", "template1", "template0"];

    // Query to fetch all non-template databases
    let rows = sqlx::query!(
        r#"
        SELECT
            datname AS name,
            pg_size_pretty(pg_database_size(datname)) AS size,
            pg_roles.rolname AS owner,
            datcollate AS collation,
            datctype AS ctype,
            datallowconn AS allow_connections,
            datconnlimit AS connection_limit,
            datistemplate AS is_template
        FROM
            pg_database
        JOIN
            pg_roles ON pg_database.datdba = pg_roles.oid
        WHERE
            datistemplate = false
        "#,
    )
    .fetch_all(pool)
    .await?;

    // Collect all databases into a JSON-friendly structure, excluding system databases
    let databases: Vec<Value> = rows
        .into_iter()
        .filter(|row| !system_databases.contains(&row.name.as_str())) // Exclude system databases
        .map(|row| {
            serde_json::json!({
                "name": row.name,
                "size": row.size,
                "owner": row.owner,
                "collation": row.collation,
                "ctype": row.ctype,
                "allow_connections": row.allow_connections,
                "connection_limit": row.connection_limit,
                "is_template": row.is_template
            })
        })
        .collect();

    Ok(databases)
}

pub async fn fetch_database_by_name(pool: &Pool<Postgres>, db_name: &str) -> Result<Option<serde_json::Value>, Error> {
    // List of system databases that should not be fetched
    let system_databases = vec!["postgres", "template1", "template0"];

    // Check if the requested database is in the list of system databases
    if system_databases.contains(&db_name) {
        return Ok(None); // If it's a system database, return None
    }

    // Query to fetch the database details
    let db_row = sqlx::query!(
        r#"
        SELECT
            datname AS name,
            pg_size_pretty(pg_database_size(datname)) AS size,
            pg_roles.rolname AS owner,
            datcollate AS collation,
            datctype AS ctype,
            datallowconn AS allow_connections,
            datconnlimit AS connection_limit,
            datistemplate AS is_template
        FROM
            pg_database
        JOIN
            pg_roles ON pg_database.datdba = pg_roles.oid
        WHERE
            datname = $1
        "#,
        db_name
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = db_row {
        // Query to fetch tables in the database
        let table_rows = sqlx::query!(
            r#"
            SELECT
                tablename
            FROM
                pg_catalog.pg_tables
            WHERE
                schemaname = 'public'
            "#,
        )
        .fetch_all(pool)
        .await?;

        let tables: Vec<String> = table_rows
            .into_iter()
            .filter_map(|row| row.tablename) // Handle Option
            .collect();

        // Construct JSON result
        let result = serde_json::json!({
            "name": record.name,
            "size": record.size,
            "owner": record.owner,
            "collation": record.collation,
            "ctype": record.ctype,
            "allow_connections": record.allow_connections,
            "connection_limit": record.connection_limit,
            "is_template": record.is_template,
            "tables": tables
        });

        return Ok(Some(result));
    }

    Ok(None)
}
pub async fn fetch_all_system_databases(pool: &Pool<Postgres>) -> Result<Vec<Value>, Error> {
    // List of system databases that should not be fetched
    let system_databases = vec![];

    // Query to fetch all non-template databases
    let rows = sqlx::query!(
        r#"
        SELECT
            datname AS name,
            pg_size_pretty(pg_database_size(datname)) AS size,
            pg_roles.rolname AS owner,
            datcollate AS collation,
            datctype AS ctype,
            datallowconn AS allow_connections,
            datconnlimit AS connection_limit,
            datistemplate AS is_template
        FROM
            pg_database
        JOIN
            pg_roles ON pg_database.datdba = pg_roles.oid
        WHERE
            datistemplate = false
        "#,
    )
    .fetch_all(pool)
    .await?;

    // Collect all databases into a JSON-friendly structure, excluding system databases
    let databases: Vec<Value> = rows
        .into_iter()
        .filter(|row| !system_databases.contains(&row.name.as_str())) // Exclude system databases
        .map(|row| {
            serde_json::json!({
                "name": row.name,
                "size": row.size,
                "owner": row.owner,
                "collation": row.collation,
                "ctype": row.ctype,
                "allow_connections": row.allow_connections,
                "connection_limit": row.connection_limit,
                "is_template": row.is_template
            })
        })
        .collect();

    Ok(databases)
}
pub async fn fetch_system_database_by_name(pool: &Pool<Postgres>, db_name: &str) -> Result<Option<serde_json::Value>, Error> {
    // List of system databases that should not be fetched
    let system_databases = vec![];

    // Check if the requested database is in the list of system databases
    if system_databases.contains(&db_name) {
        return Ok(None); // If it's a system database, return None
    }

    // Query to fetch the database details
    let db_row = sqlx::query!(
        r#"
        SELECT
            datname AS name,
            pg_size_pretty(pg_database_size(datname)) AS size,
            pg_roles.rolname AS owner,
            datcollate AS collation,
            datctype AS ctype,
            datallowconn AS allow_connections,
            datconnlimit AS connection_limit,
            datistemplate AS is_template
        FROM
            pg_database
        JOIN
            pg_roles ON pg_database.datdba = pg_roles.oid
        WHERE
            datname = $1
        "#,
        db_name
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = db_row {
        // Query to fetch tables in the database
        let table_rows = sqlx::query!(
            r#"
            SELECT
                tablename
            FROM
                pg_catalog.pg_tables
            WHERE
                schemaname = 'public'
            "#,
        )
        .fetch_all(pool)
        .await?;

        let tables: Vec<String> = table_rows
            .into_iter()
            .filter_map(|row| row.tablename) // Handle Option
            .collect();

        // Construct JSON result
        let result = serde_json::json!({
            "name": record.name,
            "size": record.size,
            "owner": record.owner,
            "collation": record.collation,
            "ctype": record.ctype,
            "allow_connections": record.allow_connections,
            "connection_limit": record.connection_limit,
            "is_template": record.is_template,
            "tables": tables
        });

        return Ok(Some(result));
    }

    Ok(None)
}
pub async fn rename_database(pool: &sqlx::Pool<sqlx::Postgres>, old_name: &str, new_name: &str) -> Result<(), sqlx::Error> {
    // List of system databases that should not be renamed
    let system_databases = vec!["postgres", "template1", "template0"];

    // Get the current database name
    let current_db_name = get_database_name(pool).await?;

    // Check if the new name already exists or is a system database
    if system_databases.contains(&new_name) {
        return Err(sqlx::Error::ColumnNotFound("New name conflicts with a system database".into()));
    }

    // Check if the old name is the same as the current database or if it's a system database
    if system_databases.contains(&old_name) || old_name == current_db_name.as_str() {
        return Err(sqlx::Error::ColumnNotFound("Cannot rename system or current database".into()));
    }

    // Execute the rename query
    let query = format!("ALTER DATABASE {} RENAME TO {}", old_name, new_name);
    sqlx::query(&query)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn set_database_owner(pool: &sqlx::Pool<sqlx::Postgres>, db_name: &str, new_owner: &str) -> Result<(), sqlx::Error> {
    // List of system databases that should not be modified
    let system_databases = vec!["postgres", "template1", "template0"];

    // Get the current database name
    let current_db_name = get_database_name(pool).await?;

    if system_databases.contains(&db_name) || db_name == current_db_name.as_str() {
        return Err(sqlx::Error::ColumnNotFound("Cannot change owner of system or current database".into()));
    }

    // Execute the set owner query
    let query = format!("ALTER DATABASE {} OWNER TO {}", db_name, new_owner);
    sqlx::query(&query)
        .execute(pool)
        .await?;

    Ok(())
}

// pub async fn database_exists(pool: &sqlx::Pool<sqlx::Postgres>, db_name: &str) -> Result<bool, sqlx::Error> {
//     // List of system databases that should not be checked
//     let system_databases = vec!["postgres", "template1", "template0"];

//     // Get the current database name
//     let current_db_name = get_database_name(pool).await?;

//     // If the database name is a system or current database, return false
//     if system_databases.contains(&db_name) {
//         return Ok(false);
//     }

//     // Query to check if the database exists
//     let row = sqlx::query!("SELECT 1 FROM pg_database WHERE datname = $1", db_name)
//         .fetch_optional(pool)
//         .await?;

//     Ok(row.is_some())
// }

// pub async fn create_user(pool: &sqlx::Pool<sqlx::Postgres>, username: &str, password: &str) -> Result<(), sqlx::Error> {
//     let query = format!("CREATE USER {} WITH PASSWORD '{}'", username, password);
//     sqlx::query(&query)
//         .execute(pool)
//         .await?;

//     Ok(())
// }

// pub async fn grant_privileges(pool: &sqlx::Pool<sqlx::Postgres>, db_name: &str, username: &str, privileges: &str) -> Result<(), sqlx::Error> {
//     // List of system databases that should not be modified
//     let system_databases = vec!["postgres", "template1", "template0"];

//     // Get the current database name
//     let current_db_name = get_database_name(pool).await?;

//     if system_databases.contains(&db_name) || db_name == current_db_name.as_str() {
//         return Err(sqlx::Error::ColumnNotFound("Cannot grant privileges on system or current database".into()));
//     }

//     // Execute the grant privileges query
//     let query = format!("GRANT {} ON DATABASE {} TO {}", privileges, db_name, username);
//     sqlx::query(&query)
//         .execute(pool)
//         .await?;

//     Ok(())
// }

pub async fn revoke_privileges(pool: &sqlx::Pool<sqlx::Postgres>, db_name: &str, username: &str, privileges: &str) -> Result<(), sqlx::Error> {
    // List of system databases that should not be modified
    let system_databases = vec!["postgres", "template1", "template0"];

    // Get the current database name
    let current_db_name = get_database_name(pool).await?;

    if system_databases.contains(&db_name) || db_name == current_db_name.as_str() {
        return Err(sqlx::Error::ColumnNotFound("Cannot revoke privileges on system or current database".into()));
    }

    // Execute the revoke privileges query
    let query = format!("REVOKE {} ON DATABASE {} FROM {}", privileges, db_name, username);
    sqlx::query(&query)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn fetch_all_privileges(pool: &sqlx::Pool<sqlx::Postgres>, db_name: &str) -> Result<Vec<(String, String, String)>, sqlx::Error> {
    // List of system databases that should not be modified
    let system_databases = vec!["postgres", "template1", "template0"];

    if system_databases.contains(&db_name) {
        return Err(sqlx::Error::ColumnNotFound("Cannot list priviledges on system".into()));
    }

    // Query to fetch privileges from the information_schema.role_table_grants
    let rows = sqlx::query!(
        r#"
        SELECT grantee, table_schema, privilege_type
        FROM information_schema.role_table_grants
        WHERE table_catalog = $1
        "#,
        db_name
    )
    .fetch_all(pool)
    .await?;

    // Map the result into a vector of tuples containing the grantee, schema, and privilege type
    let privileges: Vec<(String, String, String)> = rows
        .into_iter()
        .map(|row| (row.grantee.unwrap(), row.table_schema.unwrap(), row.privilege_type.unwrap()))
        .collect();
    

    Ok(privileges)
}