use diesel::prelude::*;
use log::warn;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use crate::MysqlConnection;
use crate::models::bet::{Bet, NewBet, PathBet};
use crate::models::game::Game;
use crate::models::nut::Nut;
use crate::models::subscription::Subscription;
use crate::models::tournament::Tournament;
use crate::routes::auth::ApiAuth;
use crate::schema::{bets, games, teams, tournaments, nuts, subscriptions};
use chrono::Local;

// Get all bets of a game
#[get("/game/<id>/bet")]
pub async fn get_game_bet(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<Vec<Bet>>, (Status, String)> {
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
        warn!("{} - User {} tried to access bets of tournament {} - routes/bet/get_game_bet()", Local::now().format("%d/%m/%Y %H:%M"), auth.user.id, id);
        return Err((Status::Forbidden, "Access Forbidden".to_string()));
    }

    
    match connection.run(
        move |c| bets::table.filter(bets::fk_games.eq(id)).get_results::<Bet>(c)
    ).await.map(Json) {
        Ok(bet) => {
           return Ok(bet)
        },
        Err(_e) => {
            return Err((
                Status::NotFound,
                "Bets not found".to_string()
            ))
        }
    }
}

#[get("/user/<id_user>/game/<id>/bet")]
pub async fn get_user_game_bet(
    connection: MysqlConnection,
    id: i32,
    id_user: i32,
    auth: ApiAuth,
) -> Result<Json<Bet>, (Status, String)> {
    
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
            warn!("{} - User {} tried to access bets of user {} for tournament {} - routes/bet/get_user_game_bet()", Local::now().format("%d/%m/%Y %H:%M"), auth.user.id, id_user, id);
            return Err((Status::Forbidden, "Access Forbidden".to_string()));
        }
    };

    
    // get the nut of the player for this game
    let nut_id = match connection.run(
        move |c| games::table
        .inner_join(teams::table.on(teams::id.eq(games::fk_team1)))
        .inner_join(tournaments::table.on(tournaments::id.eq(teams::fk_tournaments)))
        .inner_join(nuts::table.on(nuts::fk_tournaments.eq(tournaments::id).and(nuts::fk_users.eq(id_user))))
        .filter(games::id.eq(id))
        .select(nuts::id)
        .first::<i32>(c)
    ).await {
        Ok(nut_id) => nut_id,
        Err(_e) => {
            return Err((
                Status::NotFound,
                "User tournoix nuts not found".to_string()
            ))
        }
    };

    // get the bet of the user for this game
    match connection.run(
        move |c| bets::table.filter(bets::fk_nuts.eq(nut_id)).filter(bets::fk_games.eq(id)).first::<Bet>(c)
    ).await{
        Ok(bet) => {
            return Ok(Json(bet))
        },
        Err(_e) => {
            return Err((
                Status::NotFound,
                "User bet not found".to_string()
            ))
        }
    };
}

// calculate the gain of all users betting on a game
pub async fn calculate_gain(
    connection: &MysqlConnection,
    game_id: i32,
) -> Result<Status, (Status, String)>{
    // get the game
    let game = match connection.run(
        move |c| games::table.find(game_id).first::<Game>(c)
    ).await{
        Ok(game) => game,
        Err(_e) => {
            return Err((
                Status::NotFound,
                "Game not found".to_string()
            ))
        }
    };

    // get all bets linked to the game
    let bets = match connection.run(
        move |c| bets::table.filter(bets::fk_games.eq(game_id)).get_results::<Bet>(c)
    ).await{
        Ok(bets) => bets,
        Err(_e) => {
            return Err((
                Status::NotFound,
                "Bets not found".to_string()
            ))
        }
    };

    // check if there is a winner
    if game.score1 == game.score2 {
        // if there is no winner, give back the nut to the user
        for bet in bets.clone() {
            let nut = match connection.run(
                move |c| nuts::table.find(bet.fk_nuts).first::<Nut>(c)
            ).await{
                Ok(nut) => nut,
                Err(_e) => {
                    return Err((
                        Status::NotFound,
                        "Nut not found".to_string()
                    ))
                }
            };

            let new_stock = nut.stock + bet.nb_nut;
            set_stock(&connection, nut.id, new_stock).await?;
        }

        return Ok(Status::Ok);
    }	

    // find the winner
    let winner = if game.score1 > game.score2 {game.fk_team1} else {game.fk_team2};

    // if the user bet on the winning team
    let mut winner_total_bet = 0;
    let mut total_bet = 0;

    for bet in bets.clone() {
        if bet.fk_teams == winner {
            winner_total_bet += bet.nb_nut;
        }

        total_bet += bet.nb_nut;
    }
    
    // calculate the gain for the winning team
    for bet in bets {
        if bet.fk_teams == winner {
            let nut = match connection.run(
                move |c| nuts::table.find(bet.fk_nuts).first::<Nut>(c)
            ).await{
                Ok(nut) => nut,
                Err(_e) => {
                    return Err((
                        Status::NotFound,
                        "Nut not found".to_string()
                    ))
                }
            };

            let new_stock = nut.stock + bet.nb_nut + (bet.nb_nut as f64 * total_bet as f64 / winner_total_bet as f64).round() as i32;
            set_stock(&connection, nut.id, new_stock).await?;
        }
    }

    return Ok(Status::Ok);
}

