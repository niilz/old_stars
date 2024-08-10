use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(
    Deserialize, Serialize, Queryable, Selectable, Identifiable, Eq, PartialEq, Debug, Clone,
)]
#[diesel(table_name = crate::schema::old_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub pwd: String,
    #[serde(rename = "beerCount")]
    pub beer_count: Option<i32>,
    #[serde(rename = "shotCount")]
    pub shot_count: Option<i32>,
    #[serde(rename = "waterCount")]
    pub water_count: Option<i32>,
    pub fk_icon_id: Option<i32>,
}
