use crate::model::user::User;
use crate::schema::old_users::dsl::*;
use diesel::{insert_into, prelude::*, PgConnection};

pub fn insert_user(user_name: String, user_pwd: String, conn: &PgConnection) {
    let new_salt = "rnd_salt_";
    let salted_pwd = format!("{}{}", new_salt, user_pwd);
    let hashed_pwd = format!("hashed_{}", salted_pwd);
    insert_into(old_users)
        .values((
            name.eq(user_name),
            salt.eq(new_salt),
            pwd.eq(hashed_pwd),
            fk_icon_id.eq(42),
        ))
        .execute(conn)
        .expect("Could not insert user");
}
