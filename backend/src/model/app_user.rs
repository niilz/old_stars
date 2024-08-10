use crate::model::user::User;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppUser {
    pub id: i32,
    pub name: String,
    #[serde(rename = "beerCount")]
    pub beer_count: i32,
    #[serde(rename = "shotCount")]
    pub shot_count: i32,
    #[serde(rename = "waterCount")]
    pub water_count: i32,
}

impl From<&User> for AppUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.user_id,
            name: user.name.to_string(),
            beer_count: user.beer_count,
            shot_count: user.shot_count,
            water_count: user.water_count,
        }
    }
}
