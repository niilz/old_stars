use crate::schema::old_users;
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Identifiable, Eq, PartialEq, Debug, Clone)]
#[diesel(table_name = old_users)]
pub struct User {
    pub id: i32,
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
