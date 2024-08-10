use crate::schema::roles;
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Identifiable, Eq, PartialEq, Debug, Clone)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub user_id: i32,
    pub role: OldStarsRole,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub enum OldStarsRole {
    User,
    Admin,
}
