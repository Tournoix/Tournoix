use std::collections::BTreeMap;

use crate::models::game::Game;
use crate::models::game::*;
use crate::models::subscription::Subscription;
use crate::models::team::Team;
use crate::models::tournament::Tournament;
use crate::routes::auth::ApiAuth;
use crate::schema::{games, subscriptions, teams, tournaments};
use crate::{EmptyResponse, ErrorBody, ErrorResponse, MysqlConnection};
use chrono::Local;
use diesel::prelude::*;
use log::warn;
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
) -> Result<Json<Vec<GameWithTeams>>, (Status, Json<ErrorResponse>)> {
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
        warn!(
            "{} - User {} tried to access tournament {} - routes/tournoix/get_tournoix_game()",
            Local::now().format("%d/%m/%Y %H:%M"),
            auth.user.id,
            id
        );
        return Err((
            Status::Forbidden,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: 403,
                    reason: "Forbiden".into(),
                    description: "Access Forbidden".into(),
                },
            }),
        ));
    }

    // get all team from a tournament
    let teams: Vec<Team> = match connection
        .run(move |c| {
            teams::table
                .filter(teams::fk_tournaments.eq(id))
                .load::<Team>(c)
        })
        .await
    {
        Ok(teams) => teams,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 500,
                        reason: "Internal Server Error".into(),
                        description: "An error has occured".into(),
                    },
                }),
            ))
        }
    };

    // get all match from a team
    match connection
        .run(move |c| {
            let (teams1, teams2) = alias!(teams as team1, teams as team2);
            games::table
                .inner_join(teams1.on(games::fk_team1.eq(teams1.field(teams::id))))
                .inner_join(teams2.on(games::fk_team2.eq(teams2.field(teams::id))))
                .select((
                    games::id,
                    teams1.fields(teams::all_columns),
                    teams2.fields(teams::all_columns),
                    games::score1,
                    games::score2,
                    games::phase,
                    games::place,
                    games::status,
                    games::has_gained_nut,
                    teams1.field(teams::group),
                ))
                .filter(games::fk_team1.eq_any(teams.iter().map(|t| t.id)))
                .or_filter(games::fk_team1.eq_any(teams.iter().map(|t| t.id)))
                .load::<GameWithTeams>(c)
        })
        .await
    {
        Ok(matchs) => Ok(Json(matchs)),
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 500,
                        reason: "Internal Server Error".into(),
                        description: "An error has occured".into(),
                    },
                }),
            ))
        }
    }
}

