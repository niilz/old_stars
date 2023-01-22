#[macro_use]
extern crate rocket;

use backend::{
    model::{app_user::AppUser, login_data::LoginData},
    repository::connection::OldStarDb,
    service::{
        auth_service::LoginService,
        user_service::{DbUserService, UserService},
    },
    SessionResponse,
};
use rocket::{
    config::Config,
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    request::{FromRequest, Outcome},
    serde::json::Json,
    Request, Response, State,
};

use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};

const FRONT_END_URL_DEV: &'static str = "http://localhost:3000";
const FRONT_END_URL: &'static str = "https://niilz.github.io";
const SESSION_TOKEN_HEADER_NAME: &'static str = "old-star-user-session";

struct SessionToken<'a>(&'a str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionToken<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one(SESSION_TOKEN_HEADER_NAME) {
            Some(token) => {
                println!("Yeah got a token: {}", token);
                Outcome::Success(SessionToken(token))
            }
            None => {
                println!("No Session-Token was in the request");
                Outcome::Failure((Status::Unauthorized, ()))
            }
        }
    }
}

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
    println!("Options got called");
    Json("Options Response")
}

#[get("/start")]
fn start(
    login_service: &State<RwLock<LoginService>>,
    token: SessionToken,
) -> Json<Result<AppUser, &'static str>> {
    match login_service.read().unwrap().get_session_user(token.0) {
        Some(user) => Json(Ok(user)),
        None => Json(Err("No Session found")),
    }
}

#[post("/login", format = "json", data = "<login_data>")]
fn login(
    login_data: Json<LoginData>,
    login_service: &State<RwLock<LoginService>>,
) -> Json<Result<SessionResponse, &'static str>> {
    match login_service
        .write()
        .unwrap()
        .login_user(login_data.into_inner())
    {
        Some(session) => {
            println!("Issueing session token");
            Json(Ok(SessionResponse::new(session.user, session.uuid)))
        }
        None => Json(Err("Login failed")),
    }
}

#[get("/logout")]
fn logout(
    login_service: &State<RwLock<LoginService>>,
    token: SessionToken,
) -> Json<Result<(), &'static str>> {
    let session_removed = login_service.write().unwrap().remove_session(token.0);
    Json(session_removed)
}

#[post("/register", format = "json", data = "<user>")]
fn register(
    user: Json<LoginData>,
    user_service: &State<Arc<dyn UserService + Send + Sync>>,
) -> Json<Result<AppUser, String>> {
    let user = user.into_inner();
    if user.name.is_empty() || user.pwd.is_empty() {
        return Json(Err("'name' and 'pwd' must not be empty".to_string()));
    }
    match user_service.insert_user(user) {
        Ok(user) => Json(Ok(AppUser::from_user(&user))),
        Err(e) => Json(Err(format!("Could not reigster user. Error: {}", e))),
    }
}

#[get("/all", format = "json")]
fn all_users(
    user_service: &State<Arc<dyn UserService + Send + Sync>>,
) -> Json<Result<Vec<AppUser>, String>> {
    println!("Getting all users");
    match user_service.get_users() {
        Ok(users) => Json(Ok(users
            .iter()
            .map(|user| AppUser::from_user(user))
            .collect())),
        Err(e) => Json(Err(format!("Could not get all users. Error: {}", e))),
    }
}

#[delete("/delete/<id>")]
fn delete_user(
    id: i32,
    user_service: &State<Arc<dyn UserService + Send + Sync>>,
) -> Json<Result<AppUser, String>> {
    match user_service.delete_user(id) {
        Ok(user) => Json(Ok(AppUser::from_user(&user))),
        Err(e) => Json(Err(format!(
            "Did NOT delete user with id {}! Error: {}",
            id, e
        ))),
    }
}

#[get("/<drink>/<id>")]
fn add_drink(
    drink: String,
    id: i32,
    user_service: &State<Arc<dyn UserService + Send + Sync>>,
) -> Json<Result<AppUser, String>> {
    let drink_clone = drink.clone();
    match user_service.add_drink_to_user(id, &drink_clone) {
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
        #[cfg(debug)]
        let db_url = env::var("DATABASE_URL").unwrap();
        dbg!("DB-URL: {db_url}");
        let user_service = DbUserService {
            db: OldStarDb::new(),
        };
        let user_service: Arc<dyn UserService + Send + Sync> = Arc::new(user_service);
        let login_service = LoginService {
            user_service: Arc::clone(&user_service),
            sessions: HashMap::new(),
        };
        rocket::custom(config_figment)
            .mount(
                "/",
                routes![
                    hello,
                    head,
                    options,
                    start,
                    login,
                    logout,
                    register,
                    all_users,
                    delete_user,
                    add_drink
                ],
            )
            .manage(Arc::clone(&user_service))
            .manage(RwLock::new(login_service))
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
            //FRONT_END_URL_DEV,
            FRONT_END_URL,
            // This machines IP to allow acces from frontend on local network
            //env::var("LOCAL_IP").unwrap(),
            //"*",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, HEAD, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            format!(
                "Accept, Content-Type, Access-Control-Allow-Origin, {SESSION_TOKEN_HEADER_NAME}"
            ),
        ));
    }
}
