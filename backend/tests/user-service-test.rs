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
    let user = user_service_mock.insert_user(new_user_dummy);
    match user {
        Ok(user) => assert_eq!(user.beer_count, 0),
        Err(e) => panic!("test failed with: {e:?}"),
    }

    let all_users = user_service_mock.get_users();
    match all_users {
        Ok(users) => assert_eq!(users.len(), 1),
        Err(e) => panic!("test failed: {e:?}"),
    }
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
