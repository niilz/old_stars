use crate::schema::old_users::dsl::*;
use crate::{
    model::{app_user::AppUser, login_data::LoginData, session::Session},
    UserService,
};
use argon2::{
    password_hash::{self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;
use rand_core::OsRng;
use std::sync::Arc;

pub struct LoginService {
    pub user_service: Arc<dyn UserService + Sync + Send>,
}

impl LoginService {
    pub fn login_user(&self, login_data: LoginData) -> Option<AppUser> {
        let user = self.user_service.get_user_by_name(&login_data.name);
        match user {
            Ok(db_user) => {
                let stored_hash = &db_user.pwd;
                if is_password_valid(login_data.pwd, stored_hash) {
                    Some(AppUser::from_user(&db_user))
                } else {
                    None
                }
            }
            Err(e) => {
                eprintln!("Could not login user: {}, Err: {}", login_data.name, e);
                None
            }
        }
    }
}

pub fn insert_session(_conn: &PgConnection, user_name: &str) {
    let _session = Session::new(user_name);
    todo!("store session");
}

pub fn get_session(_conn: &PgConnection, _user_name: &str) {
    todo!("Retrieve Session by use_id from db");
}

pub fn hash(user_pwd: &str) -> Result<String, password_hash::Error> {
    let rnd_salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let pwd_hash = argon.hash_password_simple(user_pwd.as_bytes(), &rnd_salt)?;
    Ok(pwd_hash.to_string())
}

fn is_password_valid(input_pwd: String, stored_hash: &str) -> bool {
    let argon = Argon2::default();
    let parsed_stored_pwd = PasswordHash::new(stored_hash);
    match parsed_stored_pwd {
        Err(e) => {
            eprintln!("Could not parse the stored hash. Error: {}", e);
            false
        }
        Ok(parsed_hash) => argon
            .verify_password(input_pwd.as_bytes(), &parsed_hash)
            .is_ok(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_hash() {
        let hash = hash("MySecretPwd");
        assert!(hash.is_ok());
    }

    #[test]
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
