use std::collections::{BTreeMap, HashMap};

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
use rand::seq::SliceRandom;
use rand::thread_rng;
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

    // get all match from a team
    match connection
        .run(move |c| {
            let (teams1, teams2) = alias!(teams as team1, teams as team2);
            games::table
                .inner_join(teams1.on(games::fk_team1.eq(teams1.field(teams::id))))
                .inner_join(teams2.on(games::fk_team2.eq(teams2.field(teams::id))))
                .select((
                    games::id,
                    games::fk_tournaments,
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
                .filter(games::fk_tournaments.eq(id))
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
#[get("/teams/<id>/games")]
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
                    games::fk_tournaments,
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
                    games::fk_tournaments,
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
#[post("/games/<id>/close")]
pub async fn close_game(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<EmptyResponse>, (Status, Json<ErrorResponse>)> {
    // the user is not the owner of the game
    if !is_owner_game(&connection, id, &auth).await {
        warn!(
            "{} - User {} tried to access game {} - routes/game/close_game()",
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

    let game = connection
        .run(move |c| games::table.find(id).first::<Game>(c))
        .await
        .ok();

    update_game_fn(
        &connection,
        Json(PatchGame {
            has_gained_nut: Some(false),
            fk_team1: None,
            fk_team2: None,
            score1: None,
            score2: None,
            place: None,
            status: Some(2),
        }),
        id,
    )
    .await?;

    if let Some(game) = game {
        if game.phase >= 1 {
            // Elim game, need to move team forward

            let other_game_place = game.place + {
                if game.place % 2 == 0 {
                    1
                } else {
                    -1
                }
            };
            let other_game = connection
                .run(move |c| {
                    games::table
                        .filter(games::phase.eq(game.phase))
                        .filter(games::place.eq(other_game_place))
                        .first::<Game>(c)
                })
                .await
                .ok();

            if let Some(other_game) = other_game {
                if other_game.status == 2 {
                    let team1 = if game.place % 2 == 0 {
                        game.winner()
                    } else {
                        other_game.winner()
                    };
                    let team2 = if game.place % 2 == 0 {
                        other_game.winner()
                    } else {
                        game.winner()
                    };

                    let new_game = NewGame {
                        fk_tournaments: game.fk_tournaments,
                        fk_team1: team1,
                        fk_team2: team2,
                        score1: 0,
                        score2: 0,
                        place: (game.place as f32 / 2.0).floor() as i32,
                        phase: game.phase + 1,
                        status: 0,
                    };

                    match connection
                        .run(move |c| {
                            diesel::insert_into(games::table)
                                .values(new_game)
                                .execute(c)
                        })
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{}", e);
                            return Err((
                                Status::InternalServerError,
                                Json(ErrorResponse {
                                    error: ErrorBody {
                                        code: 500,
                                        reason: "Internel Server Error".into(),
                                        description: "An error occured".into(),
                                    },
                                }),
                            ));
                        }
                    }
                }
            }
        }
    }

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

    if !delete_tournament_games_qualif(&connection, id).await {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: 400,
                    reason: "Bad Request".into(),
                    description: "Can't reset games if a game has started".into(),
                },
            }),
        ));
    }

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
    for (_group_id, group) in groups {
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
                    fk_tournaments: id,
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
                println!("{}", _e);
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

#[post("/tournoix/<id>/elim")]
pub async fn create_games_elim(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<EmptyResponse>, (Status, Json<ErrorResponse>)> {
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

    if !delete_tournament_games_elim(&connection, id).await {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: 400,
                    reason: "Bad Request".into(),
                    description: "Can't reset games if a game has started".into(),
                },
            }),
        ));
    }

    let tournament: Tournament = match connection
        .run(move |c| tournaments::table.find(id).first::<Tournament>(c))
        .await
    {
        Ok(t) => t,
        Err(_e) => {
            return Err((
                Status::NotFound,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 404,
                        reason: "Not Found".into(),
                        description: "Tournament not found".into(),
                    },
                }),
            ))
        }
    };

    if tournament.is_qualif {
        // Tournament has qualification phase
        // Need to check if all games in qualif are over
        // And then get all the winners

        let games: Option<Vec<GameWithGroup>> = connection
            .run(move |c| {
                games::table
                    .inner_join(teams::table.on(games::fk_team1.eq(teams::id)))
                    .filter(games::fk_tournaments.eq(id))
                    .select((
                        games::id,
                        games::fk_tournaments,
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
                    .load::<GameWithGroup>(c)
            })
            .await
            .ok();

        if let Some(games) = games {
            // Check if all games are over
            if games.len() == 0 || games.iter().any(|g| g.status != 2) {
                return Err((
                    Status::BadRequest,
                    Json(ErrorResponse {
                        error: ErrorBody {
                            code: 400,
                            reason: "Bad Request".into(),
                            description: "All games in qualification need to be finisged".into(),
                        },
                    }),
                ));
            }

            // group team by group
            let mut groups: BTreeMap<i32, Vec<GameWithGroup>> = BTreeMap::new();
            for game in games {
                if groups.contains_key(&game.group) {
                    groups.get_mut(&game.group).unwrap().push(game);
                } else {
                    groups.insert(game.group, vec![game]);
                }
            }

            let mut teams_score: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
            for (id_group, games) in groups {
                for game in games {
                    let winner = game.winner();
                    *teams_score
                        .entry(id_group)
                        .or_insert(HashMap::new())
                        .entry(winner)
                        .or_insert(0) += 1;
                }
            }

            let mut winners = teams_score
                .iter()
                .map(|(_, teams)| teams.iter().max().unwrap().0)
                .collect::<Vec<&i32>>();

            // Shuffle teams
            winners.shuffle(&mut thread_rng());
            let mut games: Vec<NewGame> = vec![];
            let mut nb_game_added = 0;

            for i in (0..winners.len()).step_by(2) {
                games.push(NewGame {
                    fk_tournaments: id,
                    fk_team1: *winners[i],
                    fk_team2: *winners[i + 1],
                    score1: 0,
                    score2: 0,
                    place: nb_game_added,
                    phase: 1,
                    status: 0,
                });

                nb_game_added += 1;
            }

            match connection
                .run(move |c| {
                    diesel::insert_into(games::table)
                        .values(games.clone())
                        .execute(c)
                })
                .await
            {
                Ok(_) => return Ok(Json(EmptyResponse())),

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
            };
        } else {
            return Err((
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 500,
                        reason: "Internel Server Error".into(),
                        description: "An error occured".into(),
                    },
                }),
            ));
        }
    } else {
        // No qualif, just get all teams
        let mut teams: Vec<Team> = match connection
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

        // Check if teams are a power of 2
        // We can't generate elimination phase wihtout a power of 2
        if teams.len() & (teams.len() - 1) != 0 {
            return Err((
                Status::BadRequest,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 400,
                        reason: "Bad Request".into(),
                        description: "The number of teams must be a power of 2".into(),
                    },
                }),
            ));
        }

        // Shuffle teams
        teams.shuffle(&mut thread_rng());
        let mut games: Vec<NewGame> = vec![];
        let mut nb_game_added = 0;

        for i in (0..teams.len()).step_by(2) {
            games.push(NewGame {
                fk_tournaments: id,
                fk_team1: teams[i].id,
                fk_team2: teams[i + 1].id,
                score1: 0,
                score2: 0,
                place: nb_game_added,
                phase: 1,
                status: 0,
            });

            nb_game_added += 1;
        }

        match connection
            .run(move |c| {
                diesel::insert_into(games::table)
                    .values(games.clone())
                    .execute(c)
            })
            .await
        {
            Ok(_) => return Ok(Json(EmptyResponse())),

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
        };
    }
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

    delete_tournament_games_qualif(&connection, id).await;

    Ok(Json(EmptyResponse()))
}

