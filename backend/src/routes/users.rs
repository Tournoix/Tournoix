use diesel::prelude::*;
use rocket::http::Status;
use crate::schema::users;
use crate::models::user::UserInfo;
use rocket::serde::json::Json;

use crate::MysqlConnection;

#[get("/users/<id>")]
pub async fn get_user(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<UserInfo>, (Status, String)> {
    match connection.run(
        move |c| users::table.select((users::id, users::name, users::email)
    ).find(id).first::<UserInfo>(c)).await.map(Json) {
        Ok(user) => {
           return Ok(user)
        },

        Err(_e) => {
            return Err((
                Status::NotFound,
                "user not found".to_string()
            ))
        }
    }
}
