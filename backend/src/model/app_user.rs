use crate::model::user::User;
use serde::Serialize;

use super::role::OldStarsRole;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppUser {
    pub id: i32,
    pub name: String,
    pub role: OldStarsRole,
    #[serde(rename = "beerCount")]
    pub beer_count: i32,
    #[serde(rename = "shotCount")]
    pub shot_count: i32,
    #[serde(rename = "otherCount")]
    pub other_count: i32,
    #[serde(rename = "waterCount")]
    pub water_count: i32,
}

impl From<(User, OldStarsRole)> for AppUser {
    fn from(user_and_role: (User, OldStarsRole)) -> Self {
        let user = user_and_role.0;
        let role = user_and_role.1;
        Self {
            id: user.user_id,
            role,
            name: user.name.to_string(),
            beer_count: user.beer_count,
            shot_count: user.shot_count,
            other_count: user.other_count,
            water_count: user.water_count,
        }
    }
}
