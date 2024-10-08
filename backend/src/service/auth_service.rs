use crate::{
    model::{app_user::AppUser, login_data::LoginData, session::Session},
    service::user_service::UserService,
};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::SystemTime,
};

pub struct LoginService {
    pub user_service: Arc<Mutex<dyn UserService + Sync + Send>>,
    pub sessions: HashMap<String, Session>,
}

impl LoginService {
    pub fn login_user(&mut self, login_data: &LoginData) -> Option<Session> {
        let user_and_role = self
            .user_service
            .lock()
            .unwrap()
            .get_user_and_role(&login_data.name);
        match user_and_role {
            Ok((db_user, role)) => {
                let stored_hash = &db_user.pwd;
                if is_password_valid(&login_data.pwd, stored_hash) {
                    let app_user = AppUser::from((db_user, role));
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

    pub fn remove_user_session(&mut self, user_id: i32) -> Result<(), &'static str> {
        println!("Removing Session for user with id: {user_id}");
        let (session_id, session_data) = self
            .sessions
            .iter()
            .find(|(_, sess)| sess.user.id == user_id)
            .ok_or("Session was not removed")?;
        println!(
            "Session_id has been removed for user: {:?}",
            session_data.user
        );
        let session_id = session_id.clone();
        if self.sessions.remove(&session_id).is_some() {
            Ok(())
        } else {
            Err("No session to remove")
        }
    }
}

fn is_password_valid(input_pwd: &str, stored_hash: &str) -> bool {
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
