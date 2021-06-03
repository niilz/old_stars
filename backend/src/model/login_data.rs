use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginData {
    pub user_name: String,
    pub pwd: String,
}
