use dotenv_codegen::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::components::user_provider::UserInfo;

use self::models::User;

pub mod auth;
pub mod models;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorBody {
    pub code: i32,
    pub reason: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyResponse {}

/// Make an API call to the backend  
///
/// **T**: The return type of the API in case of success <br>
/// **method**: Method of the HTTP request <br>
/// **route**: The route of the API (only the route not the full url -> "auth/login") <br>
/// **headers**: Headers to be added to the request. **Note**: The Authorization token is automatically added if present in local storage <br>
/// **body**: JSON body of the request  
pub async fn api_call<T: DeserializeOwned>(
    method: Method,
    route: &str,
    headers: HeaderMap<HeaderValue>,
    body: String,
) -> Result<T, ErrorResponse> {
    let client = reqwest::Client::new();

    let mut request = client
        .request(method, format!("{}/{}", dotenv!("API_ENDPOINT"), route))
        .header("Accept", "application/json");

    // Add token to request if exists
    if let Some(token) = UserInfo::get_token() {
        request = request.header("Authorization", format!("bearer {}", token));
    }

    match request.headers(headers).body(body).send().await {
        Ok(r) => match r.error_for_status_ref() {
            Ok(_r) => Ok(r.json::<T>().await.unwrap()),
            Err(_e) => Err(r.json::<ErrorResponse>().await.unwrap()),
        },

        Err(_e) => Err(ErrorResponse {
            error: ErrorBody {
                code: 500,
                reason: "Internal server error".into(),
                description: "An error occured".into(),
            },
        }),
    }
}

/// Get current logged User Info
pub async fn me() -> Result<User, ErrorResponse> {
    api_call::<User>(Method::GET, "users/@me", HeaderMap::new(), "".to_string()).await
}
