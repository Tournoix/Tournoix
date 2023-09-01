use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use crate::MysqlConnection;
use crate::models::bet::{Bet, NewBet};
use crate::models::nut::Nut;
use crate::routes::auth::ApiAuth;
use crate::schema::{bets, games, teams, tournaments, nuts};

use super::auth;

//avoir les bet d un match
//avoir les bet d une team

#[get("/game/<id>/bet")]
pub async fn get_game_bet(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Vec<Bet>>, (Status, String)> {
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
) -> Result<Json<Bet>, (Status, String)> {
    // get the tournoix id
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

async fn  get_stock(connection: &MysqlConnection, id: i32) -> Result<i32, (Status, String)> {
    match connection.run(
        move |c| nuts::table.find(id).select(nuts::stock).first::<i32>(c)
    ).await{
        Ok(stock) => {
            return Ok(stock);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "No nuts found".to_string()
            ))
        }
    }
}

async fn get_user_nut(connection: &MysqlConnection, id_user: i32, id_tournament: i32) -> Result<Nut, (Status, String)> {
    match connection.run(
        move |c| nuts::table
        .filter(nuts::fk_users.eq(id_user))
        .filter(nuts::fk_tournaments.eq(id_tournament))
        .first::<Nut>(c)
    ).await{
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

async fn is_game_open(connection: &MysqlConnection, id: i32) -> Result<bool, (Status, String)> {
    match connection.run(
        move |c| games::table.find(id).select(games::is_open).first::<bool>(c)
    ).await{
        Ok(is_open) => {
            return Ok(is_open);
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
pub struct AddBetData {
    pub team_id: usize,
    pub nut: u32
}

// create a bet
#[post("/game/<id>/bet", data = "<data>")]
pub async fn create_bet(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
    data: Json<AddBetData>,
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


// il faut que lorsque l on fait un bet le stock se vide
// quand la bet est modifie le stock est mis à jour selon de difference
// quand le bet est supprimé les noix sont rendues
// si la game est terminee, il y a un score
// calculer les gain et perte de noix des parieurs

/*
#[patch("/nut/<id>", data = "<data>")]
pub async fn create_bet(
    connection: MysqlConnection,
    id: i32,
    data: Json<PatchNut>,
) -> Result<Json<Nut>, (Status, String)> {


    match connection.run(
       move |c| c.transaction(|c| {
            diesel::update(nuts::table.find(id))
                .set(data.0)
                .execute(c)?;

            let nut = nuts::table.order(nuts::id.desc()).first::<Nut>(c).map(Json)?;

            diesel::result::QueryResult::Ok(nut)
        })
    ).await {
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
}*/