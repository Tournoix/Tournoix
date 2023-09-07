#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use std::path::{Path, PathBuf};

use dotenv::dotenv;
use rocket::{
    fairing::{Fairing, Info, Kind},
    fs::NamedFile,
    http::{Header, Status},
    response::status::{NotFound, NoContent},
    serde::json::Json,
    Request, Response,
};
use rocket_sync_db_pools::database;
use routes::{
    auth::{login, logout, register},
    users::{get_current_user, get_user},
};
use serde::Serialize;

use crate::routes::game::*;
use crate::routes::nut::*;
use crate::routes::subscription::*;
use crate::routes::team::*;
use crate::routes::tournoix::*;
use crate::routes::bet::*;

use log::{error, info, trace, warn};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

pub mod crypto;
mod models;
mod routes;
mod schema;
mod tests;

#[database("tournoix_db")]
pub struct MysqlConnection(diesel::MysqlConnection);

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Serialize)]
pub struct EmptyResponse();

#[derive(Serialize)]
pub struct ErrorBody {
    pub code: i32,
    pub reason: String,
    pub description: String,
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).unwrap();

    rocket::build()
        .attach(MysqlConnection::fairing())
        .attach(CORS)
        .mount("/", routes![index, static_file])
        .mount(
            "/api",
            routes![
                // Users
                get_user,
                get_user_tournoix,
                get_user_subscription,
                get_current_user,
                // Tournoix
                get_tournoix,
                create_tournoix,
                update_tournoix,
                delete_tournoix,
                // Teams
                get_teams,
                create_team,
                update_team,
                delete_team,
                // Subscriptions
                create_subsciption,
                delete_subscription,
                // Auth
                login,
                logout,
                register,
                // games
                get_tournoix_game,
                get_team_game,
                get_game,
                create_games,
                update_game,
                close_game,
                close_game_betting,
                remove_all_games,
                // Nuts
                get_nut,
                update_nut,
                // Bets
                get_game_bet,
                get_user_game_bet,
                get_user_game_bet_result,
                create_bet,
                update_bet,
                delete_bet,
                // Others
                all_options,
                api_hole,
            ],
        )
}

#[options("/<_..>")]
fn all_options() -> NoContent {
	NoContent
}

async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("./public/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

// Serve Yew app
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[get("/<path..>", rank = 3)]
async fn static_file(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    match NamedFile::open(Path::new("./public").join(path)).await {
        Ok(file) => Ok(file),
        Err(_) => get_index().await,
    }
}

#[get("/<_path..>", rank = 2)]
async fn api_hole(_path: PathBuf) -> (Status, Json<ErrorResponse>) {
    (
        Status::NotFound,
        Json(ErrorResponse {
            error: ErrorBody {
                code: 404,
                reason: "Not Found".into(),
                description: "There is nothing here".to_string(),
            },
        }),
    )
}
