use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Any(#[from] anyhow::Error),

    #[error("internal server error")]
    JsonError(#[from] serde_json::Error),

    #[error("redis error")]
    RedisError(#[from] redis::RedisError),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let http_status = StatusCode::OK;

        let code = match self {
            AppError::JsonError(_e) => 1,
            AppError::RedisError(_e) => 2,
            AppError::Any(_e) => 3,
        };

        let body = serde_json::json!({
            "status": "error",
            "code": code,
        });
        (http_status, Json(body)).into_response()
    }
}