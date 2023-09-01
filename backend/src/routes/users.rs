use crate::models::user::UserInfo;
use crate::schema::users;
use crate::{routes::auth::ApiAuth, ErrorBody};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::{ErrorResponse, MysqlConnection};

#[get("/users/<id>")]
pub async fn get_user(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<UserInfo>, (Status, Json<ErrorResponse>)> {
    if auth.user.id != id {
        return Err((
            Status::Forbidden,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: 403,
                    description: "Access Forbidden".into(),
                },
            }),
        ));
    }

    match connection
        .run(move |c| {
            users::table
                .select((users::id, users::name, users::email))
                .find(id)
                .first::<UserInfo>(c)
        })
        .await
        .map(Json)
    {
        Ok(user) => return Ok(user),

        Err(_e) => {
            return Err((
                Status::NotFound,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 404,
                        description: "user not found".into(),
                    },
                }),
            ))
        }
    }
}

#[get("/users/@me")]
pub async fn get_current_user(auth: ApiAuth) -> Result<Json<UserInfo>, (Status, String)> {
    Ok(Json(auth.user))
}