// get the gain of the user for the game
#[get("/game/<id_game>/bet/result")]
pub async fn get_user_game_bet_result(
    connection: MysqlConnection,
    auth: ApiAuth,
    id_game: i32,
) -> Result<Json<i32>, (Status, String)> {
    // get the game info
    let game = match connection.run(
        move |c| games::table.find(id_game).first::<Game>(c)
    ).await{
        Ok(game) => game,
        Err(_e) => {
            return Err((
                Status::NotFound,
                "Game not found".to_string()
            ))
        }
    };

    // if the game is not finished
    if game.has_gained_nut == false {
        return Err((
            Status::NotFound,
            "Game nuts not given".to_string()
        ))
    }

    // get the user nut
    let nut = get_user_nut(&connection, auth.user.id, id_game).await?;

    // get the user bet
    let bet = match connection.run(
        move |c| bets::table.filter(bets::fk_nuts.eq(nut.id)).filter(bets::fk_games.eq(id_game)).first::<Bet>(c)
    ).await{
        Ok(bet) => bet,
        Err(_e) => {
            return Err((
                Status::NotFound,
                "User bet not found".to_string()
            ))
        }
    };

    // if egality
    if game.score1 == game.score2 {
        return Ok(Json(0));
    }

    // calculate the winning team
    let winner = if game.score1 > game.score2 {game.fk_team1} else {game.fk_team2};

    // if the user bet on the loosing team
    if bet.fk_teams != winner {
        return Ok(Json(-bet.nb_nut));
    }

    // get all bets linked to the game
    let bets = match connection.run(
        move |c| bets::table.filter(bets::fk_games.eq(id_game)).get_results::<Bet>(c)
    ).await{
        Ok(bets) => bets,
        Err(_e) => {
            return Err((
                Status::NotFound,
                "Bets not found".to_string()
            ))
        }
    };

    // if the user bet on the winning team
    let mut winner_total_bet = 0;
    let mut total_bet = 0;

    for bet in bets {
        if bet.fk_teams == winner {
            winner_total_bet += bet.nb_nut;
        }

        total_bet += bet.nb_nut;
    }

    return Ok(Json((bet.nb_nut as f64 * total_bet as f64 / winner_total_bet as f64).round() as i32 + bet.nb_nut));
}


// change the stock of a player
async fn set_stock(connection: &MysqlConnection, id: i32, stock: i32) -> Result<Nut, (Status, String)> {
    match connection
    .run(move |c| {
        c.transaction(|c| {
            diesel::update(nuts::table.find(id))
                .set(nuts::stock.eq(stock))
                .execute(c)?;

            let nut = nuts::table
                .find(id)
                .first::<Nut>(c)?;

            diesel::result::QueryResult::Ok(nut)
        })
    }).await{
        Ok(nut) => {
            return Ok(nut);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "No nuts found".to_string()
            ))
        }
    }
}

// get the nut of a user for a game
async fn get_user_nut(connection: &MysqlConnection, id_user: i32, id_game: i32) -> Result<Nut, (Status, String)> {
    match connection.run(
        move |c| games::table
            .inner_join(teams::table.on(teams::id.eq(games::fk_team1)))
            .inner_join(tournaments::table.on(tournaments::id.eq(teams::fk_tournaments)))
            .inner_join(nuts::table.on(nuts::fk_tournaments.eq(tournaments::id)))
            .filter(nuts::fk_users.eq(id_user))
            .filter(games::id.eq(id_game))
            .select((nuts::id, nuts::fk_users, nuts::fk_tournaments, nuts::stock))
            .first::<(i32, i32, i32, i32)>(c)
    ).await{
        Ok(nut_data) => {
            let nut = Nut {
                id: nut_data.0,
                fk_users: nut_data.1,
                fk_tournaments: nut_data.2,
                stock: nut_data.3,
            };

            return Ok(nut);
        },
        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "No nuts found".to_string()
            ))
        }
    }
}

