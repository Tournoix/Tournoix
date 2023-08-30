use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::MysqlConnection;
use crate::models::tournament::{Tournament, NewTournament, PatchTournament};
use crate::schema::tournaments;

#[get("/tournoix/<id>")]
pub async fn get_tournoix(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Tournament>, (Status, String)> {
    match connection.run(
        move |c| tournaments::table.find(id).first::<Tournament>(c)
    ).await.map(Json) {
        Ok(tournoi) => {
           return Ok(tournoi)
        },

        Err(_e) => {
            return Err((
                Status::NotFound,
                "Tournament not found".to_string()
            ))
        }
    }
}

#[post("/tournoix", data = "<data>")]
pub async fn create_tournoix(
    connection: MysqlConnection,
    data: Json<NewTournament>,
) -> Result<Json<Tournament>, (Status, String)> {
    let tournoix = data.0;

    match connection.run(
       move |c| c.transaction(|c| {
            diesel::insert_into(tournaments::table)
                .values(tournoix.clone())
                .execute(c)?;

            let tournoix = tournaments::table.order(tournaments::id.desc()).first::<Tournament>(c).map(Json)?;

            diesel::result::QueryResult::Ok(tournoix)
        })
    ).await {
        Ok(tournoix) => {
            return Ok(tournoix);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string()
            ))
        }
    }
}

#[patch("/tournoix/<id>", data = "<data>")]
pub async fn update_tournoix(
    connection: MysqlConnection,
    data: Json<PatchTournament>,
    id: i32
) -> Result<Json<Tournament>, (Status, String)> {
    let tournoix = data.0;

    match connection.run(
       move |c| c.transaction(|c| {
            diesel::update(tournaments::table.find(id))
                .set(tournoix.clone())
                .execute(c)?;

            let tournoix = tournaments::table.order(tournaments::id.desc()).first::<Tournament>(c).map(Json)?;

            diesel::result::QueryResult::Ok(tournoix)
        })
    ).await {
        Ok(tournoix) => {
            return Ok(tournoix);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string()
            ))
        }
    }
}

#[delete("/tournoix/<id>")]
pub async fn delete_tournoix(
    connection: MysqlConnection,
    id: i32
) -> Result<Json<Tournament>, (Status, String)> {
    match connection.run(
       move |c| c.transaction(|c| {
            let tournoix = tournaments::table.find(id).first::<Tournament>(c).map(Json)?;

            diesel::delete(tournaments::table.find(id)).execute(c)?;

            diesel::result::QueryResult::Ok(tournoix)
        })
    ).await {
        Ok(tournoix) => {
            return Ok(tournoix);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string()
            ))
        }
    }
}