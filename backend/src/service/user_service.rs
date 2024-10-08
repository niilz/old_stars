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

use super::error::OldStarsServiceError;

pub trait UserService: Send + Sync {
    /// Required Methods
    fn get_users(&mut self) -> Result<Vec<(User, String)>, OldStarsServiceError>;
    fn get_user_by_name(&mut self, user_name: &str)
        -> Result<(User, String), OldStarsServiceError>;
    fn insert_into_repo(
        &mut self,
        new_user: InsertUser,
        role: &OldStarsRole,
    ) -> Result<User, OldStarsServiceError>;
    fn delete_user(&mut self, id: i32) -> Result<User, OldStarsServiceError>;

    /// Provided Methods
    fn get_user_and_role(
        &mut self,
        user_name: &str,
    ) -> Result<(User, OldStarsRole), OldStarsServiceError> {
        let (user, user_role) = self.get_user_by_name(user_name)?;
        Ok((user, self.map_role(&user_role)?))
    }

    fn get_users_and_roles(&mut self) -> Result<Vec<(User, OldStarsRole)>, OldStarsServiceError> {
        let users_and_roles = self
            .get_users()?
            .into_iter()
            .map(|(user, user_role)| (user, self.map_role(&user_role)))
            .filter(|(_, user_role)| user_role.is_ok())
            .map(|(user, user_role)| (user, user_role.unwrap()))
            .collect();
        Ok(users_and_roles)
    }

    fn map_role(&self, user_role: &str) -> Result<OldStarsRole, OldStarsServiceError> {
        match user_role.try_into() {
            Ok(user_role) => Ok(user_role),
            Err(e) => {
                eprintln!("Could not map role '{user_role}'");
                Err(OldStarsServiceError::new(
                    "get-user-and-role",
                    &format!("converting role failed: {e:?}"),
                ))
            }
        }
    }

    fn insert_user(
        &mut self,
        new_user: &LoginData,
    ) -> Result<(User, OldStarsRole), OldStarsServiceError> {
        self.insert_with_role(new_user, OldStarsRole::User)
    }

    // not exposed to api
    fn insert_with_role(
        &mut self,
        new_user: &LoginData,
        user_role: OldStarsRole,
    ) -> Result<(User, OldStarsRole), OldStarsServiceError> {
        // Do not allow for duplicate users
        let users_with_given_name = self.get_user_by_name(&new_user.name);

        if users_with_given_name.is_ok() {
            return Err(OldStarsServiceError::new(
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

pub struct DbUserService {
    pub db: OldStarDb,
}

impl UserService for DbUserService {
    fn get_users(&mut self) -> Result<Vec<(User, String)>, OldStarsServiceError> {
        let users_and_roles = old_users
            .inner_join(roles)
            .filter(role.eq(OldStarsRole::User.to_string()))
            .select((User::as_select(), role))
            .load::<(User, String)>(&mut self.db.connection())?;
        Ok(users_and_roles)
    }

    fn get_user_by_name(
        &mut self,
        user_name: &str,
    ) -> Result<(User, String), OldStarsServiceError> {
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
    ) -> Result<User, OldStarsServiceError> {
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

    fn delete_user(&mut self, del_id: i32) -> Result<User, OldStarsServiceError> {
        diesel::delete(roles.filter(role_user_id.eq(del_id))).execute(&mut self.db.connection())?;
        let deleted_user = diesel::delete(old_users.filter(user_id.eq(del_id)))
            .get_result(&mut self.db.connection())?;
        Ok(deleted_user)
    }
}

impl From<password_hash::Error> for OldStarsServiceError {
    fn from(error: password_hash::Error) -> Self {
        Self::new("Hashing", &error)
    }
}
