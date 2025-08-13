#[macro_use]
extern crate rocket;

use backend::{
    SessionResponse,
    model::{app_user::AppUser, history::History, login_data::LoginData, role::OldStarsRole},
    repository::connection::OldStarDb,
    service::{
        auth_service::LoginService,
        consumption_service::{ConsumptionService, DbConsumptionRepo},
        history_service::{DbHistoryRepo, HistoryService},
        user_service::{DbUserService, UserService},
    },
};
use rocket::{
    Build, Request, Response, Rocket, State,
    config::Config,
    fairing::{Fairing, Info, Kind},
    figment::Figment,
    http::{Header, Status},
    request::{FromRequest, Outcome},
    serde::json::Json,
};

use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex, RwLock},
};

const _FRONT_END_URL_DEV: &str = "http://localhost:3000";
const FRONT_END_URL: &str = "https://www.niilz.com";
const CLUB_TOKEN_HEADER_NAME: &str = "oldstars-club-token";
const SESSION_TOKEN_HEADER_NAME: &str = "old-star-user-session";

struct ClubToken<'a>(&'a str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClubToken<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one(CLUB_TOKEN_HEADER_NAME) {
            Some(token) => {
                println!("Yeah got a club-token: {}", token);
                Outcome::Success(ClubToken(token))
            }
            None => {
                println!("No Session-Token was in the request");
                Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}

struct SessionToken<'a>(&'a str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionToken<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one(SESSION_TOKEN_HEADER_NAME) {
            Some(token) => {
                println!("Yeah got a session-token: {}", token);
                Outcome::Success(SessionToken(token))
            }
            None => {
                println!("No Session-Token was in the request");
                Outcome::Error((Status::Unauthorized, ()))
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

// Unauthorized (everyone can try to get a club-token)
#[post("/club/login", format = "json", data = "<club_pwd>")]
fn club_login(
    club_pwd: Json<LoginData>,
    login_service: &State<RwLock<LoginService>>,
) -> Json<Result<String, &'static str>> {
    println!("club-login got called");
    match login_service.write().unwrap().login_club(&club_pwd.pwd) {
        Some(club_session) => {
            println!("Issueing club token");
            Json(Ok(club_session.uuid))
        }
        None => Json(Err("Login failed")),
    }
}

// Club-Authorization required
#[get("/club/access", format = "json")]
fn has_club_access(
    club_token: ClubToken,
    login_service: &State<RwLock<LoginService>>,
) -> Json<bool> {
    println!("has_club_access got called");
    Json(login_service.read().unwrap().has_club_access(&club_token.0))
}

#[post("/login", format = "json", data = "<login_data>")]
fn login(
    club_token: ClubToken,
    login_data: Json<LoginData>,
    login_service: &State<RwLock<LoginService>>,
) -> Json<Result<SessionResponse, &'static str>> {
    println!("login got called");
    if !login_service.read().unwrap().has_club_access(&club_token.0) {
        return Json(Err("club-acces missing"));
    }
    println!("Has Club-Access");
    match login_service
        .write()
        .unwrap()
        .login_user(&login_data.into_inner())
    {
        Some(session) => {
            println!("Issueing session token");
            Json(Ok(SessionResponse::new(session.user, session.uuid)))
        }
        None => Json(Err("Login failed")),
    }
}

// User-Authorization required
#[get("/start")]
fn start(
    login_service: &State<RwLock<LoginService>>,
    token: SessionToken,
) -> Json<Result<AppUser, &'static str>> {
    println!("start got called");
    match login_service.read().unwrap().get_session_user(token.0) {
        Some(user) => Json(Ok(user)),
        None => Json(Err("No Session found")),
    }
}

#[get("/logout")]
fn logout(
    login_service: &State<RwLock<LoginService>>,
    token: SessionToken,
) -> Json<Result<(), &'static str>> {
    // Would be better to check that the token actually belongs to the person who wants it's
    // deletion
    let session_removed = login_service.write().unwrap().remove_session(token.0);
    Json(session_removed)
}

#[post("/register", format = "json", data = "<user>")]
fn register(
    club_token: ClubToken,
    user: Json<LoginData>,
    user_service: &State<Arc<Mutex<dyn UserService + Send + Sync>>>,
    login_service: &State<RwLock<LoginService>>,
) -> Json<Result<AppUser, String>> {
    println!("register got called");
    if !login_service.read().unwrap().has_club_access(club_token.0) {
        return Json(Err("Registering requires club access".to_string()));
    }
    println!("Has Club-Access");
    let user = user.into_inner();
    if user.name.is_empty() || user.pwd.is_empty() {
        return Json(Err("'name' and 'pwd' must not be empty".to_string()));
    }
    match user_service.lock().unwrap().insert_user(&user) {
        Ok((user, role)) => Json(Ok(AppUser::from((user, role)))),
        Err(e) => Json(Err(format!("Could not reigster user. Error: {}", e))),
    }
}

