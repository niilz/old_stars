use diesel::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::schema::history;
use crate::service::error::OldStarsServiceError;

use super::user::User;

#[derive(Deserialize, Serialize, Queryable, Selectable, Eq, PartialEq, Debug, Clone)]
#[diesel(table_name = history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(history_id))]
pub struct History {
    #[serde(rename = "historyId")]
    pub history_id: i32,
    #[serde(rename = "userName")]
    pub user_name: String,
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
    pub user_name: String,
    pub timestamp: SystemTime,
    pub beer_count: i32,
    pub shot_count: i32,
    pub other_count: i32,
    pub water_count: i32,
}

impl From<(SystemTime, &User)> for InsertHistory {
    fn from(history_data: (SystemTime, &User)) -> Self {
        let (timestamp, user) = history_data;
        Self {
            user_name: user.name.to_string(),
            beer_count: user.beer_count,
            shot_count: user.shot_count,
            other_count: user.other_count,
            water_count: user.water_count,
            timestamp,
        }
    }
}

impl From<(&i32, &InsertHistory)> for History {
    fn from(entry: (&i32, &InsertHistory)) -> Self {
        let (id, history) = entry;
        Self {
            history_id: *id,
            user_name: history.user_name.to_string(),
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
            user_name: Default::default(),
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

const HISTORY_FIELD_COUNT: usize = 7;

impl TryFrom<&str> for InsertHistory {
    type Error = OldStarsServiceError;

    fn try_from(csv_tuple: &str) -> Result<Self, Self::Error> {
        let context = "TryFrom CSV for History";
        let values = csv_tuple.split(',').collect::<Vec<_>>();
        if values.len() != HISTORY_FIELD_COUNT {
            return Err(OldStarsServiceError::new(
                context,
                &format!("csv does not have {HISTORY_FIELD_COUNT} fields"),
            ));
        }
        Ok(InsertHistory {
            user_name: values[1].to_string(),
            timestamp: UNIX_EPOCH + Duration::from_millis(parse_number_or_err(values[2])?),
            beer_count: parse_number_or_err(values[3])? as i32,
            shot_count: parse_number_or_err(values[4])? as i32,
            other_count: parse_number_or_err(values[5])? as i32,
            water_count: parse_number_or_err(values[6])? as i32,
        })
    }
}
fn parse_number_or_err(s: &str) -> Result<u64, OldStarsServiceError> {
    match u64::from_str_radix(s, 10) {
        Ok(num) => Ok(num),
        Err(e) => {
            eprint!("Parse-Error: {e}");
            Err(OldStarsServiceError::new(
                "Parse-History-Number",
                &format!("Could not parse {s} as timestamp"),
            ))
        }
    }
}
