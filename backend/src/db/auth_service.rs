use crate::model::{login_data::LoginData, session::Session, user::User};
use crate::schema::old_users::dsl::*;
use diesel::{prelude::*, PgConnection};

pub fn check_pwd(conn: &PgConnection, login_data: LoginData) -> bool {
    let users: Vec<User> = old_users
        .filter(name.eq(&login_data.name))
        .limit(1)
        .load(conn)
        .expect(&format!("Error checking pwd for user: {}", login_data.name));
    users.get(0).unwrap().pwd == login_data.pwd
}

pub fn insert_session(conn: &PgConnection, user_id: i32) {
    let session = Session::new(user_id);
    todo!("store session");
}

pub fn get_session(conn: &PgConnection, user_id: i32) {
    todo!("Retrieve Session by use_id from db");
}
