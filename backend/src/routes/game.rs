use crate::models::game::Game;
use crate::models::game::*;
use crate::models::subscription::Subscription;
use crate::models::team::Team;
use crate::models::tournament::Tournament;
use crate::routes::auth::ApiAuth;
use crate::schema::{games, tournaments, subscriptions, teams};
use crate::MysqlConnection;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use super::bet::calculate_gain;
use super::tournoix::is_owner;

// get all match from a tournament
#[get("/tournoix/<id>/games")]
pub async fn get_tournoix_game(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<Vec<Game>>, (Status, String)> {
    // Check if the user is a subscriber/owner of the tournament
    let is_owner = match connection
        .run(move |c| {
            tournaments::table
                .filter(tournaments::id.eq(id))
                .filter(tournaments::fk_users.eq(auth.user.id))
                .first::<Tournament>(c)
        })
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    };

    let is_subscriber = match connection
        .run(move |c| {
            subscriptions::table
                .filter(subscriptions::fk_tournaments.eq(id))
                .filter(subscriptions::fk_users.eq(auth.user.id))
                .first::<Subscription>(c)
        })
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    };

    if !is_owner && !is_subscriber {
        return Err((Status::Forbidden, "Access Forbidden".to_string()));
    }

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
    auth: ApiAuth,
) -> Result<Status, (Status, String)> {
    // the user is not the owner of the game
    if !is_owner_game(&connection, id, &auth).await {
        return Err((Status::Unauthorized, "Unauthorized".to_string()));
    }
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

// enum for the phase of the tournament
#[derive(Serialize, Deserialize, Clone)]
pub enum Phase {
    Qualification = 0,
    Elimination,
}

#[post("/tournoix/<id>/qualif")]
pub async fn create_games(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<Vec<Game>>, (Status, String)> {
    // verify if the user is the owner of the tournament
    if is_owner(&connection, id, &auth).await {
        return Err((Status::Unauthorized, "Unauthorized".to_string()));
    }

    // get all team from a tournament
    let teams = match connection
        .run(move |c| teams::table.filter(teams::fk_tournaments.eq(id)).load::<Team>(c))
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

    // group team by group
    let mut groups: Vec<Vec<Team>> = Vec::new();
    for team in teams {
        groups[team.group as usize].push(team);
    }

    // generate the games for each group
    let mut games: Vec<NewGame> = Vec::new();
    for group in groups {
        // not enough team in the group to play
        if group.len() < 2 {
            return Err((
                Status::NotFound,
                "Not enought team in a group to generate games".to_string(),
            ))
        }
    
        let mut nb_game_added = 0;

        for i in 0..group.len() {
            for j in i + 1..group.len() {
                let game = NewGame {
                    fk_team1: group[i].id,
                    fk_team2: group[j].id,
                    score1: 0,
                    score2: 0,
                    place: nb_game_added,
                    phase: Phase::Qualification as i32,
                };

                // position of the game in the group
                nb_game_added += 1;

                games.push(game);
            }
        }
    }

    // all added games
    let mut games_added: Vec<Game> = Vec::new();

    // add all games to the database
    for game in games {
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
                games_added.push(game);
            }

            Err(_e) => {
                return Err((
                    Status::InternalServerError,
                    "Internel Server Error".to_string(),
                ))
            }
        }
    }

    Ok(Json(games_added))
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
    auth: ApiAuth
) -> Result<Json<Game>, (Status, String)> {
    // the user is not the owner of the game
    if !is_owner_game(&connection, id, &auth).await {
        return Err((Status::Unauthorized, "Unauthorized".to_string()));
    }

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

// verify if the user can edit a game
async fn is_owner_game(connection: &MysqlConnection, id: i32, auth: &ApiAuth) -> bool {
    let user_id = auth.user.id;
    let game = match connection
        .run(move |c| {
            games::table
                .filter(games::id.eq(id))
                .inner_join(teams::table.on(games::fk_team1.eq(teams::id)))
                .inner_join(tournaments::table.on(teams::fk_tournaments.eq(tournaments::id)))
                .filter(tournaments::fk_users.eq(user_id))
                .first::<(Game, Team, Tournament)>(c)
        })
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    };

    return game;
}