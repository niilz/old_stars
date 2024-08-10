// @generated automatically by Diesel CLI.

diesel::table! {
    old_users (id) {
        id -> Int4,
        name -> Varchar,
        pwd -> Varchar,
        beer_count -> Int4,
        shot_count -> Int4,
        water_count -> Int4,
        fk_icon_id -> Int4,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        user_id -> Int4,
        role -> Varchar,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        name -> Varchar,
        value -> Varchar,
        user_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    old_users,
    roles,
    sessions,
);
