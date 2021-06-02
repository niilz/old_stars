#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel::pg::PgConnection;
use diesel::prelude::*;
//use dotenv::dotenv;
use rocket::http::{hyper::header::AccessControlAllowOrigin, ContentType};
use rocket::Response;
use std::env;

diesel_migrations::embed_migrations!();

#[get("/")]
fn hello() -> Response<'static> {
    Response::build()
        .header(ContentType::Plain)
        .header(AccessControlAllowOrigin::Value(
            "http://localhost:3000".to_string(),
        ))
        .finalize()
}

fn main() {
    //dotenv().ok();
    let connection = establish_connection();
    embedded_migrations::run(&connection);

    rocket::ignite().mount("/", routes![hello]).launch();
}

fn establish_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("Could not read DATABASE_URL from env");
    PgConnection::establish(&db_url).expect("Could not establish_connection()")
}
