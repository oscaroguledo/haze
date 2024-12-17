// use actix_web::{HttpRequest, HttpResponse};
// use actix_web_actors::ws;
// use actix::prelude::*;

// // Define the ChatSession actor
// struct ChatSession;

// impl ChatSession {
//     pub fn new() -> Self {
//         Self
//     }
// }

// // Implement the Actor trait for ChatSession
// impl ws::Actor for ChatSession {
//     type Context = ws::WebsocketContext<Self>;

//     fn started(&mut self, ctx: &mut Self::Context) {
//         ctx.text("Welcome to the chat!");
//     }

//     fn stopped(&mut self, _ctx: &mut Self::Context) {
//         println!("WebSocket disconnected");
//     }
// }

// // Implement StreamHandler for ChatSession to handle incoming messages
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Text(text)) => {
//                 // Handle incoming text message
//                 ctx.text(format!("Echo: {}", text));
//             }
//             Ok(ws::Message::Binary(bin)) => {
//                 // Handle incoming binary message
//                 ctx.binary(bin);
//             }
//             Err(e) => {
//                 // Handle protocol error
//                 println!("Error: {:?}", e);
//                 ctx.stop(); // Close the WebSocket connection if an error occurs
//             }
//             _ => {}
//         }
//     }
// }

// // WebSocket handler function to start the session
// pub async fn start_ws(req: HttpRequest, stream: actix_web::web::Payload) -> Result<HttpResponse, actix_web::Error> {
//     ws::start(ChatSession::new(), &req, stream)
//         .map_err(|_| HttpResponse::InternalServerError().finish().into())
// }
