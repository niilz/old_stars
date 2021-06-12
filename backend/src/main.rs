#![feature(proc_macro_hygiene, decl_macro)]
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
use diesel::{pg::PgConnection, prelude::*};
use rocket::http::{hyper::header::AccessControlAllowOrigin, ContentType};
use rocket_contrib::{database, json::Json};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::collections::HashMap;
use std::ops::Deref;
use std::{env, str::FromStr};

diesel_migrations::embed_migrations!();

const FRONT_END_URL: &'static str = "http://localhost:3000/";

#[database("db")]
struct Db(diesel::PgConnection);

#[derive(Responder, Debug)]
enum Body {
    Plain(String),
    Error(String),
    Json(Json<Vec<User>>),
}
#[derive(Responder, Debug)]
struct ApiResponse {
    body: Body,
    header: AccessControlAllowOrigin,
}
impl ApiResponse {
    fn new<'a, T: Deref<Target = str>>(inner: T) -> Self {
        ApiResponse {
            body: Body::Plain(inner.to_string()),
            header: AccessControlAllowOrigin::Value(FRONT_END_URL.to_string()),
        }
    }

    fn new_with_error<'a, T: Deref<Target = str>>(error_msg: T) -> Self {
        ApiResponse {
            body: Body::Error(error_msg.to_string()),
            header: AccessControlAllowOrigin::Value(FRONT_END_URL.to_string()),
        }
    }
    fn new_with_json(json: Json<Vec<User>>) -> Self {
        ApiResponse {
            body: Body::Json(json),
            header: AccessControlAllowOrigin::Value(FRONT_END_URL.to_string()),
        }
    }
}

#[get("/")]
fn hello() -> ApiResponse {
    ApiResponse::new("Hello from the backend-api")
}

#[head("/")]
fn head() -> ApiResponse {
    ApiResponse::new("Head Response")
}

#[options("/")]
fn options() -> ApiResponse {
    ApiResponse::new("Options Response")
}

#[options("/login")]
fn options_login() -> ApiResponse {
    ApiResponse::new("Options Response")
}

#[post("/login", format = "json", data = "<login_data>")]
fn login(login_data: Json<LoginData>, conn: Db) -> ApiResponse {
    let secret_pwd = check_pwd(&*conn, login_data.into_inner());
    // TODO: if Login Successfull add "Set-Cooky" header
    ApiResponse::new(format!("from db! pwd: {}", secret_pwd))
}

#[post("/register", format = "json", data = "<user>")]
fn register(user: Json<LoginData>, conn: Db) -> ApiResponse {
    match insert_user(&conn, user.into_inner()) {
        Ok(amount) => ApiResponse::new(format!("inserted {} user", amount)),
        Err(e) => ApiResponse::new(format!("Did not insert user! Error: {}", e)),
    }
}

#[get("/all", format = "json")]
fn all_users(conn: Db) -> ApiResponse {
    match get_users(&conn) {
        Ok(users) => ApiResponse::new_with_json(Json(users)),
        Err(e) => ApiResponse::new_with_error(format!("Did not insert user! Error: {}", e)),
    }
}

#[get("/delete/<id>")]
fn delete_user(conn: Db, id: i32) -> ApiResponse {
    match delete_user_from_db(&conn, id) {
        Ok(amount) => ApiResponse::new(format!("Deleted {} user. ID was: {}", amount, id)),
        Err(e) => {
            ApiResponse::new_with_error(format!("Did NOT delete user with id {}! Error: {}", id, e))
        }
    }
}

fn main() {
    let connection = establish_connection();
    embedded_migrations::run(&connection);

    let mut cors_options = CorsOptions::default();
    cors_options.allowed_origins = AllowedOrigins::some_exact(&[FRONT_END_URL]);
    cors_options.allowed_headers = AllowedHeaders::some(&["Accept", "Content-Type"]);
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
                all_users
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
