use crate::{
    model::{app_user::AppUser, login_data::LoginData, session::Session},
    service::user_service::UserService,
};
use argon2::{
    password_hash::{self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use std::{collections::HashMap, sync::Arc, time::SystemTime};

pub struct LoginService {
    pub user_service: Arc<dyn UserService + Sync + Send>,
    pub sessions: HashMap<String, Session>,
}

impl LoginService {
    pub fn login_user(&mut self, login_data: LoginData) -> Option<Session> {
        let user = self.user_service.get_user_by_name(&login_data.name);
        match user {
            Ok(db_user) => {
                let stored_hash = &db_user.pwd;
                if is_password_valid(login_data.pwd, stored_hash) {
                    let app_user = AppUser::from_user(&db_user);
                    let session = Session::new(app_user);
                    self.sessions
                        .insert(session.uuid.to_string(), session.clone());
                    Some(session)
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
    pub fn get_session_user(&self, session_id: &str) -> Option<AppUser> {
        match self.sessions.get(session_id) {
            Some(session) if session.exp > SystemTime::now() => Some(session.user.clone()),
            Some(_) => {
                println!("Session expired, login required");
                None
            }
            None => {
                println!("No session found, login required");
                None
            }
        }
    }

    pub fn remove_session(&mut self, session_id: &str) -> Result<(), &'static str> {
        if self.sessions.remove(session_id).is_some() {
            Ok(())
        } else {
            Err("No session to remove")
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
