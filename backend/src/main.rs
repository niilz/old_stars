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
use backend::model::{app_user::AppUser, user::User};
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
fn login(login_data: Json<LoginData>, conn: Db) -> Json<Result<AppUser, &'static str>> {
    match login_user(&*conn, login_data.into_inner()) {
        // TODO: if Login Successfull add "Set-Cooky" header
        Some(user) => Json(Ok(user)),
        None => Json(Err("Login failed")),
    }
}

#[post("/register", format = "json", data = "<user>")]
fn register(user: Json<LoginData>, conn: Db) -> Result<Json<User>, String> {
    let user = user.into_inner();
    if user.name.is_empty() || user.pwd.is_empty() {
        eprintln!("user is empty");
        return Err("'name' and 'pwd' must not be empty".to_string());
    }
    match insert_user(&conn, user) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(format!("Could not reigster user. Error: {}", e)),
    }
}

#[get("/all", format = "json")]
fn all_users(conn: Db) -> Result<Json<Vec<User>>, String> {
    match get_users(&conn) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(format!("Could not get all users. Error: {}", e)),
    }
}

#[delete("/delete/<id>")]
fn delete_user(conn: Db, id: i32) -> Result<Json<User>, String> {
    match delete_user_from_db(&conn, id) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(format!("Did NOT delete user with id {}! Error: {}", id, e)),
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
