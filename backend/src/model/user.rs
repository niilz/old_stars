use crate::schema::old_users;
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Identifiable, Eq, PartialEq, Debug)]
#[table_name = "old_users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub salt: String,
    pub pwd: String,
    pub fk_icon_id: i32,
}
