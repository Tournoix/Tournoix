use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};

use crate::components::user_provider::UserInfo;

use super::{ErrorBody, ErrorResponse};

// ---- User ----

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub email: String,
}

impl User {
    pub async fn tournaments(&self) -> Result<Vec<Tournament>, ErrorResponse> {
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
            .get(format!(
                "{}/{}",
                dotenv!("API_ENDPOINT"),
                "users/@me/tournoix"
            ))
            .header("Accept", "application/json")
            .header("Authorization", format!("bearer {}", token))
            .send()
            .await
        {
            Ok(r) => match r.error_for_status_ref() {
                Ok(_r) => Ok(r.json::<Vec<Tournament>>().await.unwrap()),

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

    pub async fn subscriptions(&self) -> Result<Vec<Tournament>, ErrorResponse> {
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
            .get(format!(
                "{}/{}",
                dotenv!("API_ENDPOINT"),
                "users/@me/subscriptions"
            ))
            .header("Accept", "application/json")
            .header("Authorization", format!("bearer {}", token))
            .send()
            .await
        {
            Ok(r) => match r.error_for_status_ref() {
                Ok(_r) => Ok(r.json::<Vec<Tournament>>().await.unwrap()),

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

    /*
    pub fn nuts(&self, tournament: &Tournament) -> i32 {

    }
     */
}

// ---- Tournament ----

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Tournament {
    pub id: usize,
    pub fk_users: i32,
    pub name: String,
    pub description: String,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub phase: i32,
    pub size_group: Option<i32>,
    pub code: String,
}

// ---- Team ----

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: usize,
    pub fk_tournaments: i32,
    pub name: String,
    pub group: i32,
}

// ---- Game ----

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: usize,
    pub fk_team1: i32,
    pub fk_team2: i32,
    pub score1: i32,
    pub score2: i32,
    pub phase: i32,
    pub place: i32,
}
