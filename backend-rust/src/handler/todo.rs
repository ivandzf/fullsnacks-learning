use actix_web::{get, post, HttpResponse, Responder, web};
use backend_rust::{InMemory, InMemoryStore};
use log::info;
use crate::handler::CustomResponse;

#[get("/api/v1/todos")]
pub async fn get_all(store: web::Data<InMemory>) -> impl Responder {
    log::info!("GET /api/v1/todos");
    HttpResponse::Ok().json(CustomResponse::ok(store.get_all()))
}

#[post("/api/v1/todo")]
pub async fn hello_post(req_body: String) -> impl Responder {
    info!("{}", req_body);
    HttpResponse::Ok().body("Hello world!")
}
