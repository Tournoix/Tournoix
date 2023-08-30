use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::MysqlConnection;
use crate::models::team::*;
use crate::schema::teams::fk_tournaments;
use crate::schema::teams;
use rocket::serde::{Deserialize, Serialize};

#[get("/tournoix/<id>/team")]
pub async fn get_teams(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Vec<Team>>, (Status, String)> {
    match connection.run(
        move |c| teams::table.filter(fk_tournaments.eq(id)).load::<Team>(c)
    ).await.map(Json) {
        Ok(teams) => {
           return Ok(teams)
        },

        Err(_e) => {
            return Err((
                Status::NotFound,
                "No teams found".to_string()
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddTeam {
    pub name: String,
    pub group: i32,
}

#[post("/tournoix/<id>/team", data = "<data>")]
pub async fn create_team(
    connection: MysqlConnection,
    data: Json<AddTeam>,
    id: i32,
) -> Result<Json<Team>, (Status, String)> {
    let team = NewTeam {
        fk_tournaments: id,
        name: data.0.name,
        group: data.0.group,
    };

    match connection.run(
       move |c| c.transaction(|c| {
            diesel::insert_into(teams::table)
                .values(team.clone())
                .execute(c)?;

            let team = teams::table.order(teams::id.desc()).first::<Team>(c).map(Json)?;

            diesel::result::QueryResult::Ok(team)
        })
    ).await {
        Ok(team) => {
            return Ok(team);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string()
            ))
        }
    }
}

#[patch("/tournoix/<id>/team", data = "<data>")]
pub async fn update_team(
    connection: MysqlConnection,
    data: Json<PatchTeam>,
    id: i32
) -> Result<Json<Team>, (Status, String)> {
    let team = data.0;

    match connection.run(
       move |c| c.transaction(|c| {
            diesel::update(teams::table)
                .filter(teams::id.eq(id))
                .set(team.clone())
                .execute(c)?;

            let team = teams::table.order(teams::id.desc()).first::<Team>(c).map(Json)?;

            diesel::result::QueryResult::Ok(team)
        })
    ).await {
        Ok(team) => {
            return Ok(team);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string()
            ))
        }
    }
}

#[delete("/team/<id>")]
pub async fn delete_team(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Team>, (Status, String)> {
    match connection.run(
       move |c| c.transaction(|c| {
            let team = teams::table.find(id).first::<Team>(c).map(Json)?;

            diesel::delete(teams::table.find(id)).execute(c)?;

            diesel::result::QueryResult::Ok(team)
        })
    ).await {
        Ok(team) => {
            return Ok(team);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string()
            ))
        }
    }
}