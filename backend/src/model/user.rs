use crate::schema::old_users;
use diesel::{Identifiable, Queryable};
use serde::Deserialize;

#[derive(Deserialize, Queryable, Identifiable)]
#[table_name = "old_users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub salt: String,
    pub pwd: String,
    pub fk_icon_id: i32,
}
