use std::fmt::Display;

use crate::model::user::User;
use crate::schema::roles;
use serde::{Deserialize, Serialize};

#[derive(
    Deserialize,
    Serialize,
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    PartialEq,
    Debug,
    Clone,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = roles)]
#[diesel(primary_key(role_id))]
pub struct Role {
    pub role_id: i32,
    pub user_id: i32,
    pub role: String,
}

#[derive(Insertable, Eq, PartialEq, Debug, Clone, Default)]
#[table_name = "roles"]
pub struct InsertRole<'a> {
    pub user_id: i32,
    pub role: &'a str,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub enum OldStarsRole {
    User,
    Admin,
}

impl TryFrom<&str> for OldStarsRole {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "admin" => Ok(Self::Admin),
            "user" => Ok(Self::User),
            v => Err(format!("Could not convert '{v}' into a role")),
        }
    }
}

impl Display for OldStarsRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Admin => write!(f, "admin"),
        }
    }
}
