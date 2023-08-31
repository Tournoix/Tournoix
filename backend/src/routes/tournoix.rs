use crate::models::tournament::{NewTournament, PatchTournament, Tournament};
use crate::schema::tournaments;
use crate::MysqlConnection;
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
) -> Result<Json<Tournament>, (Status, String)> {
    match connection
        .run(move |c| tournaments::table.find(id).first::<Tournament>(c))
        .await
        .map(Json)
    {
        Ok(tournoi) => return Ok(tournoi),

        Err(_e) => return Err((Status::NotFound, "Tournament not found".to_string())),
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddTournament {
    pub fk_users: i32,
    pub name: String,
    pub description: String,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub phase: i32,
    pub size_group: Option<i32>,
    pub code: String,
}

#[post("/tournoix", data = "<data>")]
pub async fn create_tournoix(
    connection: MysqlConnection,
    data: Json<AddTournament>,
) -> Result<Json<Tournament>, (Status, String)> {
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
            Err(_) => return Err((Status::InternalServerError, "Database error".to_string())),
        };
    }

    let add_tournoix = data.0;

    let tournoix = NewTournament {
        fk_users: add_tournoix.fk_users,
        name: add_tournoix.name,
        description: add_tournoix.description,
        date: add_tournoix.date,
        location: add_tournoix.location,
        phase: add_tournoix.phase,
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
                "Internel Server Error".to_string(),
            ))
        }
    }
}

#[patch("/tournoix/<id>", data = "<data>")]
pub async fn update_tournoix(
    connection: MysqlConnection,
    data: Json<PatchTournament>,
    id: i32,
) -> Result<Json<Tournament>, (Status, String)> {
    let tournoix = data.0;

    match connection
        .run(move |c| {
            c.transaction(|c| {
                diesel::update(tournaments::table.find(id))
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
) -> Result<Json<Tournament>, (Status, String)> {
    match connection
        .run(move |c| {
            c.transaction(|c| {
                let tournoix = tournaments::table
                    .find(id)
                    .first::<Tournament>(c)
                    .map(Json)?;

                diesel::delete(tournaments::table.find(id)).execute(c)?;

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

/*
// Get all tournaments for a user (organizer) (GET /tournoix-by-organizer)
#[get("/tournoix-by-organizer")]
pub async fn get_tournoix_by_organizer(
    connection: MysqlConnection,
    key: Result<JWT, NetworkResponse>
) -> Result<Json<Vec<Tournament>>, NetworkResponse> {
    // Check key validity and get user id
    let id = match key {
        Ok(key) => {
            match connection.run(
                move |c| tokens::table.filter(tokens::token.eq(&key.claims.jti)).filter(tokens::fk_users.eq(&key.claims.id)).first::<Token>(c)
            ).await {
                Ok(token) => {
                    // Check if token is expired
                    println!("{:?}", token.expiration_date);
                    println!("{:?}", chrono::Local::now().naive_local());

                    if token.expiration_date < chrono::Local::now().naive_local() {
                        return Err(NetworkResponse::Unauthorized(("Token expired".to_string())))
                    }
                    token.fk_users
                },
                Err(e) => return Err(NetworkResponse::Unauthorized(("Invalid token".to_string())))
            }
        },
        Err(e) => return Err(e),
    };

    match connection.run(
        move |c| tournaments::table.filter(tournaments::fk_users.eq(id)).get_results::<Tournament>(c)
    ).await.map(Json) {
        Ok(tournoix) => {
           return Ok(tournoix)
        },

        Err(_e) => {
            return Err(NetworkResponse::NotFound("No tournament found".to_string()))
        }
    }
}
*/
