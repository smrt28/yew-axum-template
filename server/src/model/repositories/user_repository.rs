use sled::Tree;
use bincode;
use shared::requests::RegisterRequest;
use crate::app_error::AppError;
use garde::Validate;
use crate::model::user::User;

//use crate::model::repositories::UserRepository;
#[derive(Clone)]
pub struct UserRepository {
    tree: Tree,  // Changed from Db to Tree
    binconfig: bincode::config::Configuration,
}

impl UserRepository {
    pub fn new(tree: Tree) -> Self {
        Self {
            tree,
            binconfig: bincode::config::standard(),
        }
    }

    pub fn find_user(&self, username: &str) -> Result<User, AppError> {
        if let Some(bytes) = self.tree.get(username)? {
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

        let user = crate::model::db::User { password: request.password.clone() };
        let bytes: Vec<u8> = bincode::encode_to_vec(&user, self.binconfig)?;
        self.tree.insert(request.username.clone(), bytes)?;
        Ok(())
    }
}