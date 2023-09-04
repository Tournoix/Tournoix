use crate::models::game::Game;
use crate::models::game::*;
use crate::models::tournament::Tournament;
use crate::schema::{games, tournaments};
use crate::MysqlConnection;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use super::bet::calculate_gain;

// get all match from a tournament
#[get("/tournoix/<id>/games")]
pub async fn get_tournoix_game(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Vec<Game>>, (Status, String)> {
    // get all team from a tournament
    let teams = match connection
        .run(move |c| tournaments::table.find(id).load::<Tournament>(c))
        .await
    {
        Ok(teams) => teams,
        Err(_) => {
            return Err((
                Status::NotFound,
                "No team found for the tournament".to_string(),
            ))
        }
    };

    let mut games = Vec::new();

    // get all match from a team
    for team in teams {
        let matchs = match connection
            .run(move |c| {
                games::table
                    .filter(games::fk_team1.eq(team.id))
                    .load::<Game>(c)
            })
            .await
        {
            Ok(matchs) => matchs,
            Err(_) => return Err((Status::NotFound, "Not match found for the tean".to_string())),
        };
        // add game to the vector
        games.extend(matchs);
    }

    Ok(Json(games))
}

// get all match from a team
#[get("/team/<id>/games")]
pub async fn get_team_game(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Vec<Game>>, (Status, String)> {
    let matchs = match connection
        .run(move |c| {
            games::table
                .filter(games::fk_team1.eq(id).or(games::fk_team2.eq(id)))
                .load::<Game>(c)
        })
        .await
    {
        Err(_) => return Err((Status::NotFound, "Wrong code".to_string())),
        Ok(matchs) => matchs,
    };

    Ok(Json(matchs))
}

// validate the score of a game, lock it and give the nut to the winners / remove it from the losers
#[get("/game/<id>/close")]
pub async fn close_game(
    connection: MysqlConnection,
    id: i32,
) -> Result<Status, (Status, String)> {
    update_game_fn(&connection, Json(PatchGame { has_gained_nut: Some(false), fk_team1: None, fk_team2:None, place: None, is_open: None }), id).await?;
    return calculate_gain(&connection, id).await;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddGame {
    pub fk_team1: i32,
    pub fk_team2: i32,
    pub phase: i32,
    pub place: i32,
}

#[post("/tournoix/qualif", data = "<data>")]
pub async fn create_games(
    connection: MysqlConnection,
    data: Json<Vec<AddGame>>,
) -> Result<Json<Vec<Game>>, (Status, String)> {
    let mut games = Vec::new();

    for game in data.0 {
        let game = NewGame {
            fk_team1: game.fk_team1,
            fk_team2: game.fk_team2,
            score1: 0,
            score2: 0,
            phase: game.phase,
            place: game.place,
        };

        match connection
            .run(move |c| {
                c.transaction(|c| {
                    diesel::insert_into(games::table)
                        .values(game.clone())
                        .execute(c)?;

                    let game = games::table.order(games::id.desc()).first::<Game>(c)?;

                    diesel::result::QueryResult::Ok(game)
                })
            })
            .await
        {
            Ok(game) => {
                games.push(game);
            }

            Err(_e) => {
                return Err((
                    Status::InternalServerError,
                    "Internel Server Error".to_string(),
                ))
            }
        }
    }

    Ok(Json(games))
}

async fn update_game_fn(
    connection: &MysqlConnection,
    data: Json<PatchGame>,
    id: i32,
) -> Result<Json<Game>, (Status, String)> {
    let game = data.0;

    // cannot update the game it the betting is closed
    if game.is_open == Some(false) {
        return Err((Status::BadRequest, "Betting is closed".to_string()));
    }

    match connection
        .run(move |c| {
            c.transaction(|c| {
                diesel::update(games::table.find(id))
                    .set(game.clone())
                    .execute(c)?;

                let game = games::table
                    .order(games::id.desc())
                    .first::<Game>(c)
                    .map(Json)?;

                diesel::result::QueryResult::Ok(game)
            })
        })
        .await
    {
        Ok(game) => {
            return Ok(game);
        }

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string(),
            ))
        }
    }
}

#[patch("/game/<id>", data = "<data>")]
pub async fn update_game(
    connection: MysqlConnection,
    data: Json<PatchGame>,
    id: i32,
) -> Result<Json<Game>, (Status, String)> {
    return update_game_fn(&connection, data, id).await;
}

// block the action of betting on a game
#[patch("/game/<id>/closeBetting")]
pub async fn close_game_betting(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Game>, (Status, String)> {
    let game = match connection.run(
       move |c| c.transaction(|c| {
            diesel::update(games::table.find(id))
                .set(games::is_open.eq(false))
                .execute(c)?;

            let game = games::table.order(games::id.desc()).first::<Game>(c).map(Json)?;

            diesel::result::QueryResult::Ok(game)
        })
    ).await {
        Ok(game) => game,
        Err(_e) => {
            
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string()
            ))
        }
    };

    Ok(game)
}
