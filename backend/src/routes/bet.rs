use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::MysqlConnection;
use crate::models::bet::Bet;
use crate::models::nut::Nut;
use crate::schema::{bets, games, teams, tournaments, nuts};

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

async fn set_stock(connection: MysqlConnection, id: i32, stock: i32) -> Result<(), (Status, String)> {
    match connection.run(
        move |c| c.transaction(|c| {
            diesel::update(nuts::table.find(id))
                .set(nuts::stock.eq(stock))
                .execute(c)?;

            diesel::result::QueryResult::Ok(())
        })
    ).await{
        Ok(_) => {
            return Ok(());
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "No nuts found".to_string()
            ))
        }
    }
}

async fn  get_stock(connection: MysqlConnection, id: i32) -> Result<i32, (Status, String)> {
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

// create a bet
#[post("/game/<id>/bet", data = "<data>")]


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