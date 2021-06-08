use crate::model::user::User;
use crate::schema::sessions;
use diesel::{Associations, Identifiable, Queryable};
use serde::Deserialize;

#[derive(Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "sessions"]
pub struct Session {
    id: i32,
    name: String,
    value: String,
    user_id: i32,
}

impl Session {
    pub fn new(use_id: i32) -> Self {
        todo!("actually create Session")
    }
}
