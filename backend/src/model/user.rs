use crate::schema::old_users;
use serde::{Deserialize, Serialize};

#[derive(
    Deserialize,
    Serialize,
    Queryable,
    Selectable,
    AsChangeset,
    Identifiable,
    Eq,
    PartialEq,
    Debug,
    Clone,
    Default,
)]
#[diesel(table_name = old_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub pwd: String,
    #[serde(rename = "beerCount")]
    pub beer_count: i32,
    #[serde(rename = "shotCount")]
    pub shot_count: i32,
    #[serde(rename = "waterCount")]
    pub water_count: i32,
    pub fk_icon_id: i32,
}

#[derive(Insertable, Eq, PartialEq, Debug, Clone, Default)]
#[table_name = "old_users"]
pub struct InsertUser<'a> {
    pub name: &'a str,
    pub pwd: &'a str,
    pub beer_count: i32,
    pub shot_count: i32,
    pub water_count: i32,
    pub fk_icon_id: i32,
}

impl<'a> InsertUser<'a> {
    pub fn new(name: &'a str, pwd: &'a str) -> Self {
        Self {
            name,
            pwd,
            ..Default::default()
        }
    }
}
