#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;

use std::path::{Path, PathBuf};

use dotenv::dotenv;
use models::{user::{UserInfo}, token::Token};
use rocket::{fs::NamedFile, response::status::NotFound, http::Status, serde::json::Json, request::{FromRequest, Outcome, self}, Request};
use rocket_sync_db_pools::database;
use routes::{users::{get_user, get_current_user}, auth::{login, logout, register}};
use schema::{tokens, users};
use serde::Serialize;

use crate::routes::tournoix::*;
use crate::routes::team::*;

mod routes;
mod tests;
mod models;
mod schema;
pub mod crypto;

#[database("tournoix_db")]
pub struct MysqlConnection(diesel::MysqlConnection);

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: ErrorBody
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub code: i32,
    pub description: String,
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(MysqlConnection::fairing())
        .mount("/", routes![index, static_file])
        .mount("/api", routes![get_user, get_tournoix, create_tournoix, update_tournoix, delete_tournoix, get_teams, create_team, update_team, delete_team, api_hole, login, logout, register, get_current_user])
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
                description: "There is nothing here".to_string(),
            }
        }),
    )
}
