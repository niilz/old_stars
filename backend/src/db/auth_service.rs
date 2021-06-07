use crate::model::{login_data::LoginData, user::User};
use crate::schema::user::dsl::*;
use diesel::{prelude::*, PgConnection};

pub fn check_pwd(conn: &PgConnection, login_data: LoginData) -> String {
    let users: Vec<User> = user
        .filter(name.eq(&login_data.user_name))
        .limit(1)
        .load(conn)
        .expect(&format!(
            "Error checking pwd for user: {}",
            login_data.user_name
        ));
    users.get(0).unwrap().pwd.to_string()
}
