#[macro_use]
extern crate juniper;
use std::{env, sync::Mutex};

use actix_cors::Cors;
use actix_web::{
    middleware,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer,
};
use graphql::{context::ContextUtil, schema::GraphqlSchema as Schema};
use juniper_actix::graphql_handler;

mod graphql;
mod repository;
mod rest;
mod util;
mod controllers;

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
    socket: web::Data<
        Mutex<tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>>,
    >,
) -> Result<HttpResponse, Error> {
    let context = ContextUtil::new(req.headers(), socket, schema.clone().as_schema_language());

    graphql_handler(&schema, &context, req, payload).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let server = HttpServer::new(move || {
        let schema = graphql::schema::schema();
        let (socket, _response) = {
            let tmp = tungstenite::connect(
                url::Url::parse(
                    &format!(
                        "ws://{}:{}",
                        std::env::var("WS_HOST").unwrap(),
                        std::env::var("WS_PORT").unwrap()
                    )
                    .to_string(),
                )
                .unwrap(),
            )
            .expect("Can't connect");

            ((Mutex::new(tmp.0)), tmp.1)
        };

        App::new()
            .app_data(Data::new(schema))
            .app_data(Data::new(socket))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials(),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql_route)))
            .service(web::scope("/api").configure(rest::rest_scoped_config))
    });
    server.bind("0.0.0.0:8080").unwrap().run().await
}
