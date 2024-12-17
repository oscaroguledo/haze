

// // src/routes.rs
// use std::time::Instant;
// use actix::*;
// use actix_files::NamedFile;
// use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
// use actix_web_actors::ws;
// use diesel::{
//     prelude::*,
//     r2d2::{self, ConnectionManager},
// };
// use serde_json::json;
// use uuid::Uuid;
// use crate::db;
// use crate::models;
// use crate::server;
// use crate::session;

// pub async fn chat_server(
//     req: HttpRequest,
//     stream: web::Payload,
//     pool: web::Data<DbPool>,
//     srv: web::Data<Addr<server::ChatServer>>,
// ) -> Result<HttpResponse, Error> {
//     ws::start(
//         session::WsChatSession {
//             id: 0,
//             hb: Instant::now(),
//             room: "main".to_string(),
//             name: None,
//             addr: srv.get_ref().clone(),
//             db_pool: pool,
//         },
//         &req,
//         stream
//     )
// }