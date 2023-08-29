use diesel::connection;
use rocket::http::Status;
use crate::schema::users;
use crate::models::user::UserInfo;
use rocket::serde::json::Json;
use crate::{MysqlConnection, crypto};

#[post("/login", data = "<data>")]
pub async fn login(
    _connection: MysqlConnection,
    data: String,
) -> Result<String, (Status, String)> {
    match _connection.run(
        move |c| users::table.select((users::id, users::name, users::email, users::password))
            .find(data.email).first::<UserInfo>(c)).await.map(Json) {
        Ok(user) => {
            if crypto::verify_password(&data.password, &user.password) {
                Ok(format!("User: {}", user.id))
            } else {
                Err((Status::Unauthorized, "Invalid login".into()))
            }
        },

        Err(e) => {
            Err((Status::Unauthorized, "Invalid login".into()))
        }
    }
}