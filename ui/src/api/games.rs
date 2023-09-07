use reqwest::{Method, header::HeaderMap};

use super::{models::{Game, GameUpdate}, api_call, ErrorResponse, EmptyResponse};

pub async fn update(
    game_id: i32,
    update_request: GameUpdate,
) -> Result<Game, ErrorResponse> {
    api_call::<Game>(
        Method::PATCH,
        &format!("games/{}", game_id),
        HeaderMap::new(),
        serde_json::to_string(&update_request).unwrap(),
    )
    .await
}

pub async fn close(
    game_id: i32,
) -> Result<EmptyResponse, ErrorResponse> {
    api_call::<EmptyResponse>(
        Method::POST,
        &format!("games/{}/close", game_id),
        HeaderMap::new(),
        String::new(),
    )
    .await
}