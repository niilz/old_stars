use crate::model::login_data::LoginData;
use crate::schema::old_users::dsl::*;
use diesel::{insert_into, prelude::*, PgConnection};

pub fn insert_user(conn: &PgConnection, user: LoginData) -> QueryResult<usize> {
    let new_salt = "rnd_salt_";
    let salted_pwd = format!("{}{}", new_salt, user.pwd);
    let hashed_pwd = format!("hashed_{}", salted_pwd);
    insert_into(old_users)
        .values((
            name.eq(user.user_name),
            salt.eq(new_salt),
            pwd.eq(hashed_pwd),
            fk_icon_id.eq(42),
        ))
        .execute(conn)
}
