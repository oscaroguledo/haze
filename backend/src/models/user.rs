use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{NaiveDateTime};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub lastseen: Option<bool>, // Optional, since this field can be NULL
    pub created_at: NaiveDateTime,  // NaiveDateTime will now work with Serde
}
