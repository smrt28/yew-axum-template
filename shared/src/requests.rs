use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct LoginRegisterRequest {
    pub username: String,
    pub password: String,
    pub invitation_code: Option<String>,
}