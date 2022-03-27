use serde::Serialize;

use self::error::CustomError;

pub mod todo;
pub mod error;

#[derive(Debug, Serialize)]
pub struct CustomResponse<T> {
    pub body: T,
    pub error: Option<CustomError>,
}

impl<T> CustomResponse<T> {
    pub fn ok(body: T) -> Self {
        CustomResponse {
            body,
            error: None,
        }
    }
}
