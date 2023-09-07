use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};

use super::{api_call, models::{GameWithTeams, Bet}, ErrorResponse};

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
    pub nb_nut: i32,
    pub team_id: i32
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

pub async fn get_user_bet_on_match(id_user: i32, id: i32) -> Result<Bet, ErrorResponse> {
    api_call::<Bet>(
        Method::GET,
        &format!("user/{}/game/{}/bet", id_user, id),
        HeaderMap::new(),
        String::new(),
    )
    .await
}
