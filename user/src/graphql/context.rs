use std::sync::Mutex;

use actix_web::{http::header::HeaderMap, web};

#[derive(Clone)]
pub struct ContextUtil {
    headers: HeaderMap,
    socket: web::Data<
        Mutex<tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>>,
    >,
    pub sdl: String,
}

impl juniper::Context for ContextUtil {}

impl ContextUtil {
    pub fn new(
        headers: &HeaderMap,
        socket: web::Data<
            Mutex<tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>>,
        >,
        sdl: String,
    ) -> ContextUtil {
        return Self {
            headers: headers.clone(),
            socket: socket.clone(),
            sdl,
        };
    }
}
