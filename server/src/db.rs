use log::info;
use shared::requests::RegisterRequest;
use crate::app_error::AppError;
//use sled;
use crate::config::Config;

#[derive(Clone)]
pub struct Storage {
    db: sled::Db,
}


impl Storage {
    pub fn new(config: &Config) -> Result<Self, AppError> {
        let path = config.storage.path.to_path(config)?;
        info!("Storage path: {}", &path.to_string_lossy());
        Ok(Self {
            db: sled::open(path)?,
        })
    }



    pub fn register_user(&mut self, request: &RegisterRequest) {

    }
}


