use backend::{
    model::{login_data::LoginData, user::User},
    service::{
        auth_service::hash,
        user_service::{UserService, UserServiceError},
    },
};
use std::collections::HashMap;

pub(crate) struct UserServiceMock {
    dummy_db: HashMap<String, User>,
}

impl UserServiceMock {
    pub(crate) fn new(user_name: &str) -> Self {
        let dummy_user = User {
            id: 1,
            name: user_name.to_string(),
            pwd: hash("hashed-pwd").unwrap().to_string(),
            ..Default::default()
        };
        let dummy_db = HashMap::from([(user_name.to_string(), dummy_user)]);
        Self { dummy_db }
    }
}

impl UserService for UserServiceMock {
    fn get_users(&mut self) -> Result<Vec<User>, UserServiceError> {
        unimplemented!("Not used in tests")
    }

    fn get_user_by_name(&mut self, user_name: &str) -> Result<User, UserServiceError> {
        match self.dummy_db.get(user_name) {
            Some(user) => Ok(user.to_owned()),
            _ => Err(UserServiceError::new("Test-Get: ", &"User-NotFound")),
        }
    }

    fn insert_user(&mut self, _new_user: LoginData) -> Result<User, UserServiceError> {
        unimplemented!("Not needed in tests")
    }

    fn delete_user(&mut self, _id: i32) -> Result<User, UserServiceError> {
        unimplemented!("Not needed in tests")
    }

    fn add_drink_to_user<'a>(
        &mut self,
        _update_id: i32,
        _drink: &'a str,
    ) -> Result<User, UserServiceError> {
        unimplemented!("Not needed in tests")
    }
}
