use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
pub const TOKEN: &str = "";
pub(crate) const API_URL: &str = "https://www.strava.com/api/v3/";

use reqwest::{Client, Error};
use std::fmt::Debug;

pub async fn get<T>(path: &str, token: &str) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
{
    let client = Client::new();

    let response = client
        .get(format!("{}/{}", API_URL, path))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| {
            panic!("Request failed to send: {}", err);
        })?;

    handle_response::<T>(response).await
}

pub async fn post<T, B>(path: &str, token: &str, body: &B) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
    B: Serialize + Debug, // Ensure the body can be serialized to JSON
{
    let client = Client::new();

    let response = client
        .post(format!("{}/{}", API_URL, path))
        .header("Authorization", format!("Bearer {}", token))
        .json(body) // Serialize the body as JSON
        .send()
        .await
        .map_err(|err| {
            panic!("Request failed to send: {}", err);
        })?;

    handle_response::<T>(response).await
}

// This helper function handles the common response logic
async fn handle_response<T>(response: reqwest::Response) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
{
    if response.status().is_success() {
        let raw_body = response.text().await.map_err(|err| {
            panic!("Failed to read response text: {}", err);
        })?;

        let content = serde_json::from_str::<T>(&raw_body).map_err(|err| {
            eprintln!("Failed to parse JSON: {}, Raw response: {}", err, raw_body);
            panic!("Failed to parse JSON: {}", err);
        })?;

        Ok(content)
    } else {
        let status = response.status();
        let error_content = response
            .json::<ErrorResponse>()
            .await
            .unwrap_or_else(|_| panic!("Failed to parse error response from server"));
        Err(ErrorWrapper {
            status,
            error: error_content,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ErrorWrapper {
    status: StatusCode,
    error: ErrorResponse,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<ErrorDetails>,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub resource: String,
    pub field: String,
    pub code: String,
}
