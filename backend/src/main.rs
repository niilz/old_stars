#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use backend::db::auth_service::*;
use backend::db::user_service::*;
use backend::model::app_user::AppUser;
use backend::model::login_data::LoginData;
use diesel::{pg::PgConnection, prelude::*};
use rocket_contrib::{database, json::Json};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::{env, str::FromStr};

diesel_migrations::embed_migrations!();

const FRONT_END_URL_DEV: &'static str = "http://localhost:3000/";
const FRONT_END_URL: &'static str = "https://niilz.github.io/old_stars/";
const FRONT_END_URL_HACK: &'static str = "https://oldstars.ngrok.io/";

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

#[options("/all")]
fn options_all() -> Json<&'static str> {
    Json("Options Response")
}

#[options("/register")]
fn options_register() -> Json<&'static str> {
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
fn register(user: Json<LoginData>, conn: Db) -> Json<Result<AppUser, String>> {
    let user = user.into_inner();
    if user.name.is_empty() || user.pwd.is_empty() {
        eprintln!("user is empty");
        return Json(Err("'name' and 'pwd' must not be empty".to_string()));
    }
    if ["admin", "club"].contains(&&*user.name.to_lowercase()) {
        eprintln!("user tried to register as: {}", user.name);
        return Json(Err("This user-name can not be taken".to_string()));
    }
    match insert_user(&conn, user) {
        Ok(user) => Json(Ok(AppUser::from_user(&user))),
        Err(e) => Json(Err(format!("Could not reigster user. Error: {}", e))),
    }
}

#[get("/all", format = "json")]
fn all_users(conn: Db) -> Json<Result<Vec<AppUser>, String>> {
    match get_users(&conn) {
        Ok(users) => Json(Ok(users
            .iter()
            .map(|user| AppUser::from_user(user))
            .collect())),
        Err(e) => Json(Err(format!("Could not get all users. Error: {}", e))),
    }
}

#[delete("/delete/<id>")]
fn delete_user(conn: Db, id: i32) -> Json<Result<AppUser, String>> {
    match delete_user_from_db(&conn, id) {
        Ok(user) => Json(Ok(AppUser::from_user(&user))),
        Err(e) => Json(Err(format!(
            "Did NOT delete user with id {}! Error: {}",
            id, e
        ))),
    }
}

#[get("/<drink>/<id>")]
fn add_drink(conn: Db, drink: String, id: i32) -> Json<Result<AppUser, String>> {
    match add_drink_to_user(&conn, id, &drink) {
        Ok(updated_user) => Json(Ok(AppUser::from_user(&updated_user))),
        Err(e) => Json(Err(format!(
            "Could not add a {} to user with id {}. Error: {}",
            drink, id, e
        ))),
    }
}

#[launch]
fn main() {
    let connection = establish_connection();
    embedded_migrations::run(&connection);

    let mut cors_options = CorsOptions::default();
    cors_options.allowed_origins =
        AllowedOrigins::some_exact(&[FRONT_END_URL, FRONT_END_URL_DEV, FRONT_END_URL_HACK]);
    cors_options.allowed_headers =
        AllowedHeaders::some(&["Accept", "Content-Type", "Access-Control-Allow-Origin"]);
    cors_options.allowed_methods = ["GET", "POST", "HEAD", "OPTIONS", "DELETE"]
        .iter()
        .map(|m| FromStr::from_str(m).unwrap())
        .collect();

    rocket::build()
        .mount(
            "/",
            routes![
                hello,
                head,
                options,
                options_login,
                login,
                register,
                options_register,
                all_users,
                options_all,
                delete_user,
                add_drink
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
