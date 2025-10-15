use garde::Validate;
use log::info;
use serde::{Deserialize, Serialize};
use shared::requests::RegisterRequest;
use crate::app_error::AppError;
//use sled;
use crate::config::Config;
use bincode::{Encode, Decode};
//use bincode::config::Configuration;

#[derive(Clone)]
pub struct Storage {
    db: sled::Db,
    binconfig: bincode::config::Configuration,
}

#[derive(Serialize, Debug, Deserialize, Encode, Decode)]
pub struct User {
    pub password: String,
}

impl Storage {
    pub fn new(config: &Config) -> Result<Self, AppError> {
        let path = config.storage.path.to_path(config)?;
        info!("Storage path: {}", &path.to_string_lossy());
        Ok(Self {
            db: sled::open(path)?,
            binconfig: bincode::config::standard()
        })
    }

    pub fn find_user(&self, username: &str) -> Result<User, AppError> {
        if let Some(bytes) = self.db.get(username)? {
            let (user, _len): (User, usize) =
                bincode::decode_from_slice(bytes.as_ref(), self.binconfig)?;
            return Ok(user);
        }
        Err(AppError::NotFound)
    }

    pub fn register_user(&mut self, request: &RegisterRequest) -> Result<(), AppError> {
        request.validate()?;

        if self.find_user(&request.username).is_ok() {
            return Err(AppError::PermissionDenied("User already exists".into()));
        }

        let user = User { password: request.password.clone() };
        let bytes: Vec<u8> = bincode::encode_to_vec(&user, self.binconfig)?;
        self.db.insert(request.username.clone(), bytes)?;
        Ok(())
    }
}


