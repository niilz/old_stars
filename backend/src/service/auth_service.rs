use crate::{
    model::{
        login_data::LoginData,
        role::OldStarsRole,
        session::{Session, SessionUser},
        user::User,
    },
    service::user_service::UserService,
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use super::error::OldStarsServiceError;

pub struct LoginService {
    pub user_service: Arc<Mutex<dyn UserService + Sync + Send>>,
    pub sessions: HashMap<String, Session>,
    pub club_sessions: HashMap<String, Session>,
}

impl LoginService {
    pub fn login_club(&mut self, password: &str) -> Option<Session> {
        let club_user = self.user_service.lock().unwrap().get_user_and_role("club");
        let session = issue_session(club_user, password)?;
        self.club_sessions
            .insert(session.uuid.to_string(), session.clone());
        Some(session)
    }

    pub fn login_user(&mut self, login_data: &LoginData) -> Option<Session> {
        let user_and_role = self
            .user_service
            .lock()
            .unwrap()
            .get_user_and_role(&login_data.name);
        let session = issue_session(user_and_role, &login_data.pwd)?;
        self.sessions
            .insert(session.uuid.to_string(), session.clone());
        Some(session)
    }

    pub fn has_club_access(&self, session_id: &str) -> bool {
        match self.club_sessions.get(session_id) {
            Some(session) if session.exp > SystemTime::now() => true,
            Some(_) => {
                println!("Club Session expired, login required");
                false
            }
            None => {
                println!("No club session found, login required");
                false
            }
        }
    }

    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        match self.sessions.get(session_id) {
            Some(session) if session.exp > SystemTime::now() => Some(session.clone()),
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
            session_data.user.name
        );
        let session_id = session_id.clone();
        if self.sessions.remove(&session_id).is_some() {
            Ok(())
        } else {
            Err("No session to remove")
        }
    }

    pub fn is_admin(&self, session_id: &str) -> bool {
        match self.sessions.get(session_id) {
            Some(session) if session.exp > SystemTime::now() => {
                session.user.role == OldStarsRole::Admin
            }
            Some(_) => {
                println!("Session expired, login required");
                false
            }
            None => {
                println!("No session found, login required");
                false
            }
        }
    }
}

fn issue_session(
    user_and_role: Result<(User, OldStarsRole), OldStarsServiceError>,
    login_password: &str,
) -> Option<Session> {
    match user_and_role {
        Ok((db_user, role)) => {
            let stored_hash = &db_user.pwd;
            if is_password_valid(&login_password, stored_hash) {
                let session_user = SessionUser {
                    name: db_user.name,
                    id: db_user.user_id,
                    role,
                };
                let session = Session::new(session_user);
                Some(session)
            } else {
                None
            }
        }
        Err(e) => {
            eprintln!("Login failed, Err: {}", e);
            None
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
