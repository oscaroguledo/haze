use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password:String,
    pub created_at: chrono::NaiveDateTime,
}