use reqwest::{Method, header::HeaderMap};

use super::{api_call, ErrorResponse, EmptyResponse, models::{Bet, BetWithUser}};

pub async fn get_bets(
    game_id: i32,
) -> Result<Vec<BetWithUser>, ErrorResponse> {
    api_call::<Vec<BetWithUser>>(
        Method::GET,
        &format!("game/{}/bet", game_id),
        HeaderMap::new(),
        String::new(),
    )
    .await
}
