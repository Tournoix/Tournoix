use crate::models::token::{NewToken, Token};
use crate::models::user::{User, UserInfo};
use crate::{crypto, ErrorBody, ErrorResponse, MysqlConnection};
use crate::{
    models::user::NewUser,
    schema::{
        tokens,
        users::{self, email},
    },
};
use chrono::{Duration, Local};
use diesel::{insert_into, prelude::*};

use log::{info, warn};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{request, Request};

#[derive(Responder, Debug)]
pub enum ApiAuthResponse {
    #[response(status = 401)]
    Unauthorized(String),
}

/// Struct used for guarding request
/// Contains the api token and the user
pub struct ApiAuth {
    pub token: String,
    pub user: UserInfo,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiAuth {
    type Error = ApiAuthResponse;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            // No token => 401
            0 => {
                warn!(
                    "{} - No authorization header found",
                    Local::now().format("%d/%m/%Y %H:%M")
                );
                Outcome::Failure((
                    Status::Unauthorized,
                    ApiAuthResponse::Unauthorized("No authorization header found".to_string()),
                ))
            }
            1 => {
                let connection = MysqlConnection::from_request(&request).await.unwrap();
                let token_str: String = keys[0][7..].into();

                match connection
                    .run(move |c| tokens::table.find(&token_str).first::<Token>(c))
                    .await
                {
                    Ok(token) => {
                        if token.expiration_date < chrono::Local::now().naive_local() {
                            // Token is expired => 401 and delete the token
                            connection
                                .run(move |c| {
                                    diesel::delete(tokens::table.find(token.token)).execute(c)
                                })
                                .await
                                .ok();

                            return Outcome::Failure((
                                Status::Unauthorized,
                                ApiAuthResponse::Unauthorized("Token expired".to_string()),
                            ));
                        }

                        match connection
                            .run(move |c| {
                                users::table
                                    .find(token.fk_users)
                                    .select((users::id, users::name, users::email))
                                    .first::<UserInfo>(c)
                            })
                            .await
                        {
                            Ok(user) => Outcome::Success(ApiAuth {
                                token: token.token,
                                user,
                            }),

                            Err(_e) => {
                                // Erro while getting the user linked to the token => 401
                                Outcome::Failure((
                                    Status::Unauthorized,
                                    ApiAuthResponse::Unauthorized("Invalid token".to_string()),
                                ))
                            }
                        }
                    }

                    Err(_e) => {
                        // Error while getting the token (either it doesn't exist or DB related error) => 401
                        Outcome::Failure((
                            Status::Unauthorized,
                            ApiAuthResponse::Unauthorized("Invalid token".to_string()),
                        ))
                    }
                }
            }
            _ => Outcome::Failure((
                Status::Unauthorized,
                ApiAuthResponse::Unauthorized("No authorization header found".to_string()),
            )),
        }
    }
}

// Login user
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub expiration_date: chrono::NaiveDateTime,
}

