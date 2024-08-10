// @generated automatically by Diesel CLI.

diesel::table! {
    old_users (user_id) {
        user_id -> Int4,
        name -> Varchar,
        pwd -> Varchar,
        beer_count -> Int4,
        shot_count -> Int4,
        water_count -> Int4,
        fk_icon_id -> Int4,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Int4,
        fk_user_id -> Int4,
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

diesel::joinable!(roles -> old_users (fk_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    old_users,
    roles,
    sessions,
);
