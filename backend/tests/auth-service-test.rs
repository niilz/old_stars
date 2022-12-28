use backend::{model::login_data::LoginData, service::auth_service::LoginService};
use std::sync::Arc;

mod mocks;

use mocks::user_service::UserServiceMock;

#[test]
fn gets_user_if_login_succeeds() {
    let dummy_user = "dummy-user";
    let login_service = LoginService {
        user_service: Arc::new(UserServiceMock::new(dummy_user)),
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
    };
    let user_that_tries_logging_in = LoginData {
        name: non_existing.to_string(),
        pwd: "hashed_pwd".to_string(),
    };
    let result = login_service.login_user(user_that_tries_logging_in);
    assert!(result.is_none());
}
