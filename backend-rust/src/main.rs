#[macro_use]
extern crate log;

use actix_web::{App, HttpServer, web::Data, middleware};
use backend_rust::{InMemory, InMemoryStore, AppState};

mod handler;
mod entity;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Starting http server: 8080");

    HttpServer::new(move || {
        let store = InMemory::new();
        let app_state = AppState {
            store: InMemory::new(),
        };

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(store)
            .service(handler::todo::get_all)
            .service(handler::todo::hello_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
