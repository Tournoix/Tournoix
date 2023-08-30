use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::MysqlConnection;
use crate::models::subscription::{Subscription, NewSubscription};
use crate::models::tournament::Tournament;
use crate::schema::subscriptions::fk_tournaments;
use crate::schema::{tournaments, subscriptions};


#[get("/users/<id>/tournoix")]
pub async fn get_user_tournoix(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Vec<Tournament>>, (Status, String)> {
    match connection.run(
        move |c| tournaments::table.filter(tournaments::fk_users.eq(id)).load::<Tournament>(c)
    ).await.map(Json) {
        Ok(tournoi) => {
           return Ok(tournoi)
        },

        Err(_e) => {
            return Err((
                Status::NotFound,
                "No created tournament found".to_string()
            ))
        }
    }
}

#[get("/users/<id>/subscriptions")]
pub async fn get_user_subscription(
    connection: MysqlConnection,
    id: i32,
) -> Result<Json<Vec<Tournament>>, (Status, String)> {
    // get all subsciptions for the user
    let tab_subscriber = match connection.run(
        move |c| subscriptions::table.filter(subscriptions::fk_users.eq(id)).load::<Subscription>(c)
    ).await {
        Ok(subscribers) => subscribers,
        Err(_) => return Err((Status::NotFound, "Subscription to tournament not found".to_string()))
    };

    // vector to store all tournaments linked to the user
    let mut tournoix_vec = Vec::new();

    // for each subscription, get the tournament
    for subscriber in tab_subscriber {
        let tournaments = match connection.run(
            move |c| tournaments::table.find(subscriber.id).load::<Tournament>(c)
        ).await {
            Ok(tournaments) => tournaments,
            Err(_) => return Err((Status::NotFound, "Wrong tournament id".to_string()))
        };

        tournoix_vec.extend(tournaments);
    }

    Ok(Json(tournoix_vec))
}

#[post("/users/<id>/subscription", data = "<code>")]
pub async fn create_subsciption(
    connection: MysqlConnection,
    code: String,
    id: i32,
) -> Result<Json<Subscription>, (Status, String)> {
    // verify the existance of the code in the database
    let tournament = match connection.run(
        move |c| tournaments::table.filter(tournaments::code.eq(code)).first::<Tournament>(c)
    ).await {
        Ok(tournament) => tournament,
        Err(_) => return Err((Status::NotFound, "Wrong code".to_string()))
    };

    // create the subscription
    let subscription = NewSubscription {
        fk_users: id,
        fk_tournaments: tournament.id,
    };

    // insert the subscription in the database
    match connection.run(
        move |c| c.transaction(|c| {
            diesel::insert_into(subscriptions::table)
                .values(subscription)
                .execute(c)?;

            let subscription = subscriptions::table.filter(subscriptions::fk_tournaments.eq(tournament.id)).filter(subscriptions::fk_users.eq(id)).first::<Subscription>(c).map(Json)?;

            diesel::result::QueryResult::Ok(subscription)
        })
    ).await {
        Ok(subscription) => {
            return Ok(subscription);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string()
            ))
        }
    }
}

#[delete("/subscription/<id>")]
pub async fn delete_subscription(
    connection: MysqlConnection,
    id: i32
) -> Result<Json<Subscription>, (Status, String)> {
    match connection.run(
       move |c| c.transaction(|c| {
            let sub = subscriptions::table.find(id).first::<Subscription>(c).map(Json)?;

            diesel::delete(subscriptions::table.find(id)).execute(c)?;

            diesel::result::QueryResult::Ok(sub)
        })
    ).await {
        Ok(sub) => {
            return Ok(sub);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string()
            ))
        }
    }
}