use actix_web::{get, post, HttpResponse, Responder};
use log::info;

#[get("/api/v1/todos")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/api/v1/todo")]
async fn hello_post(req_body: String) -> impl Responder {
    info!("{}", req_body);
    HttpResponse::Ok().body("Hello world!")
}
