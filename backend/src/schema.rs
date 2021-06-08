table! {
    old_users {
        id -> Integer,
        name -> VarChar,
        salt -> VarChar,
        pwd -> VarChar,
        fk_icon_id -> Integer,
    }
}

table! {
    sessions {
        id -> Integer,
        name -> VarChar,
        value -> VarChar,
        user_id -> Integer,
    }
}
