use crate::db::auth_service::hash;
use crate::model::login_data::LoginData;
use crate::model::user::User;
use crate::schema::old_users::dsl::*;
use argon2::password_hash;
use diesel::dsl::not;
use diesel::{insert_into, prelude::*, PgConnection};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UserServiceError {
    message: String,
}

impl UserServiceError {
    fn new(context: &str, error: &(dyn fmt::Display)) -> Self {
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

pub fn insert_user(conn: &PgConnection, user: LoginData) -> Result<User, UserServiceError> {
    let hashed_pwd = hash(&user.pwd)?;
    let inserted_user = insert_into(old_users)
        .values((
            name.eq(user.name),
            pwd.eq(hashed_pwd.to_string()),
            beer_count.eq(0),
            shot_count.eq(0),
            water_count.eq(0),
            fk_icon_id.eq(42),
        ))
        .get_result(conn)?;
    Ok(inserted_user)
}

pub fn get_users(conn: &PgConnection) -> QueryResult<Vec<User>> {
    old_users
        .filter(not(name.eq("club").or(name.eq("admin"))))
        .load::<User>(conn)
}

pub fn delete_user_from_db(conn: &PgConnection, del_id: i32) -> QueryResult<User> {
    diesel::delete(old_users.filter(id.eq(del_id))).get_result(conn)
}

pub fn add_drink_to_user<'a>(
    conn: &PgConnection,
    update_id: i32,
    drink: &'a str,
) -> QueryResult<User> {
    let update_user = old_users.filter(id.eq(update_id));
    match drink {
        "beer" => diesel::update(update_user)
            .set(beer_count.eq(beer_count + 1))
            .get_result(conn),
        "shot" => diesel::update(update_user)
            .set(shot_count.eq(shot_count + 1))
            .get_result(conn),
        "water" => diesel::update(update_user)
            .set(water_count.eq(water_count + 1))
            .get_result(conn),
        _ => unimplemented!("Other drinks are not supported"),
    }
}
