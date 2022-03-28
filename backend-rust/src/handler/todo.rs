use actix_web::{get, post, HttpResponse, Responder, web};
use backend_rust::AppState;
use crate::{handler::CustomResponse, service, entity};

#[get("/api/v1/todos")]
pub async fn get_all(app_state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(CustomResponse::ok(service::todo::get_all_todos(&app_state.store)))
}

#[post("/api/v1/todo")]
pub async fn hello_post(app_state: web::Data<AppState>, item: web::Json<entity::todo::CreateTodo>) -> impl Responder {
    service::todo::create(&app_state.store, item.0.title);
    HttpResponse::Created().json(CustomResponse::ok("Ok".to_string()))
}
