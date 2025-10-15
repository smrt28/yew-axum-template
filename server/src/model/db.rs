use garde::Validate;
use log::info;
use serde::{Deserialize, Serialize};
use shared::requests::RegisterRequest;
use crate::app_error::AppError;
use crate::config::Config;
use bincode::{Encode, Decode};
use crate::model::repositories::UserRepository;

#[derive(Clone)]
pub struct Storage {
    db: sled::Db,
    binconfig: bincode::config::Configuration,
    users: UserRepository,
}

#[derive(Serialize, Debug, Deserialize, Encode, Decode)]
pub struct User {
    pub password: String,
}

impl Storage {
    pub fn new(config: &Config) -> Result<Self, AppError> {
        let path = config.storage.path.to_path(config)?;
        info!("Storage path: {}", &path.to_string_lossy());
        let db = sled::open(path)?;
        Ok(Self {
            db: db.clone(),
            binconfig: bincode::config::standard(),
            users: UserRepository::new(db.open_tree("users")?),
        })
    }

    fn users(&self) -> Result<UserRepository, AppError> {
        Ok(UserRepository::new(self.db.open_tree("users")?))
    }
}


