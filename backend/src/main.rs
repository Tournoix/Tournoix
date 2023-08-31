#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use std::path::{Path, PathBuf};

use dotenv::dotenv;
use rocket::{fs::NamedFile, response::status::NotFound, http::{Status, Header}, serde::json::Json, fairing::{Fairing, Info, Kind}, Request, Response};
use rocket_sync_db_pools::database;
use routes::{auth::*, tournoix::*, team::*, game::get_team_game};
use serde::Serialize;

use crate::routes::users::get_user;
use crate::routes::subscription::*;
use crate::routes::game::*;

mod routes;
mod tests;
mod models;
mod schema;
pub mod crypto;

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
    pub error: i32,
    pub message: String,
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(MysqlConnection::fairing())
        .attach(CORS)
        .mount("/", routes![index, static_file])
        .mount("/api", routes![get_user, 
            get_tournoix, create_tournoix, update_tournoix, delete_tournoix, 
            get_teams, create_team, update_team, delete_team, 
            get_user_tournoix, get_user_subscription, create_subsciption, delete_subscription,
            api_hole,
            login, logout, register,
            get_tournoix_game, get_team_game, create_games, update_game])
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
            error: 404,
            message: "There is nothing here".to_string(),
        }),
    )
}
