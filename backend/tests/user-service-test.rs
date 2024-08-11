mod mocks;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use mocks::user_service::UserServiceMock;

use backend::{
    model::{login_data::LoginData, role::OldStarsRole},
    service::user_service::{UserService, UserServiceError},
};

#[test]
fn can_create_hash() {
    let user_service_mock = UserServiceMock::new();
    let hash = user_service_mock.hash("MySecretPwd");
    assert!(hash.is_ok());
}

#[test]
fn can_verify_pwd_with_hash() {
    let user_service_mock = UserServiceMock::new();
    let plain_pwd = "EvenMoreSecure";
    let pwd_hashed = user_service_mock.hash(plain_pwd).unwrap();
    let pwd_parsed = PasswordHash::new(&pwd_hashed).unwrap();
    let argon = Argon2::default();
    assert!(argon
        .verify_password(plain_pwd.as_bytes(), &pwd_parsed)
        .is_ok());
}

#[test]
fn create_user_assigns_role_user() {
    let mut user_service_mock = UserServiceMock::new();

    let new_user_dummy = LoginData {
        name: "dummy-name".to_string(),
        pwd: "dummy-pwd".to_string(),
    };
    let user_and_role = user_service_mock.insert_user(&new_user_dummy);
    match user_and_role {
        Ok((user, role)) => {
            assert_eq!(user.beer_count, 0);
            assert_eq!(role, OldStarsRole::User);
        }
        Err(e) => panic!("test failed with: {e:?}"),
    }

    let all_users = user_service_mock.get_users_and_roles();
    match all_users {
        Ok(users) => assert_eq!(users.len(), 1),
        Err(e) => panic!("test failed: {e:?}"),
    }
}

#[test]
fn admin_is_not_in_all_users() {
    let mut user_service_mock = UserServiceMock::new();

    let new_user_dummy = LoginData {
        name: "dummy-name".to_string(),
        pwd: "dummy-pwd".to_string(),
    };
    let user = user_service_mock.insert_admin(&new_user_dummy);
    match user {
        Ok((user, role)) => {
            assert_eq!(user.beer_count, 0);
            assert_eq!(role, OldStarsRole::Admin)
        }
        Err(e) => panic!("test failed with: {e:?}"),
    }

    let all_users = user_service_mock.get_users_and_roles();
    match all_users {
        Ok(users) => assert!(users.is_empty()),
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
