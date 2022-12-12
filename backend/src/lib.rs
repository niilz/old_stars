#[macro_use]
extern crate diesel;

pub mod db;
pub mod model;
pub mod schema;

use db::user_service::UserServiceError;
use model::user::User;

pub trait UserService {
    fn get_user_by_name(&self, user_name: &str) -> Result<User, UserServiceError>;
}
