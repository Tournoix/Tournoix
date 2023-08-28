use std::path::{PathBuf, Path};

use dotenv::dotenv;
use rocket::{fs::NamedFile, response::status::NotFound};
use diesel::prelude::*;
// use rocket::serde::json::Json;
use self::models::*;
// use self::schema::user::dsl::*;

mod api;
mod database;
mod models;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build().mount("/", routes![index, static_file])
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