#[macro_use]
extern crate rocket;

use backend::db::auth_service::*;
use backend::db::user_service::*;
use backend::model::app_user::AppUser;
use backend::model::login_data::LoginData;
use diesel::{pg::PgConnection, Connection};
use rocket::{
    config::Config,
    fairing::{Fairing, Info, Kind},
    figment::util::map,
    http::Header,
    serde::json::Json,
    Request, Response,
};
use rocket_sync_db_pools::{database, diesel};
use std::env;

#[database("db")]
pub struct Db(diesel::PgConnection);

const FRONT_END_URL_DEV: &'static str = "http://localhost:3000/";
const FRONT_END_URL: &'static str = "https://niilz.github.io/old_stars/";
const FRONT_END_URL_HACK: &'static str = "https://oldstars.ngrok.io/";

#[get("/")]
async fn hello() -> Json<&'static str> {
    Json("Hello from the backend-api. More changes")
}

#[head("/")]
async fn head() -> Json<&'static str> {
    Json("Head Response")
}

#[options("/<_..>")]
async fn options() -> Json<&'static str> {
    Json("Options Response")
}

#[post("/login", format = "json", data = "<login_data>")]
async fn login(login_data: Json<LoginData>, conn: Db) -> Json<Result<AppUser, &'static str>> {
    match conn.run(|c| login_user(c, login_data.into_inner())).await {
        // TODO: if Login Successfull add "Set-Cooky" header
        Some(user) => Json(Ok(user)),
        None => Json(Err("Login failed")),
    }
}

#[post("/register", format = "json", data = "<user>")]
async fn register(user: Json<LoginData>, conn: Db) -> Json<Result<AppUser, String>> {
    let user = user.into_inner();
    if user.name.is_empty() || user.pwd.is_empty() {
        eprintln!("user is empty");
        return Json(Err("'name' and 'pwd' must not be empty".to_string()));
    }
    match conn.run(|c| insert_user(c, user)).await {
        Ok(user) => Json(Ok(AppUser::from_user(&user))),
        Err(e) => Json(Err(format!("Could not reigster user. Error: {}", e))),
    }
}

#[get("/all", format = "json")]
async fn all_users(conn: Db) -> Json<Result<Vec<AppUser>, String>> {
    match conn.run(|c| get_users(c)).await {
        Ok(users) => Json(Ok(users
            .iter()
            .map(|user| AppUser::from_user(user))
            .collect())),
        Err(e) => Json(Err(format!("Could not get all users. Error: {}", e))),
    }
}

#[delete("/delete/<id>")]
async fn delete_user(conn: Db, id: i32) -> Json<Result<AppUser, String>> {
    match conn.run(move |c| delete_user_from_db(c, id)).await {
        Ok(user) => Json(Ok(AppUser::from_user(&user))),
        Err(e) => Json(Err(format!(
            "Did NOT delete user with id {}! Error: {}",
            id, e
        ))),
    }
}

#[get("/<drink>/<id>")]
async fn add_drink(conn: Db, drink: String, id: i32) -> Json<Result<AppUser, String>> {
    let drink_clone = drink.clone();
    match conn
        .run(move |c| add_drink_to_user(c, id, &drink_clone))
        .await
    {
        Ok(updated_user) => Json(Ok(AppUser::from_user(&updated_user))),
        Err(e) => Json(Err(format!(
            "Could not add a {} to user with id {}. Error: {}",
            drink, id, e
        ))),
    }
}

#[launch]
fn rocket() -> _ {
    let cert_chain = env::var("CERT_CHAIN");
    let private_key = env::var("PRIVATE_KEY");

    let config_figment = Config::figment();

    let config_figment = if let (Ok(cert), Ok(pk)) = (cert_chain, private_key) {
        println!("Found certs, using TLS");
        config_figment
            .merge(("tls.certs", &cert))
            .merge(("tls.key", &pk))
            .merge(("port", 1443))
    } else {
        println!("No TLS-configuration found");
        config_figment.merge(("port", 8000))
    };

    // Only attach the db-related routes if db is not disabled
    let no_db_value = String::from("1");
    let rocket = if env::var("NO_DB") == Ok(no_db_value) {
        println!("Running without DB");
        rocket::custom(config_figment).mount("/", routes![hello, head, options])
    } else {
        println!("Setting DB-Config");
        let db_url = env::var("DATABASE_URL").unwrap();
        println!("DB-URL: {db_url}");
        let db_config = map! { "url" => db_url };
        let config_figment_with_db = config_figment.merge(("databases", map!["db" => db_config]));
        println!("Configuring Rocket");
        rocket::custom(config_figment_with_db)
            .mount(
                "/",
                routes![
                    hello,
                    head,
                    options,
                    login,
                    register,
                    all_users,
                    delete_user,
                    add_drink
                ],
            )
            .attach(Db::fairing())
    };

    println!("Launching rocket");
    rocket.attach(Cors)
}

struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cors-Information",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            //[FRONT_END_URL, FRONT_END_URL_DEV, FRONT_END_URL_HACK].join(", "),
            "*",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, HEAD, OPTIONS, DELETE",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Accept, Content-Type, Access-Control-Allow-Origin",
        ));
    }
}
