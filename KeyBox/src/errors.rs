use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    AlreadyExists(String),
    Changed(String),
    ValidationError(String),
    Unauthorized(String),
    NotEnoughRights(),
    Internal(String),
}

impl<E: std::error::Error> From<E> for AppError {
    fn from(value: E) -> Self {
        Self::Internal(value.to_string())
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(object) => (StatusCode::NOT_FOUND, format!("{} not found", object)),
            AppError::AlreadyExists(object) => {
                (StatusCode::CONFLICT, format!("{} already exists", object))
            }
            AppError::Changed(object) => (
                StatusCode::CONFLICT,
                format!("another operation on {} in progress", object),
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                format!("validation error: {}", msg),
            ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("internal error: {}", msg),
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                format!("authorization error: {}", msg),
            ),
            AppError::NotEnoughRights() => (
                StatusCode::FORBIDDEN,
                format!("user doesn't have enough rights"),
            ),
        };

        error!(error_message = %message);

        let body = Json(ErrorResponse { message: message });

        (status, body).into_response()
    }
}
