use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use log::{info};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Any(#[from] anyhow::Error),

    #[error("internal server error")]
    JsonError(#[from] serde_json::Error),

    #[error("redis error")]
    RedisError(#[from] redis::RedisError),

    #[error("http error")]
    HttpError(#[from] reqwest::Error),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let http_status = StatusCode::OK;

        let mut message: Option<String> = None;

        let code = match self {
            AppError::JsonError(_e) => 1,
            AppError::RedisError(e) =>  {
                info!("Redis error: {}", e);
                message = Some(e.to_string());
                2
            },
            AppError::Any(_e) => 3,
            AppError::HttpError(_e) => 4,
        };

        let body = serde_json::json!({
            "status": "error",
            "code": code,
            "message": message,
        });
        (http_status, Json(body)).into_response()
    }
}