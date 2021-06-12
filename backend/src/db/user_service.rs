use crate::model::login_data::LoginData;
use crate::model::user::User;
use crate::schema::old_users::dsl::*;
use diesel::{insert_into, prelude::*, PgConnection};

pub fn insert_user(conn: &PgConnection, user: LoginData) -> QueryResult<User> {
    let new_salt = "rnd_salt_";
    let salted_pwd = format!("{}{}", new_salt, user.pwd);
    let hashed_pwd = format!("hashed_{}", salted_pwd);
    insert_into(old_users)
        .values((
            name.eq(user.name),
            salt.eq(new_salt),
            pwd.eq(hashed_pwd),
            fk_icon_id.eq(42),
        ))
        .get_result(conn)
}

pub fn get_users(conn: &PgConnection) -> QueryResult<Vec<User>> {
    old_users.load::<User>(conn)
}

pub fn delete_user_from_db(conn: &PgConnection, del_id: i32) -> QueryResult<User> {
    diesel::delete(old_users.filter(id.eq(del_id))).get_result(conn)
}
