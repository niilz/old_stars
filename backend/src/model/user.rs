use diesel::Queryable;
use serde::Deserialize;

#[derive(Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub salt: String,
    pub pwd: String,
    pub fk_icon_id: i32,
}
