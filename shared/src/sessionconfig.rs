#![allow(dead_code)]

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct SessionConfig {
    pub need_invitation: bool,
}