// get all match from a team
#[get("/team/<id>/games")]
pub async fn get_team_game(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Vec<GameWithGroup>>, (Status, String)> {
    let matchs = match connection
        .run(move |c| {
            games::table
                .inner_join(teams::table.on(games::fk_team1.eq(teams::id)))
                .select((
                    games::id,
                    games::fk_team1,
                    games::fk_team2,
                    games::score1,
                    games::score2,
                    games::phase,
                    games::place,
                    games::status,
                    games::has_gained_nut,
                    teams::group,
                ))
                .filter(games::fk_team1.eq(id).or(games::fk_team2.eq(id)))
                .load::<GameWithGroup>(c)
        })
        .await
    {
        Err(_) => return Err((Status::NotFound, "Wrong code".to_string())),
        Ok(matchs) => matchs,
    };

    Ok(Json(matchs))
}

// get a match from an id
#[get("/game/<id>")]
pub async fn get_game(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<GameWithTeams>, (Status, String)> {
    let matchs = match connection
        .run(move |c| {
            let (teams1, teams2) = alias!(teams as team1, teams as team2);
            games::table
                .inner_join(teams1.on(games::fk_team1.eq(teams1.field(teams::id))))
                .inner_join(teams2.on(games::fk_team2.eq(teams2.field(teams::id))))
                .select((
                    games::id,
                    teams1.fields(teams::all_columns),
                    teams2.fields(teams::all_columns),
                    games::score1,
                    games::score2,
                    games::phase,
                    games::place,
                    games::status,
                    games::has_gained_nut,
                    teams1.field(teams::group),
                ))
                .filter(games::id.eq(id))
                .first::<GameWithTeams>(c)
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
        warn!(
            "{} - User {} tried to access game {} - routes/game/close_game()",
            Local::now().format("%d/%m/%Y %H:%M"),
            auth.user.id,
            id
        );
        return Err((Status::Unauthorized, "Unauthorized".to_string()));
    }
    update_game_fn(
        &connection,
        Json(PatchGame {
            has_gained_nut: Some(false),
            fk_team1: None,
            fk_team2: None,
            place: None,
            status: None,
        }),
        id,
    )
    .await?;
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
) -> Result<Json<Vec<Game>>, (Status, Json<ErrorResponse>)> {
    // verify if the user is the owner of the tournament
    if !is_owner(&connection, id, &auth).await {
        warn!("{} - User {} tried to create a game for tournament {} - routes/tournoix/create_games()", Local::now().format("%d/%m/%Y %H:%M"), auth.user.id, id);
        return Err((
            Status::Forbidden,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: 403,
                    reason: "Forbiden".into(),
                    description: "Access Forbidden".into(),
                },
            }),
        ));
    }

    delete_tournament_games(&connection, id).await;

    // get all team from a tournament
    let teams = match connection
        .run(move |c| {
            teams::table
                .filter(teams::fk_tournaments.eq(id))
                .load::<Team>(c)
        })
        .await
    {
        Ok(teams) => teams,
        Err(_) => {
            return Err((
                Status::BadRequest,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 400,
                        reason: "no_team".into(),
                        description: "No team found for the tournament".into(),
                    },
                }),
            ))
        }
    };

    // group team by group
    let mut groups: BTreeMap<i32, Vec<Team>> = BTreeMap::new();
    for team in teams {
        if groups.contains_key(&team.group) {
            groups.get_mut(&team.group).unwrap().push(team);
        } else {
            groups.insert(team.group, vec![team]);
        }
    }

    // generate the games for each group
    let mut games: Vec<NewGame> = Vec::new();
    for (id, group) in groups {
        // not enough team in the group to play
        if group.len() < 2 {
            return Err((
                Status::BadRequest,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 400,
                        reason: "not_enough_team".into(),
                        description: "Not enought team in a group to generate games".into(),
                    },
                }),
            ));
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
                    status: 0,
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
                    Json(ErrorResponse {
                        error: ErrorBody {
                            code: 500,
                            reason: "Internel Server Error".into(),
                            description: "An error occured".into(),
                        },
                    }),
                ))
            }
        }
    }

    Ok(Json(games_added))
}

#[delete("/tournoix/<id>/qualif")]
pub async fn remove_all_games(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<EmptyResponse>, (Status, Json<ErrorResponse>)> {
    if !is_owner(&connection, id, &auth).await {
        warn!("{} - User {} tried to delete a game for tournament {} - routes/game/remove_all_games()", Local::now().format("%d/%m/%Y %H:%M"), auth.user.id, id);
        return Err((
            Status::Forbidden,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: 403,
                    reason: "Forbiden".into(),
                    description: "Access Forbidden".into(),
                },
            }),
        ));
    }

    delete_tournament_games(&connection, id).await;

    Ok(Json(EmptyResponse()))
}

async fn update_game_fn(
    connection: &MysqlConnection,
    data: Json<PatchGame>,
    id: i32,
) -> Result<Json<Game>, (Status, String)> {
    let game = data.0;

    // cannot update the game it the betting is closed
    /*
    if game.status == Some(false) {
        return Err((Status::BadRequest, "Betting is closed".to_string()));
    }
    */

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
    auth: ApiAuth,
) -> Result<Json<Game>, (Status, String)> {
    // the user is not the owner of the game
    if !is_owner_game(&connection, id, &auth).await {
        warn!(
            "{} - User {} tried to access game {} - routes/game/close_game_betting()",
            Local::now().format("%d/%m/%Y %H:%M"),
            auth.user.id,
            id
        );
        return Err((Status::Unauthorized, "Unauthorized".to_string()));
    }

    let game = match connection
        .run(move |c| {
            c.transaction(|c| {
                diesel::update(games::table.find(id))
                    .set(games::status.eq(1))
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
        Ok(game) => game,
        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string(),
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

pub async fn delete_tournament_games(connection: &MysqlConnection, tournament_id: i32) {
    let games = connection
        .run(move |c| {
            games::table
                .inner_join(teams::table.on(games::fk_team1.eq(teams::id)))
                .filter(teams::fk_tournaments.eq(tournament_id))
                .select(games::all_columns)
                .load::<Game>(c)
        })
        .await
        .ok();

    if let Some(games) = games {
        let _ = connection
            .run(move |c| {
                diesel::delete(games::table.filter(games::id.eq_any(games.iter().map(|g| g.id))))
                    .execute(c)
            })
            .await;
    }
}
