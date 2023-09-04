use crate::models::nut::{NewNut, Nut};
use crate::models::subscription::{NewSubscription, Subscription};
use crate::models::tournament::Tournament;
use crate::routes::auth::ApiAuth;
use crate::schema::{nuts, subscriptions, tournaments};
use crate::MysqlConnection;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

// get all tournament created by the user
#[get("/users/@me/tournoix")]
pub async fn get_user_tournoix(
    connection: MysqlConnection,
    auth: ApiAuth
) -> Result<Json<Vec<Tournament>>, (Status, String)> {
    match connection
        .run(move |c| {
            tournaments::table
                .filter(tournaments::fk_users.eq(auth.user.id))
                .load::<Tournament>(c)
        })
        .await
        .map(Json)
    {
        Ok(tournoi) => return Ok(tournoi),

        Err(_e) => return Err((Status::NotFound, "No created tournament found".to_string())),
    }
}

// get all tournament subscribed by the user
#[get("/users/@me/subscriptions")]
pub async fn get_user_subscription(
    connection: MysqlConnection,
    auth: ApiAuth
) -> Result<Json<Vec<Tournament>>, (Status, String)> {
    // get all subsciptions for the user
    let tab_subscriber = match connection
        .run(move |c| {
            subscriptions::table
                .filter(subscriptions::fk_users.eq(auth.user.id))
                .load::<Subscription>(c)
        })
        .await
    {
        Ok(subscribers) => subscribers,
        Err(_) => {
            return Err((
                Status::NotFound,
                "Subscription to tournament not found".to_string(),
            ))
        }
    };

    // vector to store all tournaments linked to the user
    let mut tournoix_vec = Vec::new();

    // for each subscription, get the tournament
    for subscriber in tab_subscriber {
        let tournaments = match connection
            .run(move |c| tournaments::table.find(subscriber.id).load::<Tournament>(c))
            .await
        {
            Ok(tournaments) => tournaments,
            Err(_) => return Err((Status::NotFound, "Wrong tournament id".to_string())),
        };

        tournoix_vec.extend(tournaments);
    }

    Ok(Json(tournoix_vec))
}

// create a subscription for a tournament with the code of the tournament and the id of the user
// + add the nuts to the user for the tournament if he doesn't have one allready
#[post("/users/@me/subscription", data = "<code>")]
pub async fn create_subsciption(
    connection: MysqlConnection,
    code: String,
    auth: ApiAuth
) -> Result<Json<Subscription>, (Status, String)> {
    // verify the existance of the code in the database
    let tournament = match connection
        .run(move |c| {
            tournaments::table
                .filter(tournaments::code.eq(code))
                .first::<Tournament>(c)
        })
        .await
    {
        Ok(tournament) => tournament,
        Err(_) => return Err((Status::NotFound, "Wrong code".to_string())),
    };

    // create the subscription
    let subscription = NewSubscription {
        fk_users: auth.user.id,
        fk_tournaments: tournament.id,
    };

    // verify if the user allready has/had nuts for this tournament
    match connection
        .run(move |c| {
            nuts::table
                .filter(nuts::fk_users.eq(auth.user.id))
                .filter(nuts::fk_tournaments.eq(tournament.id))
                .first::<Nut>(c)
        })
        .await
    {
        Ok(_) => (), // User already has received nuts for this tournament
        Err(_) => {
            // add the nuts to the user
            let nut = NewNut {
                fk_users: auth.user.id,
                fk_tournaments: tournament.id,
                stock: 20,
            };

            match connection
                .run(move |c| diesel::insert_into(nuts::table).values(nut).execute(c))
                .await
            {
                Ok(_) => (),
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        "Internel Server Error".to_string(),
                    ))
                }
            }
        }
    };

    // insert the subscription in the database
    match connection
        .run(move |c| {
            c.transaction(|c| {
                diesel::insert_into(subscriptions::table)
                    .values(subscription)
                    .execute(c)?;

                let subscription = subscriptions::table
                    .filter(subscriptions::fk_tournaments.eq(tournament.id))
                    .filter(subscriptions::fk_users.eq(auth.user.id))
                    .first::<Subscription>(c)
                    .map(Json)?;

                diesel::result::QueryResult::Ok(subscription)
            })
        })
        .await
    {
        Ok(subscription) => {
            return Ok(subscription);
        }

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string(),
            ))
        }
    }
}

// delete a subscription with the id of the tournament and the id of the user
#[delete("/subscription/<id_tournament>")]
pub async fn delete_subscription(
    connection: MysqlConnection,
    id_tournament: i32,
    auth: ApiAuth
) -> Result<Json<Subscription>, (Status, String)> {
    match connection
        .run(move |c| {
            c.transaction(|c| {
                let sub = subscriptions::table
                    .filter(subscriptions::fk_tournaments.eq(id_tournament))
                    .filter(subscriptions::fk_users.eq(auth.user.id))
                    .first::<Subscription>(c)
                    .map(Json)?;

                diesel::delete(subscriptions::table
                    .find(sub.id))
                    .execute(c)?;

                diesel::result::QueryResult::Ok(sub)
            })
        })
        .await
    {
        Ok(sub) => {
            return Ok(sub);
        }

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "Internel Server Error".to_string(),
            ))
        }
    }
}