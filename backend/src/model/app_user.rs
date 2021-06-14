use crate::model::user::User;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AppUser {
    id: i32,
    name: String,
}

impl AppUser {
    pub fn from_user(user: &User) -> Self {
        AppUser {
            id: user.id,
            name: user.name.to_string(),
        }
    }
}
