use actix_web::web;
use crate::controllers::user_controller::{get_users, create_user};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_users)
        .service(create_user);
}