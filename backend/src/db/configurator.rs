#[macro_use]
extern crate diesel_migrations;

use backend::{
    db::connection::OldStarDb,
    model::login_data::LoginData,
    service::user_service::{DbUserService, UserService},
};
use std::{
    env::{self, Args},
    process::exit,
};

embed_migrations!();

fn main() {
    let db_url = env::var("DATABASE_URL");
    let mut args = env::args();
    let (db_url, mut args) = match db_url {
        Ok(url) => {
            println!("Using DATABASE_URL environment variable");
            args.next();
            (url, args)
        }
        Err(_) => {
            if args.len() < 4 {
                eprintln!("please provide a DATABASE_URL as environment variable or as first argument. And please provide user-name and password");
                exit(1);
            }
            println!("Using database-url-argument");
            let url = args.nth(1).unwrap();
            (url, args)
        }
    };
    configure(&db_url, &mut args);
}

fn configure(url: &str, args: &mut Args) {
    if args.len() != 2 {
        eprintln!("please provide user-name and user-password as cli-args");
        exit(1)
    }

    let user_name = args.nth(0).unwrap();
    println!("the user {}", user_name);
    let user_pwd = args.nth(0).unwrap();
    println!("the pwd {}", user_pwd);

    let login_data = LoginData {
        name: user_name,
        pwd: user_pwd,
    };

    println!("trying to establish connection to url: {}", url);
    let db = OldStarDb::with_url(url);
    let conn = db.conntection();
    let user_service = DbUserService { db };
    let _ = embedded_migrations::run(&conn);
    let _ = user_service.insert_user(login_data);
}
