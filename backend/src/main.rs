#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use backend::db::auth_service::*;
use backend::model::login_data::LoginData;
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

#[derive(Responder)]
struct BaseResponder {
    inner: String,
    header: AccessControlAllowOrigin,
}
impl BaseResponder {
    fn new<'a, T: Deref<Target = str>>(inner: T) -> Self {
        BaseResponder {
            inner: inner.to_string(),
            header: AccessControlAllowOrigin::Value(FRONT_END_URL.to_string()),
        }
    }
}

#[get("/")]
fn hello() -> BaseResponder {
    BaseResponder::new("Hello from the backend-api")
}

#[head("/")]
fn head() -> BaseResponder {
    BaseResponder::new("Head Response")
}

#[options("/")]
fn options() -> BaseResponder {
    BaseResponder::new("Options Response")
}

#[options("/login")]
fn options_login() -> BaseResponder {
    BaseResponder::new("Options Response")
}

#[post("/login", format = "json", data = "<login_data>")]
fn login(login_data: Json<LoginData>, conn: Db) -> BaseResponder {
    let secret_pwd = check_pwd(&*conn, login_data.into_inner());
    // TODO: if Login Successfull add "Set-Cooky" header
    BaseResponder::new(format!("from db! pwd: {}", secret_pwd))
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
        .mount("/", routes![hello, head, options, options_login, login])
        .attach(cors_options.to_cors().unwrap())
        .attach(Db::fairing())
        .launch();
}

fn establish_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("Could not read DATABASE_URL from env");
    PgConnection::establish(&db_url).expect("Could not establish_connection()")
}
