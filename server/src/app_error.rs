use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use log::{info};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Any(#[from] anyhow::Error),

    #[error("internal server error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("http error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("invalid integer: {0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),

    #[error("generate error: {0}")]
    GenerateError(String),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let http_status = StatusCode::OK;
        let uuid = Uuid::new_v4();
        let message: String = self.to_string();

        info!("error occurred {}: {}", uuid, message);

        let (http_status, code) = match &self {
            AppError::JsonError(_)        => (StatusCode::INTERNAL_SERVER_ERROR, 1),
            AppError::RedisError(_)       => (StatusCode::BAD_GATEWAY,             2),
            AppError::Any(_)              => (StatusCode::INTERNAL_SERVER_ERROR,   3),
            AppError::HttpError(e) if e.status().map(|s| s.is_client_error()).unwrap_or(false) => (StatusCode::BAD_REQUEST, 4),
            AppError::HttpError(_)        => (StatusCode::BAD_GATEWAY,             4),
            AppError::TryFromIntError(_)  => (StatusCode::BAD_REQUEST,             5),
            AppError::GenerateError(_)    => (StatusCode::BAD_REQUEST,             6),
            AppError::IOError(e) if e.kind() == std::io::ErrorKind::NotFound  => (StatusCode::NOT_FOUND, 7),
            AppError::IOError(_)          => (StatusCode::INTERNAL_SERVER_ERROR,   7),
        };

        let body = serde_json::json!({
            "status": "error",
            "code": code,
            "uuid": uuid.to_string(),
        });
        (http_status, Json(body)).into_response()
    }
}