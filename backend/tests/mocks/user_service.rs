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
    fn new(user_name: &str) -> Self {
        let dummy_user = User {
            id: 1,
            name: user_name.to_string(),
            pwd: hash("hashed-pwd").unwrap().to_string(),
            beer_count: 0,
            shot_count: 0,
            water_count: 0,
            fk_icon_id: 0,
        };
        let dummy_db_by_id = HashMap::from([(1, dummy_user)]);
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
            Some(user) => Ok(*user),
            _ => Err(UserServiceError::new("Test-Get: ", &"User-NotFound")),
        }
    }

    fn insert_user(&self, new_user: LoginData) -> Result<User, UserServiceError> {
        match self.dummy_db_by_name.get(&new_user.name) {
            Some(_) => Err(UserServiceError::new(
                "Test-Insert: ",
                &"User-Already-Exists",
            )),
            None => {
                self.id += 1;
                let user = User {
                    id: self.id,
                    name: new_user.name,
                    pwd: new_user.pwd,
                    beer_count: 0,
                    shot_count: 0,
                    water_count: 0,
                    fk_icon_id: 0,
                };
                user.id = self.id;
                self.dummy_db_by_id.insert(self.id, user);
                self.dummy_db_by_name.insert(user.name, user);
                Ok(user)
            }
        }
    }

    fn delete_user(&self, id: i32) -> Result<User, UserServiceError> {
        match self.dummy_db_by_id.remove(&id) {
            Some(user) => {
                self.dummy_db_by_name
                    .remove(&user.name)
                    .expect("Deleted user by id did not exist by name");
                Ok(user)
            }
            None => Err(UserServiceError::new(
                "Test-Delete: ",
                &"User-did-not-exist",
            )),
        }
    }

    fn add_drink_to_user<'a>(
        &self,
        update_id: i32,
        drink: &'a str,
    ) -> Result<User, UserServiceError> {
        todo!()
    }
}
