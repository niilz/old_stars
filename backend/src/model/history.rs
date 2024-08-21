use diesel::prelude::*;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::schema::history;

use super::user::User;

#[derive(Deserialize, Serialize, Queryable, Selectable, Eq, PartialEq, Debug, Clone)]
#[diesel(table_name = history)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(history_id))]
pub struct History {
    #[serde(rename = "historyId")]
    pub history_id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub timestamp: SystemTime,
    #[serde(rename = "beerCount")]
    pub beer_count: i32,
    #[serde(rename = "shotCount")]
    pub shot_count: i32,
    #[serde(rename = "otherCount")]
    pub other_count: i32,
    #[serde(rename = "waterCount")]
    pub water_count: i32,
}

#[derive(Insertable, Eq, PartialEq, Debug, Clone)]
#[table_name = "history"]
pub struct InsertHistory {
    pub user_id: i32,
    pub timestamp: SystemTime,
    pub beer_count: i32,
    pub shot_count: i32,
    pub other_count: i32,
    pub water_count: i32,
}

impl From<&User> for InsertHistory {
    fn from(user: &User) -> Self {
        Self {
            user_id: user.user_id,
            beer_count: user.beer_count,
            shot_count: user.shot_count,
            other_count: user.other_count,
            water_count: user.water_count,
            timestamp: SystemTime::now(),
        }
    }
}

impl From<(&i32, &InsertHistory)> for History {
    fn from(entry: (&i32, &InsertHistory)) -> Self {
        let (id, history) = entry;
        Self {
            history_id: *id,
            user_id: history.user_id,
            timestamp: history.timestamp,
            beer_count: history.beer_count,
            shot_count: history.shot_count,
            other_count: history.other_count,
            water_count: history.water_count,
        }
    }
}

impl Default for History {
    fn default() -> Self {
        Self {
            history_id: Default::default(),
            user_id: Default::default(),
            timestamp: SystemTime::now(),
            beer_count: Default::default(),
            shot_count: Default::default(),
            other_count: Default::default(),
            water_count: Default::default(),
        }
    }
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }
}
