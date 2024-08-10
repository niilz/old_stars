use crate::{
    model::{login_data::LoginData, role::OldStarsRole, user::User},
    repository::connection::OldStarDb,
    schema::{old_users::dsl::*, roles::dsl::*},
    service::auth_service::hash,
};
use argon2::password_hash;
use diesel::{dsl::not, insert_into, prelude::*};
use std::{error::Error, fmt};

pub trait UserService: Send + Sync {
    fn get_users(&mut self) -> Result<Vec<User>, UserServiceError>;
    fn get_user_by_name(&mut self, user_name: &str) -> Result<User, UserServiceError>;
    fn insert_user(&mut self, new_user: LoginData) -> Result<User, UserServiceError>;
    fn delete_user(&mut self, id: i32) -> Result<User, UserServiceError>;
    fn add_drink_to_user<'a>(
        &mut self,
        update_id: i32,
        drink: &'a str,
    ) -> Result<User, UserServiceError>;
}

#[derive(Debug)]
pub struct UserServiceError {
    message: String,
}

pub struct DbUserService {
    pub db: OldStarDb,
}

impl UserService for DbUserService {
    fn get_users(&mut self) -> Result<Vec<User>, UserServiceError> {
        let users = old_users
            .inner_join(roles)
            .filter(role.eq(OldStarsRole::User.to_string()))
            .select(User::as_select())
            .load::<User>(&mut self.db.connection())?;
        Ok(users)
    }

    fn get_user_by_name(&mut self, user_name: &str) -> Result<User, UserServiceError> {
        let user = old_users
            .filter(name.eq(user_name))
            .first::<User>(&mut self.db.connection())?;
        Ok(user)
    }

    fn insert_user(&mut self, new_user: LoginData) -> Result<User, UserServiceError> {
        // Do not allow for duplicate users
        let users_with_given_name = old_users
            .filter(name.eq(&new_user.name))
            .load::<User>(&mut self.db.connection())?;

        if users_with_given_name.len() != 0 {
            return Err(UserServiceError::new(
                "Registration",
                &"User already exists",
            ));
        }
        let hashed_pwd = hash(&new_user.pwd)?;
        let inserted_user = insert_into(old_users)
            .values((
                name.eq(new_user.name),
                pwd.eq(hashed_pwd.to_string()),
                beer_count.eq(0),
                shot_count.eq(0),
                water_count.eq(0),
                fk_icon_id.eq(42),
            ))
            .get_result(&mut self.db.connection())?;
        Ok(inserted_user)
    }

    fn delete_user(&mut self, del_id: i32) -> Result<User, UserServiceError> {
        let deleted_user = diesel::delete(old_users.filter(id.eq(del_id)))
            .get_result(&mut self.db.connection())?;
        Ok(deleted_user)
    }

    fn add_drink_to_user<'a>(
        &mut self,
        update_id: i32,
        drink: &'a str,
    ) -> Result<User, UserServiceError> {
        // TODO: Check if adding is allowd according to water:alcohol ratio
        let update_user = old_users.filter(id.eq(update_id));
        let mut connection = self.db.connection();
        let updated_user = match drink {
            "beer" => diesel::update(update_user)
                .set(beer_count.eq(beer_count + 1))
                .get_result(&mut connection)?,
            "shot" => diesel::update(update_user)
                .set(shot_count.eq(shot_count + 1))
                .get_result(&mut connection)?,
            "water" => diesel::update(update_user)
                .set(water_count.eq(water_count + 1))
                .get_result(&mut connection)?,
            _ => unimplemented!("Other drinks are not supported"),
        };
        Ok(updated_user)
    }
}

impl UserServiceError {
    pub fn new(context: &str, error: &(dyn fmt::Display)) -> Self {
        UserServiceError {
            message: format!("Error during {}: {}", context, error),
        }
    }
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in UserService: {}", self.message)
    }
}
impl Error for UserServiceError {}

impl From<diesel::result::Error> for UserServiceError {
    fn from(error: diesel::result::Error) -> Self {
        Self::new("db-communication", &error)
    }
}

impl From<password_hash::Error> for UserServiceError {
    fn from(error: password_hash::Error) -> Self {
        Self::new("Hashing", &error)
    }
}

#[cfg(test)]
mod tests {
    use super::UserServiceError;

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
}
