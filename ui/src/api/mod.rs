use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};

use crate::components::user_provider::UserInfo;

use self::models::User;

pub mod models;
pub mod auth;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorBody {
    pub code: i32,
    pub reason: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

pub async fn me() -> Result<User, ErrorResponse> {
    let token = UserInfo::get_token();

    if token.is_none() {
        return Err(ErrorResponse {
            error: ErrorBody {
                code: 400,
                reason: "Bad Request".into(),
                description: "User not authentified".into(),
            },
        });
    }

    let token = token.unwrap();
    let client = reqwest::Client::new();

    match client
        .get(format!("{}/{}", dotenv!("API_ENDPOINT"), "users/@me"))
        .header("Accept", "application/json")
        .header("Authorization", format!("bearer {}", token))
        .send()
        .await
    {
        Ok(r) => match r.error_for_status_ref() {
            Ok(_r) => {
                let response = r.json::<User>().await.unwrap();

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