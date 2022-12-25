#[macro_use]
extern crate diesel;

pub mod db;
pub mod model;
pub mod schema;

use db::user_service::UserServiceError;
use model::{login_data::LoginData, user::User};

pub trait UserService: Send + Sync {
    fn get_user_by_name(&self, user_name: &str) -> Result<User, UserServiceError>;
    fn insert_user(&self, new_user: LoginData) -> Result<User, UserServiceError>;
    fn delete_user(&self, id: i32) -> Result<User, UserServiceError>;
    fn add_drink_to_user<'a>(
        &self,
        update_id: i32,
        drink: &'a str,
    ) -> Result<User, UserServiceError>;
}