#[get("/user/<name>", format = "json")]
fn user(
    name: &str,
    token: SessionToken,
    login_service: &State<RwLock<LoginService>>,
    user_service: &State<Arc<Mutex<dyn UserService + Send + Sync>>>,
) -> Json<Result<AppUser, String>> {
    println!("get-user got called");
    if !login_service
        .read()
        .unwrap()
        .get_session_user(token.0)
        .is_some()
    {
        return Json(Err("No valid session".to_string()));
    }
    println!("Getting user with id: {name}");
    match user_service.lock().unwrap().get_user_and_role(name) {
        Ok(user_and_role) => Json(Ok(AppUser::from(user_and_role))),
        Err(e) => Json(Err(format!("Could not get user. Error: {}", e))),
    }
}

#[get("/all", format = "json")]
fn all_users(
    token: SessionToken,
    login_service: &State<RwLock<LoginService>>,
    user_service: &State<Arc<Mutex<dyn UserService + Send + Sync>>>,
) -> Json<Result<Vec<AppUser>, String>> {
    println!("all_users got called");
    if !login_service
        .read()
        .unwrap()
        .get_session_user(token.0)
        .is_some()
    {
        return Json(Err("No valid session".to_string()));
    }
    println!("Getting all users");
    match user_service.lock().unwrap().get_users_and_roles() {
        Ok(users_with_roles) => Json(Ok(users_with_roles
            .into_iter()
            .map(|(user, role)| AppUser::from((user, role)))
            .collect())),
        Err(e) => Json(Err(format!("Could not get all users. Error: {}", e))),
    }
}

#[get("/admin", format = "json")]
fn is_admin(token: SessionToken, login_service: &State<RwLock<LoginService>>) -> Json<bool> {
    println!("is-admin got called");
    Json(login_service.read().unwrap().is_admin(&token.0))
}

#[delete("/delete/<id>")]
fn delete_user(
    id: i32,
    user_service: &State<Arc<Mutex<dyn UserService + Send + Sync>>>,
    login_service: &State<RwLock<LoginService>>,
    token: SessionToken,
) -> Json<Result<(), String>> {
    println!("delete-user got called");

    // Check that user is admin
    match login_service.read().unwrap().get_session_user(token.0) {
        Some(user) if user.role == OldStarsRole::Admin => user,
        Some(_) => return Json(Err("Only Admins are allowed to delete".to_string())),
        _ => {
            return Json(Err("No valid session".to_string()));
        }
    };

    match user_service.lock().unwrap().delete_user(id) {
        Ok(_user) => {
            let _session_delete_result = login_service.write().unwrap().remove_user_session(id);
            Json(Ok(()))
        }
        Err(e) => Json(Err(format!(
            "Did NOT delete user with id {}! Error: {}",
            id, e
        ))),
    }
}

#[get("/<consumption>/<user_id>")]
fn add_consumption(
    consumption: String,
    user_id: i32,
    db_conn: &State<OldStarDb>,
    consumption_service: &State<RwLock<ConsumptionService<DbConsumptionRepo>>>,
    login_service: &State<RwLock<LoginService>>,
    token: SessionToken,
) -> Json<Result<AppUser, String>> {
    println!("add-consumption '{consumption}' got called");
    if !login_service
        .read()
        .unwrap()
        .get_session_user(token.0)
        .is_some()
    {
        return Json(Err("No valid session".to_string()));
    }

    let consumption_clone = consumption.clone();
    match consumption_service
        .write()
        .unwrap()
        .add_consumption_to_user(user_id, &consumption_clone, &mut db_conn.connection())
    {
        Ok(updated_user) => Json(Ok(AppUser::from((updated_user, OldStarsRole::User)))),
        Err(e) => Json(Err(format!(
            "Could not add a {} to user with id {}. Error: {}",
            consumption, user_id, e
        ))),
    }
}

