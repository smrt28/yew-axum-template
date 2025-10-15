use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Encode, Decode)]
pub struct User {
    pub password: String,
}
