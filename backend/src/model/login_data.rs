use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginData {
    pub name: String,
    pub pwd: String,
}
