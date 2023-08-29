use diesel::{prelude::*, insert_into};
use rocket::http::Status;
use crate::{schema::{users::{self, email}, tokens}, models::user::NewUser};
use crate::models::{user::User, token::NewToken};
use rocket::serde::json::Json;
use crate::{MysqlConnection, crypto};
use rocket::serde::{Deserialize, Serialize};
use chrono::Duration;


// Login user
#[derive(Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub expiration_date: chrono::NaiveDateTime
}

#[post("/auth/login", data = "<data>")]
pub async fn login(
    connection: MysqlConnection,
    data: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (Status, String)> {
    // Find user by email
    match connection.run(
        {
            let data = data.clone();
            move |c| users::table.filter(email.eq(&data.email)).first::<User>(c)
        }
    ).await {
        Ok(user) => {
            // Check if password is correct   
            if crypto::verify_password(&user.password, &data.password) {
                // Generate token
                let token_string = crypto::generate_token();

                // Add token to db
                let token = NewToken {
                    token: token_string,
                    fk_users: user.id,
                    expiration_date: (chrono::offset::Local::now() + Duration::hours(3)).naive_local()
                };

                match connection.run(
                    {
                        let token = token.clone();
                        move |c| insert_into(tokens::dsl::tokens).values(token).execute(c)
                    }
                ).await {
                    Ok(_) => {
                        let reponse = LoginResponse {
                            token: token.token,
                            expiration_date: token.expiration_date
                        };
        
                        // Return token
                        return Ok(Json(reponse));
                    },

                    Err(e) => {
                        return Err((
                            Status::InternalServerError,
                            e.to_string()
                        ))
                    }
                }
            } else {
                return Err((
                    Status::Unauthorized,
                    "incorrect login".to_string()
                ))
            }
        },

        Err(_e) => {
            return Err((
                Status::Unauthorized,
                "incorrect login".to_string()
            ))
        }
    }
}

// Logout user
#[derive(Deserialize, Clone)]
pub struct LogoutRequest {
    pub token: String
}

#[post("/auth/logout", data = "<data>")]
pub async fn logout(
    connection: MysqlConnection,
    data: Json<LogoutRequest>,
) -> Result<Json<()>, (Status, String)> {
    // Delete token from db
    match connection.run(
        {
            let data = data.clone();
            move |c| diesel::delete(tokens::table.filter(tokens::token.eq(&data.token))).execute(c)
        }
    ).await {
        Ok(_) => {
            return Ok(Json(()))
        },

        Err(e) => {
            return Err((
                Status::InternalServerError,
                e.to_string()
            ))
        }
    }
}


#[post("/auth/register", data = "<data>")]
pub async fn register(
    connection: MysqlConnection,
    data: Json<User>,
) -> Result<Json<User>, (Status, String)> {
    // Check if email is already used
    match connection.run(
        {
            let data = data.clone();
            move |c| users::table.filter(email.eq(&data.email)).first::<User>(c)
        }
    ).await {
        Ok(_user) => {
            return Err((
                Status::Conflict,
                "email already used".to_string()
            ))
        },

        Err(_e) => {
            // Hash password and check if return error
            let password = match crypto::hash_password(&data.password) {
                Ok(password) => password,
                Err(e) => {
                    return Err((
                        Status::InternalServerError,
                        e.to_string()
                    ))
                }
            };

            // If no error, create user
            let new_user = NewUser {
                name: data.name.clone(),
                email: data.email.clone(),
                password: password,
            };

            // Add user to db
            match connection.run(
                {
                    let user = new_user.clone();
                    move |c| insert_into(users::dsl::users).values(user).execute(c)
                }
            ).await {
                Ok(_) => {
                    return Ok(Json(data.into_inner()))
                },

                Err(e) => {
                    return Err((
                        Status::InternalServerError,
                        e.to_string()
                    ))
                }
            }
        }
    }
}