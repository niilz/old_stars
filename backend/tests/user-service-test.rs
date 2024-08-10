mod mocks;
use mocks::user_service::UserServiceMock;

use backend::{
    model::login_data::LoginData,
    service::user_service::{UserService, UserServiceError},
};

#[test]
fn create_user_assigns_role_user() {
    let mut user_service_mock = UserServiceMock::new("dummy-user");
    let new_user_dummy = LoginData {
        name: "dummy-name".to_string(),
        pwd: "dummy-pwd".to_string(),
    };
    //let _ = user_service_mock.insert_user(new_user_dummy);
}

#[test]
fn can_create_user_service_error() {
    let dummy_ctx = "Some process";
    let dummy_msg = "Something did not work";

    let error_mock = UserServiceError::new(dummy_ctx, &dummy_msg);
    assert_eq!(
        error_mock.message,
        format!("Error during {dummy_ctx}: {dummy_msg}")
    );
}
