use actix_web::{ResponseError, HttpResponse, http::StatusCode};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum CustomErrorType {
    InternalError,
}

#[derive(Debug, Serialize)]
pub struct CustomError {
    pub message: Option<String>,
    pub err_type: CustomErrorType,
}

impl CustomError {
    pub fn message(&self) -> String {
        match &self.message {
            Some(c) => c.clone(),
            None => String::from(""),
        }
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self.err_type {
            CustomErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.message().clone())
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
