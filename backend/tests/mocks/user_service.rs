use backend::{
    model::{
        role::OldStarsRole,
        user::{InsertUser, User},
    },
    service::user_service::{UserService, UserServiceError},
};
use std::collections::HashMap;

pub(crate) struct UserServiceMock {
    dummy_db: HashMap<String, (User, String)>,
}

impl UserServiceMock {
    fn dummy_insert(&self, new_user: InsertUser) -> User {
        User {
            user_id: self.dummy_db.len() as i32 + 1,
            name: new_user.name.to_string(),
            pwd: new_user.pwd.to_string(),
            ..Default::default()
        }
    }
}

impl UserServiceMock {
    pub(crate) fn new() -> Self {
        Self {
            dummy_db: HashMap::new(),
        }
    }
}

fn insert_to_user(user: &InsertUser, id: i32) -> User {
    User {
        user_id: id,
        name: user.name.to_string(),
        pwd: user.pwd.to_string(),
        ..Default::default()
    }
}

impl UserService for UserServiceMock {
    fn get_users(&mut self) -> Result<Vec<(User, String)>, UserServiceError> {
        Ok(self
            .dummy_db
            .values()
            .cloned()
            .filter(|(_, role)| role == "user")
            .collect())
    }

    fn get_user_by_name(&mut self, user_name: &str) -> Result<(User, String), UserServiceError> {
        match self.dummy_db.get(user_name) {
            Some(user_and_role) => Ok(user_and_role.to_owned()),
            _ => Err(UserServiceError::new("Test-Get: ", &"User-NotFound")),
        }
    }

    fn insert_into_repo(
        &mut self,
        new_user: InsertUser,
        role: OldStarsRole,
    ) -> Result<User, UserServiceError> {
        let user = insert_to_user(&new_user, self.dummy_db.len() as i32 + 1);
        self.dummy_db
            .insert(user.name.to_string(), (user, role.to_string()));

        let user = self
            .dummy_db
            .get(new_user.name)
            .map(|(user, _)| user.to_owned())
            .unwrap();

        Ok(user)
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