// check if the game is open
async fn is_game_open(connection: &MysqlConnection, id: i32) -> Result<bool, (Status, String)> {
    match connection.run(
        move |c| games::table.find(id).select(games::status).first::<i32>(c)
    ).await{
        Ok(status) => {
            return Ok(status == 0);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "No nuts found".to_string()
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BetData {
    pub team_id: usize,
    pub nut: u32
}

// create a bet
#[post("/game/<id>/bet", data = "<data>")]
pub async fn create_bet(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
    data: Json<BetData>,
) -> Result<Json<Bet>, (Status, String)> {
    // get the tournoix id
    let nut = get_user_nut(&connection, auth.user.id, id).await?;

    // if the game is not open
    if !is_game_open(&connection, id).await? {
        return Err((
            Status::NotFound,
            "Game is not open".to_string()
        ))
    }

    // if the user don't have enough nut
    if data.nut > nut.stock as u32 {
        return Err((
            Status::NotFound,
            "Not enough nut".to_string()
        ))
    }

    // remove the nut from the user
    let new_stock = nut.stock - data.nut as i32;
    let nut = set_stock(&connection, nut.id, new_stock).await?;

    // create the bet
    let new_bet = NewBet {
        fk_nuts: nut.id,
        fk_games: id,
        nb_nut: data.nut as i32,
        fk_teams: data.team_id as i32
    };

    // add the bet to the database
    match connection.run(
        move |c| {
            diesel::insert_into(bets::table)
                .values(new_bet)
                .execute(c)
        }
    ).await{
        Ok(_) => {
            // get the bet
            let bet = match connection.run(
                move |c| bets::table.order(bets::id.desc()).first::<Bet>(c)
            ).await{
                Ok(bet) => bet,
                Err(_e) => {
                    return Err((
                        Status::InternalServerError,
                        "No bet found".to_string()
                    ))
                }
            };

            return Ok(Json(bet));
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "No bet found".to_string()
            ))
        }
    };
}

// update a bet
#[patch("/game/<id>/bet", data = "<data>")]
pub async fn update_bet(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
    data: Json<BetData>,
) -> Result<Json<Bet>, (Status, String)> {
    // get the tournoix id
    let nut = get_user_nut(&connection, auth.user.id, id).await?;

    // get the betted nut
    let bet = match connection.run(
        move |c| bets::table.filter(bets::fk_games.eq(id)).filter(bets::fk_nuts.eq(nut.id)).first::<Bet>(c)
    ).await{
        Ok(bet) => bet,
        Err(_e) => {
            return Err((
                Status::NotFound,
                "User bet not found".to_string()
            ))
        }
    };

    // if the game is not open
    if !is_game_open(&connection, id).await? {
        return Err((
            Status::NotFound,
            "Game is not open".to_string()
        ))
    }

    // calculate the difference between the old and the new bet
    let diff = data.nut - bet.nb_nut as u32;

    // if the user don't have enough nut
    if diff > nut.stock as u32 {
        return Err((
            Status::NotFound,
            "Not enough nut".to_string()
        ))
    }

    // remove or give back the nut from the user
    let new_stock = nut.stock - diff as i32;
    let _ = set_stock(&connection, nut.id, new_stock).await?;

    // if the new bet nut number is 0, remove the bet
    if data.nut == 0 {
        match connection.run(
            move |c| {
                diesel::delete(bets::table.find(bet.id))
                    .execute(c)
            }
        ).await{
            Ok(_) => {
                return Ok(Json(bet));
            },

            Err(_e) => {
                return Err((
                    Status::InternalServerError,
                    "No bet found".to_string()
                ))
            }
        };
    }

    // create the bet
    let updated_bet = PathBet {
        nb_nut: Some(data.nut as i32),
        fk_teams: Some(data.team_id as i32)
    };

    // add the bet to the database
    match connection.run(
        move |c| {
            diesel::update(bets::table.find(bet.id))
                .set(updated_bet)
                .execute(c)
        }
    ).await{
        Ok(_) => {
            // get the bet
            let bet = match connection.run(
                move |c| bets::table.find(bet.id).first::<Bet>(c)
            ).await{
                Ok(bet) => bet,
                Err(_e) => {
                    return Err((
                        Status::InternalServerError,
                        "No bet found".to_string()
                    ))
                }
            };

            return Ok(Json(bet));
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "No bet found".to_string()
            ))
        }
    };
}

// delete a bet
#[delete("/game/<id>/bet")]
pub async fn delete_bet(
    connection: MysqlConnection,
    auth: ApiAuth,
    id: i32,
) -> Result<Json<Bet>, (Status, String)> {
    //get the nut of the player
    let nut = get_user_nut(&connection, auth.user.id, id).await?;

    match connection
        .run(move |c| {
            c.transaction(|c| {
                // get the bet
                let bet = bets::table
                    .filter(bets::fk_nuts.eq(nut.id))
                    .filter(bets::fk_games.eq(id))
                    .first::<Bet>(c)?;

                // remove the bet
                diesel::delete(bets::table
                    .find(bet.id))
                    .execute(c)?;

                diesel::result::QueryResult::Ok(bet)
            })
        })
        .await
    {
        Ok(bet) => {
            // place the nut back in the stock
            let new_stock = nut.stock + bet.nb_nut;
            set_stock(&connection, nut.id, new_stock).await?;

            return Ok(Json(bet));
        },
        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string(),
            ))
        }
    }
}