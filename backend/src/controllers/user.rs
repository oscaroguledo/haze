use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use sqlx::PgPool;
use crate::models::user::User;

#[get("/users")]
pub async fn get_users(db_pool: web::Data<PgPool>) -> impl Responder {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db_pool.get_ref())
        .await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching users"),
    }
}

#[post("/users")]
pub async fn create_user(db_pool: web::Data<PgPool>, new_user: web::Json<User>) -> impl Responder {
    let id = Uuid::new_v4();
    let query = sqlx::query!(
        "INSERT INTO users (id, name, email, created_at) VALUES ($1, $2, $3, now())",
        id,
        new_user.name,
        new_user.email
    )
    .execute(db_pool.get_ref())
    .await;

    match query {
        Ok(_) => HttpResponse::Created().body("User created"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}
