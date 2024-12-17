use actix_web::web;
use crate::controllers::user::{get_users, create_user};


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_users)
        .service(create_user);
//         .service(get_user_by_id)
//         .service(get_user_by_phone)
//         .service(get_conversation_by_id)
//         .service(get_rooms)
//
}
