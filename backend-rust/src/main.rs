use actix_web::{App, HttpServer};
use backend_rust::InMemoryStore;
use log::info;

mod handler;
mod entity;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting http server: 8080");

    HttpServer::new(|| {
        App::new()
            .service(handler::todo::index)
            .service(handler::todo::hello_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
