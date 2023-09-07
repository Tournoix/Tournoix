use crate::models::game::Game;
use crate::models::nut::NewNut;
use crate::models::team::Team;
use crate::models::tournament::{NewTournament, PatchTournament, Tournament};
use crate::routes::auth::ApiAuth;
use crate::schema::{games, nuts, teams, tournaments};
use crate::{EmptyResponse, ErrorBody, ErrorResponse, MysqlConnection};
use diesel::prelude::*;
use diesel::result::Error;
use log::info;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[get("/tournoix/<id>/me@/is_owner")]
pub async fn get_tournoix_is_owner(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<bool>, (Status, Json<ErrorResponse>)> {
    return Ok(Json(!is_owner(&connection, id, &auth).await));
}

#[get("/tournoix/<id>")]
pub async fn get_tournoix(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<Tournament>, (Status, Json<ErrorResponse>)> {
    match connection
        .run(move |c| tournaments::table.find(id).first::<Tournament>(c))
        .await
    {
        Ok(tournoi) => {
            if tournoi.user_has_rights(&connection, auth.user).await {
                Ok(Json(tournoi))
            } else {
                Err((
                    Status::Forbidden,
                    Json(ErrorResponse {
                        error: ErrorBody {
                            code: 403,
                            reason: "Forbidden".into(),
                            description: "Access forbidden".into(),
                        },
                    }),
                ))
            }
        }

        Err(_e) => {
            info!("{} - User {} tried to access non-existing tournament {} - routes/tournoix/get_tournoix()", chrono::Local::now().format("%d/%m/%Y %H:%M"), auth.user.id, id);

            return Err((
                Status::NotFound,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 404,
                        reason: "Not Found".into(),
                        description: "Tournament with given id does not exists".into(),
                    },
                }),
            ));
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddTournament {
    pub name: String,
    pub description: Option<String>,
    pub date: chrono::NaiveDateTime,
    pub location: Option<String>,
    pub size_group: Option<i32>,
    pub is_qualif: bool,
    pub is_elim: bool,
}

#[post("/tournoix", data = "<data>")]
pub async fn create_tournoix(
    connection: MysqlConnection,
    data: Json<AddTournament>,
    auth: ApiAuth,
) -> Result<Json<Tournament>, (Status, Json<ErrorResponse>)> {
    let mut generated_code = String::new();
    let mut code_exist = true;

    while code_exist {
        generated_code = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        // Check if the code already exists in the database
        match connection
            .run({
                let generated_code = generated_code.clone();
                move |c| {
                    tournaments::table
                        .filter(tournaments::code.eq(generated_code))
                        .first::<Tournament>(c)
                }
            })
            .await
        {
            Ok(_) => {
                // The code exists, generate a new one in the next iteration
                continue;
            }
            Err(Error::NotFound) => {
                // The code doesn't exist, break the loop
                code_exist = false;
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

    let add_tournoix = data.0;

    let tournoix = NewTournament {
        fk_users: auth.user.id,
        name: add_tournoix.name,
        description: add_tournoix.description,
        date: add_tournoix.date,
        location: add_tournoix.location,
        phase: 0,
        size_group: add_tournoix.size_group,
        code: generated_code, // Use the generated code
        is_qualif: add_tournoix.is_qualif,
        is_elim: add_tournoix.is_elim,
    };

    match connection
        .run(move |c| {
            c.transaction(|c| {
                diesel::insert_into(tournaments::table)
                    .values(tournoix.clone())
                    .execute(c)?;

                let tournoix = tournaments::table
                    .order(tournaments::id.desc())
                    .first::<Tournament>(c)
                    .map(Json)?;

                diesel::result::QueryResult::Ok(tournoix)
            })
        })
        .await
    {
        Ok(tournoix) => {
            info!(
                "{} - User {} created tournament {} - routes/tournoix/create_tournoix()",
                chrono::Local::now().format("%d/%m/%Y %H:%M"),
                auth.user.id,
                tournoix.id
            );

            // Add nuts to tournament owner
            let nut = NewNut {
                fk_users: auth.user.id,
                fk_tournaments: tournoix.id,
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

            return Ok(tournoix);
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

#[patch("/tournoix/<id>", data = "<data>")]
pub async fn update_tournoix(
    connection: MysqlConnection,
    data: Json<PatchTournament>,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<Tournament>, (Status, Json<ErrorResponse>)> {
    // verify if the user is the owner of the tournament
    if !is_owner(&connection, id, &auth).await {
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

    let tournoix = data.0;
    let actual_tournoix = match connection
        .run(move |c| tournaments::table.find(id).first::<Tournament>(c))
        .await
    {
        Ok(tournoi) => tournoi,

        Err(_e) => {
            return Err((
                Status::NotFound,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 404,
                        reason: "Not Found".into(),
                        description: "Tournament with given id does not exists".into(),
                    },
                }),
            ));
        }
    };

    if tournament_is_started(&connection, id).await
        && ((tournoix.is_elim.is_some() && tournoix.is_elim.unwrap() != actual_tournoix.is_elim)
            || (tournoix.is_qualif.is_some()
                && tournoix.is_qualif.unwrap() != actual_tournoix.is_qualif))
    {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: 400,
                    reason: "Bad Request".into(),
                    description: "Cannot modify tournament structure when already started".into(),
                },
            }),
        ));
    }

    match connection
        .run(move |c| {
            c.transaction(|c| {
                diesel::update(
                    tournaments::table
                        .find(id)
                        .filter(tournaments::fk_users.eq(auth.user.id)),
                )
                .set(tournoix.clone())
                .execute(c)?;

                let tournoix = tournaments::table
                    .order(tournaments::id.desc())
                    .first::<Tournament>(c)
                    .map(Json)?;

                diesel::result::QueryResult::Ok(tournoix)
            })
        })
        .await
    {
        Ok(tournoix) => {
            return Ok(tournoix);
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

#[delete("/tournoix/<id>")]
pub async fn delete_tournoix(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Json<EmptyResponse>, (Status, Json<ErrorResponse>)> {
    // verify if the user is the owner of the tournament
    if !is_owner(&connection, id, &auth).await {
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

    match connection
        .run(move |c| diesel::delete(tournaments::table.find(id)).execute(c))
        .await
    {
        Ok(_r) => Ok(Json(EmptyResponse())),

        Err(_e) => Err((
            Status::InternalServerError,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: 500,
                    reason: "Internel Server Error".into(),
                    description: "An error occured".into(),
                },
            }),
        )),
    }
}

// verify if the user is the owner of the tournament
pub async fn is_owner(connection: &MysqlConnection, id: i32, auth: &ApiAuth) -> bool {
    let auth_id = auth.user.id;
    match connection
        .run(move |c| {
            tournaments::table
                .find(id)
                .filter(tournaments::fk_users.eq(auth_id))
                .first::<Tournament>(c)
        })
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn tournament_is_started(connection: &MysqlConnection, id: i32) -> bool {
    // verify if the tournament has games
    match connection
        .run(move |c| {
            games::table
                .inner_join(teams::table.on(games::fk_team1.eq(teams::id)))
                .filter(teams::fk_tournaments.eq(id))
                .load::<(Game, Team)>(c)
        })
        .await
    {
        Ok(games) => games.len() > 0,
        Err(_) => false,
    }
}
