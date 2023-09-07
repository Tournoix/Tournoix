use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};

use super::{api_call, models::{GameWithTeams, Bet, Nut}, ErrorResponse};

pub async fn get(game_id: i32) -> Result<GameWithTeams, ErrorResponse> {
    api_call::<GameWithTeams>(
        Method::GET,
        &format!("game/{}", game_id),
        HeaderMap::new(),
        String::new(),
    )
    .await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BetData {
    pub nut: i32,
    pub team_id: i32
}

pub async fn get_nb_nut(tournament_id: i32) -> Result<Nut, ErrorResponse> {
    api_call::<Nut>(
        Method::GET,
        &format!("tournoix/{}/nut", tournament_id),
        HeaderMap::new(),
        String::new(),
    )
    .await
}

pub async fn bet(game_id: i32, bet_request: BetData) -> Result<Bet, ErrorResponse> {
    api_call::<Bet>(
        Method::POST,
        &format!("game/{}/bet", game_id),
        HeaderMap::new(),
        serde_json::to_string(&bet_request).unwrap(),
    )
    .await
}

pub async fn delete_bet(game_id: i32) -> Result<Bet, ErrorResponse> {
    api_call::<Bet>(
        Method::DELETE,
        &format!("game/{}/bet", game_id),
        HeaderMap::new(),
        String::new(),
    )
    .await
}

// !! CAREFUL, the fk_ fields are mixed up !!
pub async fn get_user_bet_on_match(id_user: i32, id: i32) -> Result<Bet, ErrorResponse> {
    api_call::<Bet>(
        Method::GET,
        &format!("user/{}/game/{}/bet", id_user, id),
        HeaderMap::new(),
        String::new(),
    )
    .await
}
