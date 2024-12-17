use actix_web::{HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub async fn start_ws(req: HttpRequest, stream: actix_web::web::Payload) -> HttpResponse {
    ws::start(ChatSession::new(), &req, stream)
}

struct ChatSession;

impl ChatSession {
    pub fn new() -> Self {
        Self
    }
}

impl ws::Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text("Welcome to the chat!");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket disconnected");
    }
}
