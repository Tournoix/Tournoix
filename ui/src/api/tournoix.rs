use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};

use super::{api_call, models::{Tournament, Subscription}, ErrorResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTournoixRequest {
    pub name: String,
    pub description: String,
    pub date: chrono::NaiveDateTime,
    pub location: String,
    pub is_qualif: bool,
    pub is_elim: bool,
}

pub async fn create(
    create_tounoix_request: CreateTournoixRequest,
) -> Result<Tournament, ErrorResponse> {
    api_call::<Tournament>(
        Method::POST,
        "tournoix",
        HeaderMap::new(),
        serde_json::to_string(&create_tounoix_request).unwrap(),
    )
    .await
}

pub async fn get(tournoix_id: i32) -> Result<Tournament, ErrorResponse> {
    api_call::<Tournament>(
        Method::GET,
        &format!("tournoix/{}", tournoix_id),
        HeaderMap::new(),
        String::new(),
    )
    .await
}

pub async fn get_by_code(code: impl Into<String>) -> Result<Tournament, ErrorResponse> {
    api_call::<Tournament>(
        Method::GET,
        &format!("tournoix_by_code/{}", code.into()),
        HeaderMap::new(),
        String::new(),
    )
    .await
}

pub async fn subscribe(request: SubscriptionRequest) -> Result<Subscription, ErrorResponse> {
    api_call::<Subscription>(
        Method::POST,
        "/users/@me/subscription",
        HeaderMap::new(),
        serde_json::to_string(&request).unwrap(),
    )
    .await
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubscriptionRequest {
    pub code: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTournoixRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub phase: Option<i32>,
    pub size_group: Option<i32>,
    pub is_qualif: Option<bool>,
    pub is_elim: Option<bool>,
}

pub async fn update(
    tournoix_id: i32,
    update_request: UpdateTournoixRequest,
) -> Result<Tournament, ErrorResponse> {
    api_call::<Tournament>(
        Method::PATCH,
        &format!("tournoix/{}", tournoix_id),
        HeaderMap::new(),
        serde_json::to_string(&update_request).unwrap(),
    )
    .await
}
