use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("User not found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Internal(e) => {
                tracing::error!(error = %e, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
        };

        let body = axum::Json(json!({
            "error": message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
