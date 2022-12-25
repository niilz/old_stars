//#[macro_use]
//extern crate diesel_migrations;

use diesel::connection::Connection;
use diesel::PgConnection;
use std::env;

//embed_migrations!();

pub struct OldStarDb {
    db_url: String,
}

impl OldStarDb {
    pub fn new() -> Self {
        // Only attach the db-related routes if db is not disabled
        println!("Setting DB-Config");
        let db_url = env::var("DATABASE_URL").expect("Database-Url is not set as env-var");
        println!("DB-URL: {db_url:?}");

        Self { db_url }
    }

    pub fn conntection(&self) -> PgConnection {
        PgConnection::establish(&self.db_url).expect("Could not establish connection")
    }
}

/*
let _ = embedded_migrations::run(&conn);
let _ = user_service::insert_user(&conn, login_data);
*/
