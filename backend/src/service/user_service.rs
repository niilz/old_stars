use crate::{
    model::{
        login_data::LoginData,
        role::{InsertRole, OldStarsRole},
        user::{InsertUser, User},
    },
    repository::connection::OldStarDb,
    schema::{
        old_users::dsl::*,
        roles::dsl::{role, roles, user_id as role_user_id},
    },
};
use argon2::{
    password_hash::{self, SaltString},
    Argon2, PasswordHasher,
};
use diesel::{insert_into, prelude::*};
use rand_core::OsRng;
use std::{error::Error, fmt};

pub trait UserService: Send + Sync {
    /// Required Methods
    fn get_users(&mut self) -> Result<Vec<(User, String)>, UserServiceError>;
    fn get_user_by_name(&mut self, user_name: &str) -> Result<(User, String), UserServiceError>;
    fn insert_into_repo(
        &mut self,
        new_user: InsertUser,
        role: &OldStarsRole,
    ) -> Result<User, UserServiceError>;
    fn delete_user(&mut self, id: i32) -> Result<User, UserServiceError>;
    fn add_drink_to_user(&mut self, update_id: i32, drink: &str) -> Result<User, UserServiceError>;

    /// Provided Methods
    fn get_user_and_role(
        &mut self,
        user_name: &str,
    ) -> Result<(User, OldStarsRole), UserServiceError> {
        let (user, user_role) = self.get_user_by_name(user_name)?;
        Ok((user, self.map_role(&user_role)?))
    }

    fn get_users_and_roles(&mut self) -> Result<Vec<(User, OldStarsRole)>, UserServiceError> {
        let users_and_roles = self
            .get_users()?
            .into_iter()
            .map(|(user, user_role)| (user, self.map_role(&user_role)))
            .filter(|(_, user_role)| user_role.is_ok())
            .map(|(user, user_role)| (user, user_role.unwrap()))
            .collect();
        Ok(users_and_roles)
    }

    fn map_role(&self, user_role: &str) -> Result<OldStarsRole, UserServiceError> {
        match user_role.try_into() {
            Ok(user_role) => Ok(user_role),
            Err(e) => {
                eprintln!("Could not map role '{user_role}'");
                Err(UserServiceError::new(
                    "get-user-and-role",
                    &format!("converting role failed: {e:?}"),
                ))
            }
        }
    }

    fn insert_user(
        &mut self,
        new_user: &LoginData,
    ) -> Result<(User, OldStarsRole), UserServiceError> {
        self.insert_with_role(new_user, OldStarsRole::User)
    }

    // not exposed to api
    fn insert_with_role(
        &mut self,
        new_user: &LoginData,
        user_role: OldStarsRole,
    ) -> Result<(User, OldStarsRole), UserServiceError> {
        // Do not allow for duplicate users
        let users_with_given_name = self.get_user_by_name(&new_user.name);

        if users_with_given_name.is_ok() {
            return Err(UserServiceError::new(
                "Registration",
                &"User already exists",
            ));
        }
        let hashed_pwd = self.hash(&new_user.pwd)?;
        let new_user = InsertUser::new(&new_user.name, &hashed_pwd);

        let admin = self.insert_into_repo(new_user, &user_role)?;
        Ok((admin, user_role))
    }

    fn hash(&self, user_pwd: &str) -> Result<String, password_hash::Error> {
        let rnd_salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        let pwd_hash = argon.hash_password(user_pwd.as_bytes(), &rnd_salt)?;
        Ok(pwd_hash.to_string())
    }
}

#[derive(Debug)]
pub struct UserServiceError {
    pub message: String,
}

pub struct DbUserService {
    pub db: OldStarDb,
}

impl UserService for DbUserService {
    fn get_users(&mut self) -> Result<Vec<(User, String)>, UserServiceError> {
        let users_and_roles = old_users
            .inner_join(roles)
            .filter(role.eq(OldStarsRole::User.to_string()))
            .select((User::as_select(), role))
            .load::<(User, String)>(&mut self.db.connection())?;
        Ok(users_and_roles)
    }

    fn get_user_by_name(&mut self, user_name: &str) -> Result<(User, String), UserServiceError> {
        let (user, user_role) = old_users
            .inner_join(roles)
            .filter(name.eq(user_name))
            .select((User::as_select(), role))
            .first::<(User, String)>(&mut self.db.connection())?;
        Ok((user, user_role))
    }

    fn insert_into_repo(
        &mut self,
        new_user: InsertUser,
        user_role: &OldStarsRole,
    ) -> Result<User, UserServiceError> {
        let inserted_user: User = insert_into(old_users)
            .values(new_user)
            .get_result(&mut self.db.connection())?;
        let user_role = InsertRole {
            role: &user_role.to_string(),
            user_id: inserted_user.user_id,
        };
        insert_into(roles)
            .values(user_role)
            .execute(&mut self.db.connection())?;
        Ok(inserted_user)
    }

    fn delete_user(&mut self, del_id: i32) -> Result<User, UserServiceError> {
        diesel::delete(roles.filter(role_user_id.eq(del_id))).execute(&mut self.db.connection())?;
        let deleted_user = diesel::delete(old_users.filter(user_id.eq(del_id)))
            .get_result(&mut self.db.connection())?;
        Ok(deleted_user)
    }

    fn add_drink_to_user(&mut self, update_id: i32, drink: &str) -> Result<User, UserServiceError> {
        // TODO: Check if adding is allowd according to water:alcohol ratio
        let update_user = old_users.filter(user_id.eq(update_id));
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
