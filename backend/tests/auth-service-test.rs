use backend::{
    model::{
        login_data::LoginData,
        role::OldStarsRole,
        session::{Session, SessionUser, TWENTY_FOUR_HOURS},
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
fn club_login_works() {
    let dummy_club_user = LoginData {
        name: "club".to_string(),
        pwd: "club_pwd".to_string(),
    };
    let mut user_service_mock = UserServiceMock::new();
    let Ok(_user) = user_service_mock.insert_user(&dummy_club_user) else {
        panic!("test failed: mock-insert did not work");
    };

    let mut login_service = login_service_mock_with_user_service(user_service_mock);
    let Some(club_session) = login_service.login_club(&dummy_club_user.pwd) else {
        panic!(
            "test fails: Login for '{dummy_club_user:?}' failed. Used PWD: '{}'",
            dummy_club_user.pwd
        );
    };

    assert!(Uuid::parse_str(&club_session.uuid).is_ok());

    let has_club_access = login_service.has_club_access(&club_session.uuid);
    assert!(has_club_access);
}

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

    let mut login_service = login_service_mock_with_user_service(user_service_mock);

    let Some(session_ctx) = login_service.login_user(&dummy_user) else {
        panic!("test fails: No user found for '{dummy_user:?}'");
    };
    let found_user = session_ctx.user;

    assert_eq!(found_user.id, 1);
    assert_eq!(found_user.name, dummy_user.name);
    assert_eq!(found_user.role, OldStarsRole::User);
}

#[test]
fn gets_none_if_user_does_not_exist() {
    let user_that_tries_logging_in = LoginData {
        name: "non-existing-user".to_string(),
        pwd: "hashed_pwd".to_string(),
    };

    let mut login_service = login_service_mock();
    let result = login_service.login_user(&user_that_tries_logging_in);
    assert!(result.is_none());
}

#[test]
fn gets_user_if_session_exists_and_is_valid() {
    let dummy_user = LoginData {
        name: "dummy-user".to_string(),
        pwd: "hashed-pwd".to_string(),
    };

    let session_id = Uuid::new_v4().to_string();
    let dummy_app_user = get_dummy_user(&dummy_user.name);
    let dummy_session = Session {
        user: dummy_app_user.clone(),
        uuid: session_id.clone(),
        exp: SystemTime::now() + Duration::from_secs(60),
    };

    let mut login_service = login_service_mock();
    login_service.sessions = HashMap::from([(session_id.clone(), dummy_session)]);

    let session = login_service
        .get_session(&session_id)
        .expect("User should be present");
    assert_eq!(session.user, dummy_app_user);
}

#[test]
fn no_user_if_session_expired() {
    let session_id = Uuid::new_v4().to_string();
    let dummy_app_user = get_dummy_user(DUMMY_USER_NAME);
    let expired_session = Session {
        user: dummy_app_user,
        uuid: session_id.clone(),
        exp: SystemTime::now() - Duration::from_secs(TWENTY_FOUR_HOURS),
    };
    let mut login_service = login_service_mock();
    login_service.sessions = HashMap::from([(session_id.clone(), expired_session)]);

    let no_user_found = login_service.get_session(&session_id);
    assert!(no_user_found.is_none());
}

#[test]
fn no_user_if_no_session_present() {
    let session_id = Uuid::new_v4().to_string();
    let login_service = login_service_mock();
    let no_user_found = login_service.get_session(&session_id);
    assert!(no_user_found.is_none());
}

#[test]
fn can_remove_session() {
    let session_id = Uuid::new_v4().to_string();
    let dummy_app_user = get_dummy_user(DUMMY_USER_NAME);
    let dummy_session = Session::new(dummy_app_user);
    let mut login_service = login_service_mock();
    login_service.sessions = HashMap::from([(session_id.to_string(), dummy_session)]);
    let delete_result = login_service.remove_session(&session_id);
    assert!(delete_result.is_ok());
}

#[test]
fn can_remove_session_by_user_id() {
    let session_id = Uuid::new_v4().to_string();
    let dummy_app_user = get_dummy_user(DUMMY_USER_NAME);
    let dummy_session = Session::new(dummy_app_user);
    let mut login_service = login_service_mock();
    login_service.sessions = HashMap::from([(session_id.to_string(), dummy_session)]);
    let delete_result = login_service.remove_user_session(DUMMY_ID);
    assert!(delete_result.is_ok());
}

#[test]
fn err_if_no_session_to_remove_available() {
    let session_id = Uuid::new_v4().to_string();
    let mut login_service = login_service_mock();
    let delete_result = login_service.remove_session(&session_id);
    assert!(delete_result.is_err());
}

// Helper
fn get_dummy_user(user_name: &str) -> SessionUser {
    SessionUser {
        id: DUMMY_ID,
        role: OldStarsRole::User,
        name: user_name.to_string(),
    }
}

fn login_service_mock_with_user_service(
    user_service: impl UserService + Send + Sync + 'static,
) -> LoginService {
    let user_service = Arc::new(Mutex::new(user_service));
    LoginService {
        user_service,
        sessions: HashMap::new(),
        club_sessions: HashMap::new(),
    }
}

fn login_service_mock() -> LoginService {
    login_service_mock_with_user_service(UserServiceMock::new())
}
