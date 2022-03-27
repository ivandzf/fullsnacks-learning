use actix_web::{App, HttpServer};
use backend_rust::{InMemory, InMemoryStore};
use log;

mod handler;
mod entity;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::info!("Starting http server: 8080");

    HttpServer::new(move || {
        let store = InMemory::new();

        App::new()
            .app_data(store)
            .service(handler::todo::get_all)
            .service(handler::todo::hello_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
