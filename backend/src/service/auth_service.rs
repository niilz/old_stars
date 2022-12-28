use crate::{
    model::{app_user::AppUser, login_data::LoginData},
    service::user_service::UserService,
};
use argon2::{
    password_hash::{self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
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
