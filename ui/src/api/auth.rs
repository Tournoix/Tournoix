use serde::{Serialize, Deserialize};
use dotenv_codegen::dotenv;

use crate::components::user_provider::UserInfo;

use super::{ErrorResponse, ErrorBody};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub token: String,
    pub expiration_date: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login to the API
/// On success return the session token
pub async fn login(login_request: LoginRequest) -> Result<TokenResponse, ErrorResponse> {
    let client = reqwest::Client::new();

    match client
        .post(format!("{}/{}", dotenv!("API_ENDPOINT"), "auth/login"))
        .header("Accept", "application/json")
        .body(serde_json::to_string(&login_request).unwrap())
        .send()
        .await
    {
        Ok(r) => match r.error_for_status_ref() {
            Ok(_r) => {
                let response = r.json::<TokenResponse>().await.unwrap();

                Ok(response)
            }

            Err(_e) => Err(r.json::<ErrorResponse>().await.unwrap()),
        },

        Err(_e) => Err(ErrorResponse {
            error: ErrorBody {
                code: 500,
                reason: "Internal server error".into(),
                description: "An error occured".into(),
            },
        }),
    }
}

/// Logout to the API
/// Will revoke the session token
pub async fn logout() -> bool {
    let token = UserInfo::get_token();

    if token.is_none() {
        return false;
    }

    let token = token.unwrap();

    let client = reqwest::Client::new();

    match client
        .post(format!("{}/{}", dotenv!("API_ENDPOINT"), "auth/logout"))
        .header("Accept", "application/json")
        .header("Authorization", format!("bearer {}", token))
        .send()
        .await
    {
        Ok(r) => match r.error_for_status_ref() {
            Ok(_r) => true,

            Err(_e) => false,
        },

        Err(_e) => false,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub id: usize,
    pub name: String,
    pub email: String,
}

pub async fn register(
    register_request: RegisterRequest,
) -> Result<RegisterResponse, ErrorResponse> {
    let client = reqwest::Client::new();

    match client
        .post(format!("{}/{}", dotenv!("API_ENDPOINT"), "auth/register"))
        .header("Accept", "application/json")
        .body(serde_json::to_string(&register_request).unwrap())
        .send()
        .await
    {
        Ok(r) => match r.error_for_status_ref() {
            Ok(_r) => {
                let response = r.json::<RegisterResponse>().await.unwrap();

                Ok(response)
            }

            Err(_e) => Err(r.json::<ErrorResponse>().await.unwrap()),
        },

        Err(_e) => Err(ErrorResponse {
            error: ErrorBody {
                code: 500,
                reason: "Internal server error".into(),
                description: "An error occured".into(),
            },
        }),
    }
}