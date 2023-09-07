use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};

use super::{api_call, ErrorResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub token: String,
    pub expiration_date: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login to the API
/// On success return the session token
pub async fn login(login_request: LoginRequest) -> Result<TokenResponse, ErrorResponse> {
    api_call::<TokenResponse>(
        Method::POST,
        "auth/login",
        HeaderMap::new(),
        serde_json::to_string(&login_request).unwrap(),
    )
    .await
}

/// Logout to the API
/// Will revoke the session token
pub async fn logout() -> bool {
    match api_call::<String>(Method::POST, "auth/logout", HeaderMap::new(), String::new()).await {
        Ok(_) => true,
        Err(_e) => false,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub id: usize,
    pub name: String,
    pub email: String,
}

pub async fn register(
    register_request: RegisterRequest,
) -> Result<RegisterResponse, ErrorResponse> {
    api_call::<RegisterResponse>(
        Method::POST,
        "auth/register",
        HeaderMap::new(),
        serde_json::to_string(&register_request).unwrap(),
    )
    .await
}
