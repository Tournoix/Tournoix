use crate::models::tournament::{NewTournament, PatchTournament, Tournament};
use crate::routes::auth::ApiAuth;
use crate::schema::tournaments;
use crate::{ErrorBody, ErrorResponse, MysqlConnection};
use diesel::prelude::*;
use diesel::result::Error;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

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
            return Err((
                Status::NotFound,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 404,
                        reason: "Not Found".into(),
                        description: "Tournament with given id does not exists".into(),
                    },
                }),
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddTournament {
    pub name: String,
    pub description: Option<String>,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub size_group: Option<i32>,
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
) -> Result<Json<Tournament>, (Status, String)> {
    // verify if the user is the owner of the tournament
    if !is_owner(&connection, id, &auth).await {
        return Err((Status::Unauthorized, "Unauthorized".to_string()));
    }

    let tournoix = data.0;

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
                "Internel Server Error".to_string(),
            ))
        }
    }
}

#[delete("/tournoix/<id>")]
pub async fn delete_tournoix(
    connection: MysqlConnection,
    id: i32,
    auth: ApiAuth,
) -> Result<Status, (Status, Json<ErrorResponse>)> {
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
        Ok(_r) => Ok(Status::NoContent),

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
