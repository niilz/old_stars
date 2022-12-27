use backend::{
    db::{auth_service::hash, user_service::UserServiceError},
    model::{login_data::LoginData, user::User},
    UserService,
};
pub(crate) struct UserServiceMock;
impl UserService for UserServiceMock {
    fn get_user_by_name(&self, user_name: &str) -> Result<User, UserServiceError> {
        let existing_user = "dummy-user";
        match user_name {
            "dummy-user" => Ok(User {
                id: 1,
                name: existing_user.to_string(),
                pwd: hash("hashed-pwd").unwrap().to_string(),
                beer_count: 0,
                shot_count: 0,
                water_count: 0,
                fk_icon_id: 0,
            }),
            _ => Err(UserServiceError::new(
                "Test-User-NotFound: ",
                &"Test-Error" as &dyn std::fmt::Display,
            )),
        }
    }

    fn insert_user(&self, new_user: LoginData) -> Result<User, UserServiceError> {
        todo!()
    }

    fn delete_user(&self, id: i32) -> Result<User, UserServiceError> {
        todo!()
    }

    fn add_drink_to_user<'a>(
        &self,
        update_id: i32,
        drink: &'a str,
    ) -> Result<User, UserServiceError> {
        todo!()
    }
}
