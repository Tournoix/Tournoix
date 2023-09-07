use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};

use super::{api_call, models::GameWithTeams, ErrorResponse};

pub async fn get(game_id: i32) -> Result<GameWithTeams, ErrorResponse> {
    api_call::<GameWithTeams>(
        Method::GET,
        &format!("game/{}", game_id),
        HeaderMap::new(),
        String::new(),
    )
    .await
}