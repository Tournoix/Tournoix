use std::env;
use std::f32::consts::E;
use crate::models::token::{NewToken, Token};
use crate::models::user::User;
use crate::{crypto, MysqlConnection};
use crate::{
    models::user::NewUser,
    schema::{
        tokens,
        users::{self, email, password},
    },
};
use chrono::{Duration, Utc};
use diesel::{insert_into, prelude::*, connection};
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Responder;
use rocket::request::{Outcome, Request, FromRequest};


#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i32,
    pub jti: String,
    exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl <'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::Unauthorized, NetworkResponse::Unauthorized("No authorization header found".to_string()))),
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(_e) => match &_e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        Outcome::Failure((Status::Unauthorized, NetworkResponse::Unauthorized("Token expired".to_string())))
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        println!("Invalid token - {}", key);
                        Outcome::Failure((Status::Unauthorized, NetworkResponse::Unauthorized("Invalid token".to_string())))
                    },
                    _ => Outcome::Failure((Status::Unauthorized, NetworkResponse::Unauthorized(format!("Invalid token - {}", _e)))),
                }
            }
        }
    }
}


// Login user
#[derive(Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    // pub expiration_date: chrono::NaiveDateTime,
}

pub fn create_jwt(id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(6))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        id: id,
        jti: crypto::generate_uuid(),
        exp: expiration as usize,
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}

pub async fn login_user(
    connection: MysqlConnection,
    data: Json<LoginRequest>,
) -> Result<String, NetworkResponse> {
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
                    match create_jwt(user.id) {
                        Ok(token) => {
                            // Add token.jti to db (for logout)
                            let tmp_token = NewToken {
                                token: decode_jwt(token.clone()).unwrap().jti,
                                fk_users: user.id,
                                expiration_date: (chrono::offset::Local::now() + Duration::hours(3))
                                    .naive_local(),
                            };

                            match connection
                                .run({
                                    move |c| insert_into(tokens::dsl::tokens).values(tmp_token).execute(c)
                                })
                                .await
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err(NetworkResponse::Unauthorized(e.to_string()));
                                }
                            }


                            // Return token
                            return Ok(token);
                        }
                        Err(_e) => {
                            return Err(NetworkResponse::Unauthorized("incorrect login".to_string()));
                        }
                    }
                } else {
                    return Err(NetworkResponse::Unauthorized("incorrect login".to_string()));
                }
            }
    
            Err(_e) => return Err(NetworkResponse::Unauthorized("incorrect login".to_string())),
        }
}

#[post("/auth/login", data = "<data>")]
pub async fn login(
    connection: MysqlConnection,
    data: Json<LoginRequest>,
) -> Result<String, NetworkResponse> {
    let token = login_user(connection, data);

    let response = LoginResponse {
        token: token.await.unwrap(),
    };

    Ok(response.token)
}

/* #[post("/auth/login", data = "<data>")]
pub async fn login(
    connection: MysqlConnection,
    data: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (Status, String)> {
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

                        // Return token
                        return Ok(Json(reponse));
                    }

                    Err(e) => return Err((Status::InternalServerError, e.to_string())),
                }
            } else {
                return Err((Status::Unauthorized, "incorrect login".to_string()));
            }
        }

        Err(_e) => return Err((Status::Unauthorized, "incorrect login".to_string())),
    }
} */

// Logout user
#[derive(Deserialize, Clone)]
pub struct LogoutRequest {
    pub token: String,
}

/* #[post("/auth/logout", data = "<data>")]
pub async fn logout(
    connection: MysqlConnection,
    data: Json<LogoutRequest>,
) -> Result<Json<()>, (Status, String)> {
    // Delete token from db
    match connection
        .run({
            let data = data.clone();
            move |c| diesel::delete(tokens::table.filter(tokens::token.eq(&data.token))).execute(c)
        })
        .await
    {
        Ok(_) => return Ok(Json(())),

        Err(e) => return Err((Status::InternalServerError, e.to_string())),
    }
} */

#[post("/auth/logout")]
pub async fn logout(
    connection: MysqlConnection,
    key: Result<JWT, NetworkResponse>
) -> Result<Json<()>, NetworkResponse> {
    
    // Check key validity and get user id
    let id = match key {
        Ok(key) => {
            // Check if token is in db
            match connection
                .run({
                    move |c| tokens::table.filter(tokens::token.eq(&key.claims.jti)).filter(tokens::fk_users.eq(&key.claims.id)).first::<Token>(c)
                })
                .await
            {
                Ok(token) => token.fk_users,
                Err(e) => return Err(NetworkResponse::Unauthorized(e.to_string())),
            }
        },
        Err(e) => return Err(e),
    };

    // Delete token from db
    match connection
        .run({
            move |c| diesel::delete(tokens::table.filter(tokens::fk_users.eq(id))).execute(c)
        })
        .await
    {
        Ok(_) => return Ok(Json(())),

        Err(e) => return Err(NetworkResponse::BadRequest(("Internel Server Error".to_string())))
    }
    
}

#[post("/auth/register", data = "<data>")]
pub async fn register(
    connection: MysqlConnection,
    data: Json<NewUser>,
) -> Result<Json<NewUser>, (Status, String)> {
    // Check if email is already used
    match connection
        .run({
            let data = data.clone();
            move |c| users::table.filter(email.eq(&data.email)).first::<User>(c)
        })
        .await
    {
        Ok(_user) => return Err((Status::Conflict, "email already used".to_string())),

        Err(_e) => {
            // Hash password and check if return error
            let hashed_password = match crypto::hash_password(&data.password.as_str()) {
                Ok(hashed_password) => hashed_password,
                Err(e) => return Err((Status::InternalServerError, e.to_string())),
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
                    move |c| insert_into(users::dsl::users).values(user).execute(c)
                })
                .await
            {
                Ok(_) => return Ok(Json(data.into_inner())),

                Err(e) => return Err((Status::InternalServerError, e.to_string())),
            }
        }
    }
}