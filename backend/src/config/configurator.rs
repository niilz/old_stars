use backend::{
    model::{login_data::LoginData, role::OldStarsRole},
    repository::connection::OldStarDb,
    service::user_service::{DbUserService, UserService},
};
use clap::Parser;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{
    env::{self},
    process::exit,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    user: String,
    #[arg(short, long)]
    password: String,
    #[arg(short, long)]
    role: OldStarsRole,
    #[arg(short, long)]
    db_url: Option<String>,
}

fn main() {
    let db_url = env::var("DATABASE_URL");
    let args = Args::parse();
    let db_url = match db_url {
        Ok(url) => {
            println!("Using DATABASE_URL environment variable");
            url
        }
        Err(_) => {
            if args.db_url.is_none() {
                eprintln!("please provide a DATABASE_URL as environment variable or as specify with '--db_url'. And please provide user-name and password");
                exit(1);
            }
            println!("Using database-url-argument");
            let url = args.db_url.as_ref().unwrap().to_string();
            url
        }
    };
    configure(&db_url, args);
}

fn configure(url: &str, args: Args) {
    let user_name = args.user;
    println!("the user {}", user_name);
    let user_pwd = args.password;
    println!("the pwd {}", user_pwd);

    let login_data = LoginData {
        name: user_name,
        pwd: user_pwd,
    };

    println!("trying to establish connection to url: {}", url);
    let db = OldStarDb::with_url(url);

    let mut conn: diesel::pg::PgConnection = db.connection();
    let mut user_service = DbUserService { db };
    run_migration(&mut conn);

    match args.role {
        OldStarsRole::Admin => {
            let _ = user_service.insert_with_role(&login_data, OldStarsRole::Admin);
        }
        OldStarsRole::Club => {
            let _ = user_service.insert_with_role(&login_data, OldStarsRole::Club);
        }
        OldStarsRole::User => {
            let _ = user_service.insert_user(&login_data);
        }
    }
}

fn run_migration(conn: &mut impl MigrationHarness<diesel::pg::Pg>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}
