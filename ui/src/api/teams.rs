use reqwest::{header::HeaderMap, Method};

use super::{
    api_call,
    models::{Team, TeamUpdate},
    ErrorResponse,
};

pub async fn update(team_id: i32, update_request: TeamUpdate) -> Result<Team, ErrorResponse> {
    api_call::<Team>(
        Method::PATCH,
        &format!("teams/{}", team_id),
        HeaderMap::new(),
        serde_json::to_string(&update_request).unwrap(),
    )
    .await
}
