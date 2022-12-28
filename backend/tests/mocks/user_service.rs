use backend::{
    db::{auth_service::hash, user_service::UserServiceError},
    model::{login_data::LoginData, user::User},
    UserService,
};
use std::collections::HashMap;

pub(crate) struct UserServiceMock {
    id: i32,
    dummy_db_by_id: HashMap<i32, User>,
    dummy_db_by_name: HashMap<String, User>,
}

impl UserServiceMock {
    pub(crate) fn new(user_name: &str) -> Self {
        let dummy_user = User {
            id: 1,
            name: user_name.to_string(),
            pwd: hash("hashed-pwd").unwrap().to_string(),
            beer_count: 0,
            shot_count: 0,
            water_count: 0,
            fk_icon_id: 0,
        };
        let dummy_db_by_id = HashMap::from([(1, dummy_user.clone())]);
        let dummy_db_by_name = HashMap::from([(user_name.to_string(), dummy_user)]);
        Self {
            id: 1,
            dummy_db_by_id,
            dummy_db_by_name,
        }
    }
}

impl UserService for UserServiceMock {
    fn get_user_by_name(&self, user_name: &str) -> Result<User, UserServiceError> {
        match self.dummy_db_by_name.get(user_name) {
            Some(user) => Ok(user.to_owned()),
            _ => Err(UserServiceError::new("Test-Get: ", &"User-NotFound")),
        }
    }

    fn insert_user(&self, _new_user: LoginData) -> Result<User, UserServiceError> {
        unimplemented!("Not needed in tests")
    }

    fn delete_user(&self, _id: i32) -> Result<User, UserServiceError> {
        unimplemented!("Not needed in tests")
    }

    fn add_drink_to_user<'a>(
        &self,
        _update_id: i32,
        _drink: &'a str,
    ) -> Result<User, UserServiceError> {
        unimplemented!("Not needed in tests")
    }
}
