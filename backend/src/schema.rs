table! {
    old_users {
        id -> Integer,
        name -> VarChar,
        salt -> VarChar,
        pwd -> VarChar,
        beer_count -> Integer,
        shot_count -> Integer,
        water_count -> Integer,
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
