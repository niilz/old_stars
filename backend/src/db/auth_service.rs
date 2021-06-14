use crate::model::{app_user::AppUser, login_data::LoginData, session::Session, user::User};
use crate::schema::old_users::dsl::*;
use diesel::{prelude::*, PgConnection};

pub fn login_user(conn: &PgConnection, login_data: LoginData) -> Option<AppUser> {
    let users: Vec<User> = old_users
        .filter(name.eq(&login_data.name))
        .limit(1)
        .load(conn)
        .expect(&format!("Error checking pwd for user: {}", login_data.name));

    match users.get(0) {
        Some(db_user) => {
            let stored_hash = &db_user.pwd;
            let stored_salt = &db_user.salt;
            let hash_to_check = hash(login_data.pwd, stored_salt);

            if stored_hash == &hash_to_check {
                Some(AppUser::from_user(db_user))
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn insert_session(conn: &PgConnection, user_id: i32) {
    let session = Session::new(user_id);
    todo!("store session");
}

pub fn get_session(conn: &PgConnection, user_id: i32) {
    todo!("Retrieve Session by use_id from db");
}

pub fn get_salt() -> String {
    // TODO: Use actual rand generated
    "rnd_salt_".to_string()
}
pub fn hash(user_pwd: String, rnd_salt: &str) -> String {
    // TODO: Use actual hashing lib
    let salted_pwd = format!("{}{}", rnd_salt, user_pwd);
    let hashed_pwd = format!("hashed_{}", salted_pwd);
    hashed_pwd
}
