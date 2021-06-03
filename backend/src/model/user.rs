use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub salt: String,
    pub pwd_hash: String,
    pub fk_icon_id: u32,
}
