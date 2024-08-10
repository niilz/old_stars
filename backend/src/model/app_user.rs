use crate::model::user::User;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppUser {
    pub id: i32,
    pub name: String,
    #[serde(rename = "beerCount")]
    pub beer_count: Option<i32>,
    #[serde(rename = "shotCount")]
    pub shot_count: Option<i32>,
    #[serde(rename = "waterCount")]
    pub water_count: Option<i32>,
}

impl AppUser {
    pub fn from_user(user: &User) -> Self {
        AppUser {
            id: user.id,
            name: user.name.to_string(),
            beer_count: user.beer_count,
            shot_count: user.shot_count,
            water_count: user.water_count,
        }
    }
}
