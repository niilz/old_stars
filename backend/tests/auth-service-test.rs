use backend::{
    model::{
        app_user::AppUser,
        login_data::LoginData,
        role::OldStarsRole,
        session::{Session, TWENTY_FOUR_HOURS},
    },
    service::{auth_service::LoginService, user_service::UserService},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use uuid::Uuid;

mod mocks;

use mocks::user_service::UserServiceMock;

const DUMMY_USER_NAME: &str = "dummy-user";
const DUMMY_ID: i32 = 1;

#[test]
fn gets_user_if_login_succeeds() {
    let dummy_user = LoginData {
        name: "dummy-user".to_string(),
        pwd: "hashed-pwd".to_string(),
    };
    let mut user_service_mock = UserServiceMock::new();
    let Ok(_user) = user_service_mock.insert_user(&dummy_user) else {
        panic!("test failed: mock-insert did not work");
    };

    let mut login_service = LoginService {
        user_service: Arc::new(Mutex::new(user_service_mock)),
        sessions: HashMap::new(),
    };

    let Some(session_ctx) = login_service.login_user(&dummy_user) else {
        panic!("test fails: No user found for '{dummy_user:?}'");
    };
    let found_user = session_ctx.user;

    assert_eq!(found_user.id, 1);
    assert_eq!(found_user.name, dummy_user.name);
    assert_eq!(found_user.beer_count, 0);
    assert_eq!(found_user.water_count, 0);
    assert_eq!(found_user.shot_count, 0);
    assert_eq!(found_user.role, OldStarsRole::User);
}

#[test]
fn gets_none_if_user_does_not_exist() {
    let user_that_tries_logging_in = LoginData {
        name: "non-existing-user".to_string(),
        pwd: "hashed_pwd".to_string(),
    };

    let mut login_service = LoginService {
        user_service: Arc::new(Mutex::new(UserServiceMock::new())),
        sessions: HashMap::new(),
    };
    let result = login_service.login_user(&user_that_tries_logging_in);
    assert!(result.is_none());
}

#[test]
fn gets_user_if_session_exists_and_is_valid() {
    let dummy_user = LoginData {
        name: "dummy-user".to_string(),
        pwd: "hashed-pwd".to_string(),
    };
    let user_service = Arc::new(Mutex::new(UserServiceMock::new()));

    let session_id = Uuid::new_v4().to_string();
    let dummy_app_user = get_dummy_user(&dummy_user.name);
    let dummy_session = Session {
        user: dummy_app_user.clone(),
        uuid: session_id.clone(),
        exp: SystemTime::now() + Duration::from_secs(60),
    };
    let login_service = LoginService {
        user_service,
        sessions: HashMap::from([(session_id.clone(), dummy_session)]),
    };
    let session_user = login_service
        .get_session_user(&session_id)
        .expect("User should be present");
    assert_eq!(session_user, dummy_app_user);
}

#[test]
fn no_user_if_session_expired() {
    let user_service = Arc::new(Mutex::new(UserServiceMock::new()));
    let session_id = Uuid::new_v4().to_string();
    let dummy_app_user = get_dummy_user(DUMMY_USER_NAME);
    let expired_session = Session {
        user: dummy_app_user,
        uuid: session_id.clone(),
        exp: SystemTime::now() - Duration::from_secs(TWENTY_FOUR_HOURS),
    };
    let login_service = LoginService {
        user_service,
        sessions: HashMap::from([(session_id.clone(), expired_session)]),
    };
    let no_user_found = login_service.get_session_user(&session_id);
    assert!(no_user_found.is_none());
}

#[test]
fn no_user_if_no_session_present() {
    let user_service = Arc::new(Mutex::new(UserServiceMock::new()));
    let session_id = Uuid::new_v4().to_string();
    let login_service = LoginService {
        user_service,
        sessions: HashMap::new(),
    };
    let no_user_found = login_service.get_session_user(&session_id);
    assert!(no_user_found.is_none());
}

#[test]
fn can_remove_session() {
    let user_service = Arc::new(Mutex::new(UserServiceMock::new()));
    let session_id = Uuid::new_v4().to_string();
    let dummy_app_user = get_dummy_user(DUMMY_USER_NAME);
    let dummy_session = Session::new(dummy_app_user);
    let mut login_service = LoginService {
        user_service,
        sessions: HashMap::from([(session_id.to_string(), dummy_session)]),
    };
    let delete_result = login_service.remove_session(&session_id);
    assert!(delete_result.is_ok());
}

#[test]
fn can_remove_session_by_user_id() {
    let user_service = Arc::new(Mutex::new(UserServiceMock::new()));
    let session_id = Uuid::new_v4().to_string();
    let dummy_app_user = get_dummy_user(DUMMY_USER_NAME);
    let dummy_session = Session::new(dummy_app_user);
    let mut login_service = LoginService {
        user_service,
        sessions: HashMap::from([(session_id.to_string(), dummy_session)]),
    };
    let delete_result = login_service.remove_user_session(DUMMY_ID);
    assert!(delete_result.is_ok());
}

#[test]
fn err_if_no_session_to_remove_available() {
    let user_service = Arc::new(Mutex::new(UserServiceMock::new()));
    let session_id = Uuid::new_v4().to_string();
    let mut login_service = LoginService {
        user_service,
        sessions: HashMap::new(),
    };
    let delete_result = login_service.remove_session(&session_id);
    assert!(delete_result.is_err());
}

// Helper
fn get_dummy_user(user_name: &str) -> AppUser {
    AppUser {
        id: DUMMY_ID,
        role: OldStarsRole::User,
        name: user_name.to_string(),
        beer_count: 2,
        shot_count: 2,
        other_count: 42,
        water_count: 1,
    }
}
