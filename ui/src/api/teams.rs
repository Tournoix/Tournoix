use reqwest::{Method, header::HeaderMap};

use super::{models::{Team, TeamUpdate}, api_call, ErrorResponse};

pub async fn update(team_id: i32, update_request: TeamUpdate) -> Result<Team, ErrorResponse> {
    api_call::<Team>(Method::PATCH, &format!("teams/{}", team_id), HeaderMap::new(), serde_json::to_string(&update_request).unwrap()).await
}