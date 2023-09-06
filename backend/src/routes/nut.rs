use crate::models::nut::{Nut, PatchNut};
use crate::models::tournament::Tournament;
use crate::routes::auth::ApiAuth;
use crate::schema::nuts::{self, fk_tournaments, fk_users};
use crate::schema::tournaments;
use crate::{ErrorBody, ErrorResponse, MysqlConnection};
use diesel::prelude::*;
use log::warn;
use rocket::http::Status;
use rocket::serde::json::Json;

// get the nut of a user for a tournament
#[get("/tournoix/<id>/nut")]
pub async fn get_nut(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<Nut>, (Status, Json<ErrorResponse>)> {
    match connection
        .run(move |c| {
            nuts::table
                .filter(fk_tournaments.eq(id))
                .filter(fk_users.eq(auth.user.id))
                .first::<Nut>(c)
        })
        .await
    {
        Ok(nut) => Ok(Json(nut)),
        Err(_e) => {
            return Err((
                Status::NotFound,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 404,
                        reason: "Not Found".into(),
                        description: "Nuts not found".into(),
                    },
                }),
            ))
        }
    }
}

#[patch("/nut/<id>", data = "<data>")]
pub async fn update_nut(
    connection: MysqlConnection,
    id: i32,
    data: Json<PatchNut>,
    auth: ApiAuth,
) -> Result<Json<Nut>, (Status, String)> {
    // Check if the caller is an owner of the tournament
    match connection
        .run(move |c| {
            tournaments::table
                .filter(tournaments::id.eq(id))
                .filter(tournaments::fk_users.eq(auth.user.id))
                .first::<Tournament>(c)
        })
        .await
    {
        Ok(_) => (),
        Err(_) => {
            warn!(
                "User {} tried to update the nut of tournament {} - routes/nut/update_nut()",
                auth.user.id, id
            );
            return Err((Status::Forbidden, "Access Forbidden".to_string()));
        }
    };

    match connection
        .run(move |c| {
            c.transaction(|c| {
                diesel::update(nuts::table.find(id))
                    .set(data.0)
                    .execute(c)?;

                let nut = nuts::table
                    .order(nuts::id.desc())
                    .first::<Nut>(c)
                    .map(Json)?;

                diesel::result::QueryResult::Ok(nut)
            })
        })
        .await
    {
        Ok(nut) => {
            return Ok(nut);
        }

        Err(_e) => return Err((Status::InternalServerError, "No nuts found".to_string())),
    }
}
