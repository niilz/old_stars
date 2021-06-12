#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use backend::db::auth_service::*;
use backend::db::user_service::*;
use backend::model::login_data::LoginData;
use backend::model::user::User;
use diesel::{pg::PgConnection, prelude::*, result::Error};
use rocket_contrib::{database, json::Json};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::ops::Deref;
use std::{env, str::FromStr};

diesel_migrations::embed_migrations!();

const FRONT_END_URL: &'static str = "http://localhost:3000/";

#[database("db")]
struct Db(diesel::PgConnection);

#[get("/")]
fn hello() -> Json<&'static str> {
    Json("Hello from the backend-api")
}

#[head("/")]
fn head() -> Json<&'static str> {
    Json("Head Response")
}

#[options("/")]
fn options() -> Json<&'static str> {
    Json("Options Response")
}

#[options("/login")]
fn options_login() -> Json<&'static str> {
    Json("Options Response")
}

#[post("/login", format = "json", data = "<login_data>")]
fn login(login_data: Json<LoginData>, conn: Db) -> Json<String> {
    let secret_pwd = check_pwd(&*conn, login_data.into_inner());
    // TODO: if Login Successfull add "Set-Cooky" header
    Json(format!("from db! pwd: {}", secret_pwd))
}

#[post("/register", format = "json", data = "<user>")]
fn register(user: Json<LoginData>, conn: Db) -> Json<String> {
    match insert_user(&conn, user.into_inner()) {
        Ok(amount) => Json(format!("inserted {} user", amount)),
        Err(e) => Json(format!("Did not insert user! Error: {}", e)),
    }
}

#[get("/all", format = "json")]
fn all_users(conn: Db) -> Result<Json<Vec<User>>, Error> {
    match get_users(&conn) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}

#[delete("/delete/<id>")]
fn delete_user(conn: Db, id: i32) -> Json<String> {
    match delete_user_from_db(&conn, id) {
        Ok(amount) => Json(format!("Deleted {} user. ID was: {}", amount, id)),
        Err(e) => Json(format!("Did NOT delete user with id {}! Error: {}", id, e)),
    }
}

fn main() {
    let connection = establish_connection();
    embedded_migrations::run(&connection);

    let mut cors_options = CorsOptions::default();
    cors_options.allowed_origins = AllowedOrigins::some_exact(&[FRONT_END_URL]);
    cors_options.allowed_headers =
        AllowedHeaders::some(&["Accept", "Content-Type", "Access-Control-Allow-Origin"]);
    cors_options.allowed_methods = ["GET", "POST", "HEAD", "OPTIONS", "DELETE"]
        .iter()
        .map(|m| FromStr::from_str(m).unwrap())
        .collect();

    rocket::ignite()
        .mount(
            "/",
            routes![
                hello,
                head,
                options,
                options_login,
                login,
                register,
                all_users,
                delete_user
            ],
        )
        .attach(cors_options.to_cors().unwrap())
        .attach(Db::fairing())
        .launch();
}

fn establish_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("Could not read DATABASE_URL from env");
    PgConnection::establish(&db_url).expect("Could not establish_connection()")
}
