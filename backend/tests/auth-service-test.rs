use argon2::{Argon2, PasswordHash, PasswordVerifier};
use backend::{
    model::{app_user::AppUser, login_data::LoginData, session::Session},
    service::auth_service::{hash, LoginService},
};
use std::{collections::HashMap, sync::Arc, time::Instant};
use uuid::Uuid;

mod mocks;

use mocks::user_service::UserServiceMock;

#[test]
fn gets_user_if_login_succeeds() {
    let dummy_user = "dummy-user";
    let login_service = LoginService {
        user_service: Arc::new(UserServiceMock::new(dummy_user)),
        sessions: HashMap::new(),
    };
    let user_that_logs_in = LoginData {
        name: dummy_user.to_string(),
        pwd: "hashed-pwd".to_string(),
    };
    let found_user = login_service.login_user(user_that_logs_in).unwrap();
    assert_eq!(found_user.id, 1);
    assert_eq!(found_user.name, dummy_user);
    assert_eq!(found_user.beer_count, 0);
    assert_eq!(found_user.water_count, 0);
    assert_eq!(found_user.shot_count, 0);
}

#[test]
fn gets_none_if_user_does_not_exist() {
    let existing_user = "dummy-user";
    let non_existing = "non-existing-user}";
    let login_service = LoginService {
        user_service: Arc::new(UserServiceMock::new(existing_user)),
        sessions: HashMap::new(),
    };
    let user_that_tries_logging_in = LoginData {
        name: non_existing.to_string(),
        pwd: "hashed_pwd".to_string(),
    };
    let result = login_service.login_user(user_that_tries_logging_in);
    assert!(result.is_none());
}

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

#[test]
fn gets_user_if_session_exists_and_is_valid() {
    let dummy_user = "dummy-user";
    let user_service = Arc::new(UserServiceMock::new(dummy_user));
    let session_id = Uuid::new_v4().to_string();
    let dummy_user = AppUser {
        id: 1,
        name: "dummy-user".to_string(),
        beer_count: 2,
        shot_count: 2,
        water_count: 1,
    };
    let dummy_session = Session {
        user: dummy_user.clone(),
        uuid: session_id.clone(),
        exp: Instant::now(),
    };
    let login_service = LoginService {
        user_service,
        sessions: HashMap::from([(session_id.clone(), dummy_session)]),
    };
    let session_user = login_service
        .get_session_user(&session_id)
        .expect("User should be present");
    assert_eq!(session_user, dummy_user);
}
