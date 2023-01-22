use model::app_user::AppUser;
use serde::Serialize;

#[macro_use]
extern crate diesel;

pub mod model;
pub mod repository;
pub mod schema;
pub mod service;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SessionResponse {
    user: AppUser,
    #[serde(rename = "sessionId")]
    session_id: String,
}

impl SessionResponse {
    pub fn new(user: AppUser, session_id: String) -> Self {
        Self { user, session_id }
    }
}