#[post("/auth/login", data = "<data>")]
pub async fn login(
    connection: MysqlConnection,
    data: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (Status, Json<ErrorResponse>)> {
    // Find user by email
    match connection
        .run({
            let data = data.clone();
            move |c| users::table.filter(email.eq(&data.email)).first::<User>(c)
        })
        .await
    {
        Ok(user) => {
            // Check if password is correct
            if crypto::verify_password(&user.password, &data.password) {
                // Generate token
                let token_string = crypto::generate_token();

                // Add token to db
                let token = NewToken {
                    token: token_string,
                    fk_users: user.id,
                    expiration_date: (chrono::offset::Local::now() + Duration::hours(3))
                        .naive_local(),
                };

                match connection
                    .run({
                        let token = token.clone();
                        move |c| insert_into(tokens::dsl::tokens).values(token).execute(c)
                    })
                    .await
                {
                    Ok(_) => {
                        let reponse = LoginResponse {
                            token: token.token,
                            expiration_date: token.expiration_date,
                        };
                        info!(
                            "{} - User {} logged in",
                            Local::now().format("%d/%m/%Y %H:%M"),
                            user.id
                        );
                        // Return token
                        return Ok(Json(reponse));
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
            } else {
                warn!(
                    "{} - User {} tried to login with incorrect password",
                    Local::now().format("%d/%m/%Y %H:%M"),
                    user.id
                );
                return Err((
                    Status::Unauthorized,
                    Json(ErrorResponse {
                        error: ErrorBody {
                            code: 401,
                            reason: "Unauthorized".into(),
                            description: "Email or password incorrect".into(),
                        },
                    }),
                ));
            }
        }

        Err(_e) => {
            warn!(
                "{} - User with email {} tried to login but doesn't exist",
                Local::now().format("%d/%m/%Y %H:%M"),
                data.email
            );
            return Err((
                Status::Unauthorized,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 401,
                        reason: "Unauthorized".into(),
                        description: "Email or password incorrect".into(),
                    },
                }),
            ));
        }
    }
}

// Logout user
#[derive(Deserialize, Clone)]
pub struct LogoutRequest {
    pub token: String,
}

#[post("/auth/logout")]
pub async fn logout(
    connection: MysqlConnection,
    auth: ApiAuth,
) -> Result<Json<String>, (Status, Json<ErrorResponse>)> {
    // Delete token from db
    match connection
        .run(move |c| diesel::delete(tokens::table.find(auth.token)).execute(c))
        .await
    {
        Ok(_) => {
            info!(
                "{} - User {} logged out",
                Local::now().format("%d/%m/%Y %H:%M"),
                auth.user.id
            );
            return Ok(Json("Logout successfull".into()));
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

#[post("/auth/register", data = "<data>")]
pub async fn register(
    connection: MysqlConnection,
    data: Json<NewUser>,
) -> Result<Json<UserInfo>, (Status, Json<ErrorResponse>)> {
    // Check if email is already used
    match connection
        .run({
            let data = data.clone();
            move |c| users::table.filter(email.eq(&data.email)).first::<User>(c)
        })
        .await
    {
        Ok(_user) => {
            return Err((
                Status::Conflict,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: 409,
                        reason: "Conflict".into(),
                        description: "Email already used".to_string(),
                    },
                }),
            ))
        }

        Err(_e) => {
            // Hash password and check if return error
            let hashed_password = match crypto::hash_password(&data.password.as_str()) {
                Ok(hashed_password) => hashed_password,
                Err(_e) => {
                    return Err((
                        Status::InternalServerError,
                        Json(ErrorResponse {
                            error: ErrorBody {
                                code: 500,
                                reason: "Internal Server Error".into(),
                                description: "An error has occured".to_string(),
                            },
                        }),
                    ))
                }
            };

            // If no error, create user
            let new_user = NewUser {
                name: data.name.clone(),
                email: data.email.clone(),
                password: hashed_password,
            };

            // Add user to db
            match connection
                .run({
                    let user = new_user.clone();
                    move |c| {
                        c.transaction(|c| {
                            insert_into(users::dsl::users).values(user).execute(c)?;

                            let user = users::table
                                .order(users::id.desc())
                                .select((users::id, users::name, users::email))
                                .first::<UserInfo>(c)
                                .map(Json)?;

                            diesel::result::QueryResult::Ok(user)
                        })
                    }
                })
                .await
            {
                Ok(user) => {
                    info!(
                        "{} - User {} registered",
                        Local::now().format("%d/%m/%Y %H:%M"),
                        user.id
                    );
                    return Ok(user);
                }

                Err(_e) => {
                    return Err((
                        Status::InternalServerError,
                        Json(ErrorResponse {
                            error: ErrorBody {
                                code: 500,
                                reason: "Internal Server Error".into(),
                                description: "An error has occured".to_string(),
                            },
                        }),
                    ))
                }
            }
        }
    }
}
