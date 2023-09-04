use chrono::TimeZone;
use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};

use super::{api_call, ErrorResponse, EmptyResponse, tournoix::UpdateTournoixRequest};

// ---- User ----

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub email: String,
}

impl User {
    pub async fn tournaments(&self) -> Result<Vec<Tournament>, ErrorResponse> {
        api_call::<Vec<Tournament>>(
            Method::GET,
            "users/@me/tournoix",
            HeaderMap::new(),
            String::new(),
        )
        .await
    }

    pub async fn subscriptions(&self) -> Result<Vec<Tournament>, ErrorResponse> {
        api_call::<Vec<Tournament>>(
            Method::GET,
            "users/@me/subscriptions",
            HeaderMap::new(),
            String::new(),
        )
        .await
    }

    /*
    pub fn nuts(&self, tournament: &Tournament) -> i32 {

    }
     */
}

// ---- Tournament ----

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Tournament {
    pub id: i32,
    pub fk_users: i32,
    pub name: String,
    pub description: String,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub phase: i32,
    pub size_group: Option<i32>,
    pub code: String,
}

impl Tournament {
    pub fn is_elim(&self) -> bool {true}
    pub fn is_qualif(&self) -> bool {true}

    pub fn date_locale(&self) -> chrono::NaiveDateTime {
        chrono::Local.from_utc_datetime(&self.date.unwrap()).naive_local()
    }

    pub async fn update(&self, update_request: UpdateTournoixRequest) -> Result<Tournament, ErrorResponse> {
        super::tournoix::update(self.id, update_request).await
    }

    pub async fn delete(&self) -> Result<EmptyResponse, ErrorResponse> {
        api_call::<EmptyResponse>(Method::DELETE, &format!("tournoix/{}", self.id), HeaderMap::new(), String::new()).await
    }

    pub async fn get_user_nut(&self) -> Result<Nut, ErrorResponse> {
        api_call::<Nut>(Method::GET, &format!("tournoix/{}/nut", self.id), HeaderMap::new(), String::new()).await
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Nut {
    pub id: i32,
    pub fk_users: i32,
    pub fk_tournaments: i32,
    pub stock: i32,
}
