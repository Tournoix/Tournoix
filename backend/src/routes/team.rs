use crate::models::game::Game;
use crate::models::subscription::Subscription;
use crate::models::team::*;
use crate::models::tournament::Tournament;
use crate::schema::teams::fk_tournaments;
use crate::schema::{games, subscriptions, teams, tournaments};
use crate::{EmptyResponse, ErrorBody, ErrorResponse, MysqlConnection};
use diesel::prelude::*;
use log::warn;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use super::auth::ApiAuth;
use super::tournoix::{is_owner, tournament_is_started};

// get all team from a tournament
#[get("/tournoix/<id>/teams")]
pub async fn get_teams(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<Vec<Team>>, (Status, String)> {
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
            "{} - User {} tried to access teams of tournament {} - routes/team/get_teams()",
            chrono::Local::now().format("%d/%m/%Y %H:%M"),
            auth.user.id,
            id
        );
        return Err((Status::Forbidden, "Access Forbidden".to_string()));
    }

    match connection
        .run(move |c| teams::table.filter(fk_tournaments.eq(id)).load::<Team>(c))
        .await
        .map(Json)
    {
        Ok(teams) => return Ok(teams),

        Err(_e) => return Err((Status::NotFound, "No teams found".to_string())),
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddTeam {
    pub name: String,
    pub group: i32,
}

#[post("/tournoix/<id>/teams", data = "<data>")]
pub async fn create_team(
    connection: MysqlConnection,
    data: Json<AddTeam>,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<Team>, (Status, Json<ErrorResponse>)> {
    // Check if the user is the owner of the tournament
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
            warn!(
                "{} - User {} tried to create a team for tournament {} - routes/team/create_team()",
                chrono::Local::now().format("%d/%m/%Y %H:%M"),
                auth.user.id,
                id
            );
            return Err((
                Status::Forbidden,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 403,
                        reason: "Forbidden".into(),
                        description: "Access Forbidden".into(),
                    },
                }),
            ));
        }
    };

    // cannot create a team if the tournament is started
    if tournament_is_started(&connection, id).await {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: 400,
                    reason: "Bad Request".into(),
                    description: "Cannot create a team as the tournament has started".into(),
                },
            }),
        ));
    }

    let team = NewTeam {
        fk_tournaments: id,
        name: data.0.name,
        group: data.0.group,
    };

    match connection
        .run(move |c| {
            c.transaction(|c| {
                diesel::insert_into(teams::table)
                    .values(team.clone())
                    .execute(c)?;

                let team = teams::table
                    .order(teams::id.desc())
                    .first::<Team>(c)
                    .map(Json)?;

                diesel::result::QueryResult::Ok(team)
            })
        })
        .await
    {
        Ok(team) => {
            return Ok(team);
        }

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 500,
                        reason: "Internal Server Error".into(),
                        description: "An error occured".into(),
                    },
                }),
            ))
        }
    }
}

#[patch("/teams/<id>", data = "<data>")]
pub async fn update_team(
    connection: MysqlConnection,
    data: Json<PatchTeam>,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<Team>, (Status, Json<ErrorResponse>)> {
    match connection
        .run(move |c| teams::table.find(id).first::<Team>(c))
        .await
    {
        Ok(team) => {
            // Check user permission
            let tournament = connection
                .run(move |c| {
                    tournaments::table
                        .find(team.fk_tournaments)
                        .first::<Tournament>(c)
                })
                .await
                .unwrap();

            if auth.user.id != tournament.fk_users {
                warn!("{} - User {} tried to update team of tournament {} - routes/team/update_team()", chrono::Local::now().format("%d/%m/%Y %H:%M"), auth.user.id, id);
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

            let team_data = data.0;

            // if it s an update on the group, we cannot update it since there is games
            if tournament_is_started(&connection, tournament.id).await && team_data.group.is_some()
            {
                return Err((
                    Status::BadRequest,
                    Json(ErrorResponse {
                        error: ErrorBody {
                            code: 400,
                            reason: "Bad Request".into(),
                            description: "Cannot update the group if tournament has started".into(),
                        },
                    }),
                ));
            }

            match connection
                .run(move |c| {
                    c.transaction(|c| {
                        diesel::update(teams::table)
                            .filter(teams::id.eq(id))
                            .set(team_data)
                            .execute(c)?;

                        let team = teams::table
                            .order(teams::id.desc())
                            .first::<Team>(c)
                            .map(Json)?;

                        diesel::result::QueryResult::Ok(team)
                    })
                })
                .await
            {
                Ok(team) => {
                    return Ok(team);
                }

                Err(_e) => {
                    return Err((
                        Status::InternalServerError,
                        Json(ErrorResponse {
                            error: ErrorBody {
                                code: 500,
                                reason: "Internal Server Error".into(),
                                description: "An error occured".into(),
                            },
                        }),
                    ))
                }
            }
        }
        Err(_e) => {
            return Err((
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 500,
                        reason: "Internal Server Error".into(),
                        description: "An error occured".into(),
                    },
                }),
            ))
        }
    };
}

#[delete("/teams/<id>")]
pub async fn delete_team(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<EmptyResponse>, (Status, Json<ErrorResponse>)> {
    match connection
        .run(move |c| teams::table.find(id).first::<Team>(c))
        .await
    {
        Ok(team) => {
            // Check user permission
            let tournament = connection
                .run(move |c| {
                    tournaments::table
                        .find(team.fk_tournaments)
                        .first::<Tournament>(c)
                })
                .await
                .unwrap();

            if auth.user.id != tournament.fk_users {
                warn!(
                    "{} - User {} tried to delete team {} - routes/team/delete_team()",
                    chrono::Local::now().format("%d/%m/%Y %H:%M"),
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

            // if there is allready a match for this team, we can't delete it
            if let Some(_game) = connection
                .run(move |c| {
                    games::table
                        .filter(games::fk_team1.eq(id))
                        .or_filter(games::fk_team2.eq(id))
                        .first::<Game>(c)
                })
                .await
                .ok()
            {
                return Err((
                    Status::BadRequest,
                    Json(ErrorResponse {
                        error: ErrorBody {
                            code: 400,
                            reason: "Bad Request".into(),
                            description: "Cannot remove the team, this team is in a match".into(),
                        },
                    }),
                ));
            }

            match connection
                .run(move |c| diesel::delete(teams::table.find(&id)).execute(c))
                .await
            {
                Ok(_) => {
                    return Ok(Json(EmptyResponse()));
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
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 500,
                        reason: "Internal Server Error".into(),
                        description: "An error occured".into(),
                    },
                }),
            ))
        }
    };
}
