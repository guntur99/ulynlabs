// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error; // Crate yang bagus untuk error handling

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Authorization failed: {0}")]
    AuthorizationError(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error), // Konversi otomatis dari sqlx::Error

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error), // Konversi dari jsonwebtoken error

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_type = format!("{:?}", self)
            .split('(')
            .collect::<Vec<&str>>()[0]
            .to_string(); // Dapatkan nama enum

        let (status, error_message) = match self {
            AppError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::AuthorizationError(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFoundError(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::DatabaseError(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "A database error occurred.".to_string())
            }
            AppError::ExternalServiceError(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::JwtError(ref e) => {
                tracing::error!("JWT error: {:?}", e);
                (StatusCode::UNAUTHORIZED, "Invalid or expired token.".to_string())
            }
             AppError::ReqwestError(ref e) => {
                tracing::error!("Reqwest error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error communicating with external service.".to_string())
            }
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred.".to_string(),
            ),
        };

        let body = Json(json!({
            "error": {
                "type": error_type,
                "message": error_message,
            }
        }));

        (status, body).into_response()
    }
}

// Opsional: Alias untuk Result yang menggunakan AppError
pub type AppResult<T> = Result<T, AppError>;