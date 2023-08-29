#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use std::path::{Path, PathBuf};

use dotenv::dotenv;
use rocket::{fs::NamedFile, http::Status, response::status::NotFound, serde::json::Json};
use rocket_sync_db_pools::database;
use routes::users::get_user;
use serde::Serialize;

mod routes;
mod tests;
mod models;
mod schema;
pub mod crypto;

#[database("tournoix_db")]
pub struct MysqlConnection(diesel::MysqlConnection);

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
        .mount("/", routes![index, static_file])
        .mount("/api", routes![get_user, api_hole])
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
