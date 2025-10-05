use serde::{Deserialize, Serialize};
use macros::AutoJIntoResponse;

#[derive(Serialize, Debug, Deserialize)]
pub struct LoginRegisterRequest {
    pub username: String,
    pub password: String,
    pub invitation_code: Option<String>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub invitation_code: Option<String>,
}

#[derive(Serialize, Debug, Deserialize, AutoJIntoResponse)]
pub struct RegisterResponse {
    pub status: String,
    pub message: Option<String>,
}


#[derive(Serialize, Debug, Deserialize)]
pub struct ServerResponse<P>
where P: Serialize
{
    pub status: String,
    pub message: Option<String>,
    pub result: Option<P>,
}


impl<P> ServerResponse<P>
    where P: Serialize
{
    pub fn new(status: &str) -> Self {
        Self {
            status: status.to_string(),
            message: None,
            result: None,
        }
    }
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn result(mut self, result: P) -> Self {
        self.result = Some(result);
        self
    }
}




#[cfg(feature = "server")]
mod server_impl {

use axum::response::{IntoResponse, Response, Json};
use axum::http::StatusCode;
use super::*;

impl<P> IntoResponse for ServerResponse<P>
where
    P: Serialize
{
    fn into_response(self) -> Response {
        let status_code = match self.status.as_str() {
            "error" | "Error" | "ERROR" => StatusCode::BAD_REQUEST,
            _ => StatusCode::OK,
        };

        (status_code, Json(self)).into_response()
    }
}

} // mod server_impl