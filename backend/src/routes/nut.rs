use crate::models::nut::{Nut, PatchNut};
use crate::schema::{games, bets};
use crate::schema::nuts::{self, fk_tournaments, fk_users};
use crate::MysqlConnection;
use diesel::dsl::sum;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

// get the nut of a user for a tournament
#[get("/tournoix/<id>/user/<id_user>/nut")]
pub async fn get_nut(
    connection: MysqlConnection,
    id: i32,
    id_user: i32,
) -> Result<Json<Nut>, (Status, String)> {
    match connection
        .run(move |c| {
            nuts::table
                .filter(fk_tournaments.eq(id))
                .filter(fk_users.eq(id_user))
                .first::<Nut>(c)
        })
        .await
    {
        Ok(mut nut) => {
            // add the nut placed on bet open
            let bets = match connection
                .run(move |c| {
                    bets::table
                        .inner_join(games::table.on(games::id.eq(bets::fk_games)))
                        .filter(bets::fk_nuts.eq(nut.id))
                        .filter(games::is_open.eq(true))
                        .select(sum(bets::nb_nut))
                        .first::<Option<i64>>(c)
                })
                .await
            {
                Ok(bets) => bets,
                Err(_e) => return Err((Status::NotFound, "Bets not found".to_string())),
            };

            // add the nut placed on bet open to the stock of the nut
            nut.stock += bets.unwrap_or(0) as i32;

            return Ok(Json(nut));
        }
        Err(_e) => return Err((Status::NotFound, "Nuts not found".to_string())),
    }
}

#[patch("/nut/<id>", data = "<data>")]
pub async fn update_nut(
    connection: MysqlConnection,
    id: i32,
    data: Json<PatchNut>,
) -> Result<Json<Nut>, (Status, String)> {
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