#[delete("/tournoix/<id>/elim")]
pub async fn remove_all_games_elim(
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

    delete_tournament_games_elim(&connection, id).await;

    Ok(Json(EmptyResponse()))
}

async fn update_game_fn(
    connection: &MysqlConnection,
    data: Json<PatchGame>,
    id: i32,
) -> Result<Json<Game>, (Status, Json<ErrorResponse>)> {
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

        Err(e) => {
            warn!("{}", e);

            return Err((
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 500,
                        reason: "Internel Server Error".into(),
                        description: "An error occured".into(),
                    },
                }),
            ));
        }
    }
}

#[patch("/games/<id>", data = "<data>")]
pub async fn update_game(
    connection: MysqlConnection,
    data: Json<PatchGame>,
    id: i32,
) -> Result<Json<Game>, (Status, Json<ErrorResponse>)> {
    return update_game_fn(&connection, data, id).await;
}

// block the action of betting on a game
#[patch("/games/<id>/closeBetting")]
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

pub async fn delete_tournament_games_qualif(
    connection: &MysqlConnection,
    tournament_id: i32,
) -> bool {
    let games = connection
        .run(move |c| {
            games::table
                .filter(games::fk_tournaments.eq(tournament_id))
                .filter(games::phase.eq(0))
                .select(games::all_columns)
                .load::<Game>(c)
        })
        .await
        .ok();

    if let Some(games) = games {
        if games.iter().any(|g| g.status != 0) {
            return false;
        } else {
            let _ = connection
                .run(move |c| {
                    diesel::delete(
                        games::table.filter(games::id.eq_any(games.iter().map(|g| g.id))),
                    )
                    .execute(c)
                })
                .await;
        }
    }

    return true;
}

pub async fn delete_tournament_games_elim(
    connection: &MysqlConnection,
    tournament_id: i32,
) -> bool {
    let games = connection
        .run(move |c| {
            games::table
                .filter(games::fk_tournaments.eq(tournament_id))
                .filter(games::phase.gt(0))
                .select(games::all_columns)
                .load::<Game>(c)
        })
        .await
        .ok();

    if let Some(games) = games {
        if games.iter().any(|g| g.status != 0) {
            return false;
        } else {
            let _ = connection
                .run(move |c| {
                    diesel::delete(
                        games::table.filter(games::id.eq_any(games.iter().map(|g| g.id))),
                    )
                    .execute(c)
                })
                .await;
        }
    }

    return true;
}
