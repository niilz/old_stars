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
