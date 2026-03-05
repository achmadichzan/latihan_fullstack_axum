use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn baru(code: StatusCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            error: self.message,
        });

        (self.code, body).into_response()
    }
}