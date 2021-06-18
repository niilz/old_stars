use crate::db::auth_service::{get_salt, hash};
use crate::model::login_data::LoginData;
use crate::model::user::User;
use crate::schema::old_users::dsl::*;
use diesel::dsl::not;
use diesel::{insert_into, prelude::*, PgConnection};

pub fn insert_user(conn: &PgConnection, user: LoginData) -> QueryResult<User> {
    let new_salt = get_salt();
    let hashed_pwd = hash(user.pwd, &new_salt);
    insert_into(old_users)
        .values((
            name.eq(user.name),
            salt.eq(new_salt),
            pwd.eq(hashed_pwd),
            beer_count.eq(0),
            shot_count.eq(0),
            water_count.eq(0),
            fk_icon_id.eq(42),
        ))
        .get_result(conn)
}

pub fn get_users(conn: &PgConnection) -> QueryResult<Vec<User>> {
    old_users
        .filter(not(name.eq("club").and(not(name.eq("admin")))))
        .load::<User>(conn)
}

pub fn delete_user_from_db(conn: &PgConnection, del_id: i32) -> QueryResult<User> {
    diesel::delete(old_users.filter(id.eq(del_id))).get_result(conn)
}

pub fn add_drink_to_user(conn: &PgConnection, update_id: i32, drink: &str) -> QueryResult<User> {
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
