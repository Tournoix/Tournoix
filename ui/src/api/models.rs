use chrono::TimeZone;
use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};

use super::{api_call, tournoix::UpdateTournoixRequest, EmptyResponse, ErrorResponse};

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
    pub date: chrono::NaiveDateTime,
    pub location: Option<String>,
    pub phase: i32,
    pub size_group: Option<i32>,
    pub code: String,
    pub is_qualif: bool,
    pub is_elim: bool,
}

impl Tournament {
    pub async fn update(
        &self,
        update_request: UpdateTournoixRequest,
    ) -> Result<Tournament, ErrorResponse> {
        super::tournoix::update(self.id, update_request).await
    }

    pub async fn delete(&self) -> Result<EmptyResponse, ErrorResponse> {
        api_call::<EmptyResponse>(
            Method::DELETE,
            &format!("tournoix/{}", self.id),
            HeaderMap::new(),
            String::new(),
        )
        .await
    }

    pub async fn get_user_nut(&self) -> Result<Nut, ErrorResponse> {
        api_call::<Nut>(
            Method::GET,
            &format!("tournoix/{}/nut", self.id),
            HeaderMap::new(),
            String::new(),
        )
        .await
    }

    pub async fn get_teams(&self) -> Result<Vec<Team>, ErrorResponse> {
        api_call::<Vec<Team>>(
            Method::GET,
            &format!("tournoix/{}/teams", self.id),
            HeaderMap::new(),
            String::new(),
        )
        .await
    }

    pub async fn add_teams(&self, team: AddTeamRequest) -> Result<Team, ErrorResponse> {
        api_call::<Team>(
            Method::POST,
            &format!("tournoix/{}/teams", self.id),
            HeaderMap::new(),
            serde_json::to_string(&team).unwrap(),
        )
        .await
    }

    pub async fn get_matches(&self) -> Result<Vec<GameWithTeams>, ErrorResponse> {
        api_call::<Vec<GameWithTeams>>(
            Method::GET,
            &format!("tournoix/{}/games", self.id),
            HeaderMap::new(),
            String::new(),
        )
        .await
    }

    pub async fn generate_qualif_games(&self) -> Result<Vec<Game>, ErrorResponse> {
        api_call::<Vec<Game>>(
            Method::POST,
            &format!("tournoix/{}/qualif", self.id),
            HeaderMap::new(),
            String::new(),
        )
        .await
    }

    pub async fn reset_qualif_games(&self) -> Result<EmptyResponse, ErrorResponse> {
        api_call::<EmptyResponse>(
            Method::DELETE,
            &format!("tournoix/{}/qualif", self.id),
            HeaderMap::new(),
            String::new(),
        )
        .await
    }

    pub async fn generate_elim_games(&self) -> Result<Vec<Game>, ErrorResponse> {
        api_call::<Vec<Game>>(
            Method::POST,
            &format!("tournoix/{}/elim", self.id),
            HeaderMap::new(),
            String::new(),
        )
        .await
    }

    pub async fn reset_elim_games(&self) -> Result<EmptyResponse, ErrorResponse> {
        api_call::<EmptyResponse>(
            Method::DELETE,
            &format!("tournoix/{}/elim", self.id),
            HeaderMap::new(),
            String::new(),
        )
        .await
    }
}

// ---- Team ----

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Team {
    pub id: i32,
    pub fk_tournaments: i32,
    pub name: String,
    pub group: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TeamUpdate {
    pub name: Option<String>,
    pub group: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddTeamRequest {
    pub name: String,
    pub group: i32,
}

impl Team {
    pub async fn update(&self, update_request: TeamUpdate) -> Result<Team, ErrorResponse> {
        super::teams::update(self.id, update_request).await
    }

    pub async fn delete(&self) -> Result<EmptyResponse, ErrorResponse> {
        api_call::<EmptyResponse>(
            Method::DELETE,
            &format!("teams/{}", self.id),
            HeaderMap::new(),
            String::new(),
        )
        .await
    }
}

// ---- Game ----

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: i32,
    pub fk_team1: i32,
    pub fk_team2: i32,
    pub score1: i32,
    pub score2: i32,
    pub phase: i32,
    pub place: i32,
    pub status: i32,
    pub has_gained_nut: bool,
    pub group: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameUpdate {
    pub score1: Option<i32>,
    pub score2: Option<i32>,
    pub phase: Option<i32>,
    pub status: Option<i32>,
}

impl Game {
    pub async fn update(&self, update_request: GameUpdate) -> Result<Game, ErrorResponse> {
        super::games::update(self.id, update_request).await
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct GameWithTeams {
    pub id: i32,
    pub team1: Team,
    pub team2: Team,
    pub score1: i32,
    pub score2: i32,
    pub phase: i32,
    pub place: i32,
    pub status: i32,
    pub has_gained_nut: bool,
    pub group: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nut {
    pub id: i32,
    pub fk_users: i32,
    pub fk_tournaments: i32,
    pub stock: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bet {
    pub id: i32,
    pub fk_nuts: i32,
    pub fk_games: i32,
    pub fk_teams: i32,
    pub nb_nut: i32,
}