#[get("/historize")]
fn historize(
    db_conn: &State<OldStarDb>,
    history_service: &State<RwLock<HistoryService<DbHistoryRepo>>>,
    login_service: &State<RwLock<LoginService>>,
    token: SessionToken,
) -> Json<Result<Vec<History>, String>> {
    println!("historize got called");
    // Check that user is admin
    match login_service.read().unwrap().get_session_user(token.0) {
        Some(user) if user.role == OldStarsRole::Admin => user,
        Some(_) => return Json(Err("Only Admins are allowed to historize".to_string())),
        _ => {
            return Json(Err("No valid session".to_string()));
        }
    };

    match history_service
        .write()
        .unwrap()
        .historize_consumptions(&mut db_conn.connection())
    {
        Ok(histories) => Json(Ok(histories)),
        Err(e) => Json(Err(format!("Could not historize consumptions: {e}"))),
    }
}

#[post("/history", data = "<csv>")]
fn history_from_csv(
    db_conn: &State<OldStarDb>,
    history_service: &State<RwLock<HistoryService<DbHistoryRepo>>>,
    login_service: &State<RwLock<LoginService>>,
    token: SessionToken,
    csv: &str,
) -> Json<Result<Vec<History>, String>> {
    println!("histories from csv got called");
    // Check that user is admin
    match login_service.read().unwrap().get_session_user(token.0) {
        Some(user) if user.role == OldStarsRole::Admin => user,
        Some(_) => {
            return Json(Err(
                "Only Admins are allowed to insert histories".to_string()
            ));
        }
        _ => {
            return Json(Err("No valid session".to_string()));
        }
    };

    match history_service
        .write()
        .unwrap()
        .histories_from_csv(csv, &mut db_conn.connection())
    {
        Ok(histories) => Json(Ok(histories)),
        Err(e) => Json(Err(format!("Could not insert histories from csv: {e}"))),
    }
}

#[get("/histories")]
fn histories(
    db_conn: &State<OldStarDb>,
    history_service: &State<RwLock<HistoryService<DbHistoryRepo>>>,
    login_service: &State<RwLock<LoginService>>,
    token: SessionToken,
) -> Json<Result<Vec<History>, String>> {
    println!("histories got called");
    if !login_service
        .read()
        .unwrap()
        .get_session_user(token.0)
        .is_some()
    {
        return Json(Err("No valid session".to_string()));
    }

    match history_service
        .read()
        .unwrap()
        .load_histories(&mut db_conn.connection())
    {
        Ok(histories) => Json(Ok(histories)),
        Err(e) => Json(Err(format!("Could not load consumptions histories: {e}"))),
    }
}

#[rocket::main]
async fn main() {
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

    println!("Launching rocket");
    if let Err(e) = rocket(config_figment).launch().await {
        eprintln!("Schade, rocket failed. Err: {e:?}");
        drop(e);
    };
}

fn rocket(config_figment: Figment) -> Rocket<Build> {
    // Only attach the db-related routes if db is not disabled
    let no_db_value = String::from("1");

    if env::var("NO_DB") == Ok(no_db_value) {
        println!("Running without DB");
        rocket::custom(config_figment)
            .mount("/", routes![hello, head, options])
            .attach(Cors)
    } else {
        let db_url = env::var("DATABASE_URL").unwrap();
        dbg!(db_url);
        let user_service = DbUserService {
            db: OldStarDb::new(),
        };
        let user_service: Arc<Mutex<dyn UserService + Send + Sync>> =
            Arc::new(Mutex::new(user_service));
        let login_service = LoginService {
            user_service: Arc::clone(&user_service),
            sessions: HashMap::new(),
            club_sessions: HashMap::new(),
        };
        let db_conn = OldStarDb::new();
        let consumption_service = ConsumptionService::new(DbConsumptionRepo::default());
        let history_service = HistoryService::new(DbHistoryRepo::default());
        rocket::custom(config_figment)
            .mount(
                "/",
                routes![
                    hello,
                    head,
                    options,
                    start,
                    club_login,
                    has_club_access,
                    is_admin,
                    login,
                    logout,
                    register,
                    user,
                    all_users,
                    delete_user,
                    add_consumption,
                    historize,
                    histories,
                    history_from_csv
                ],
            )
            .manage(Arc::clone(&user_service))
            .manage(RwLock::new(login_service))
            .manage(RwLock::new(consumption_service))
            .manage(RwLock::new(history_service))
            .manage(db_conn)
            .attach(Cors)
    }
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
            //_FRONT_END_URL_DEV,
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
                "Accept, Content-Type, Access-Control-Allow-Origin, {SESSION_TOKEN_HEADER_NAME}, {CLUB_TOKEN_HEADER_NAME}"
            ),
        ));
    }
}
