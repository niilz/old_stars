use crate::model::{app_user::AppUser, login_data::LoginData, session::Session, user::User};
use crate::schema::old_users::dsl::*;
use argon2::{
    password_hash::{self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::{prelude::*, PgConnection};
use rand_core::OsRng;

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

            let hash_to_check = hash(&login_data.pwd);

            /*
            if stored_hash == &hash_to_check {
                Some(AppUser::from_user(db_user))
            } else {
                None
            }
            */
            Some(AppUser::from_user(db_user))
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

pub fn hash(user_pwd: &str) -> Result<String, password_hash::Error> {
    let rnd_salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let pwd_hash = argon.hash_password_simple(user_pwd.as_bytes(), &rnd_salt)?;
    Ok(pwd_hash.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    fn can_create_hash() {
        let hash = hash("MySecretPwd");
        assert!(hash.is_ok());
    }

    fn can_verify_pwd_with_hash() {
        let plain_pwd = "EvenMoreSecure";
        let pwd_hashed = hash(plain_pwd).unwrap();
        let pwd_parsed = PasswordHash::new(&pwd_hashed).unwrap();
        let argon = Argon2::default();
        assert!(argon
            .verify_password(plain_pwd.as_bytes(), &pwd_parsed)
            .is_ok());
    }
}
