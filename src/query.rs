use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
pub const API_URL: &str = "https://www.strava.com/api/v3";

use url::Url;

pub async fn get<T>(path: &str, token: &str) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
{
    let client = Client::new();

    let response = client
        .get(path)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| {
            panic!("Request failed to send: {}", err);
        })?;

    handle_response::<T>(response).await
}

pub async fn post<T, B>(path: &str, token: &str, body: B) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
    B: Serialize + Debug,
{
    let client = Client::new();

    let response = client
        .post(format!("{}/{}", API_URL, path))
        .header("Authorization", format!("Bearer {}", token))
        .json(&body) // Serialize the body as JSON
        .send()
        .await
        .map_err(|err| {
            panic!("Request failed to send: {}", err);
        })?;

    handle_response::<T>(response).await
}

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

pub struct Filter {
    pub query: Vec<(String, String)>,
    pub path_params: Vec<(String, String)>,
}

pub trait EndPoint {
    fn path(&self) -> String;
}

#[async_trait]
pub trait Sendable<T, U> {
    async fn send(self) -> Result<U, ErrorWrapper>;
}

pub trait Query: Sized + Clone {
    fn format_to_query_params(
        url: &str,
        params: Vec<(String, String)>,
    ) -> Result<String, Box<dyn Error>> {
        Ok(Url::parse_with_params(url, params.iter())?.to_string())
    }

    fn get_query_params(self) -> Vec<(String, String)>;
}

pub trait Endpoint: Sized + Clone {
    fn endpoint(&self) -> String;
}

pub trait PathQuery: Endpoint {
    fn get_path_params(&self) -> HashMap<String, String>;
}

pub trait Page {
    fn page(self, number: u32) -> Self;
}
pub trait PerPage {
    fn per_page(self, number: u32) -> Self;
}

pub trait PageSize {
    fn page_size(self, number: u32) -> Self;
}

pub trait ID {
    fn id(self, id: u64) -> Self;
}

pub trait AfterCursor {
    fn after_cursor(self, cursor: String) -> Self;
}

pub trait IncludeAllEfforts {
    fn include_all_efforts(self, should_include: bool) -> Self;
}

pub trait TimeFilter {
    fn before(self, timestamp: i64) -> Self;

    fn after(self, timestamp: i64) -> Self;
}

pub async fn get_with_query<T, U>(mut inst: T, token: &str) -> Result<U, ErrorWrapper>
where
    T: Endpoint + Query + PathQuery + Sendable<T, U>,
    U: DeserializeOwned + Debug,
{
    let url = T::format_to_query_params(&inst.endpoint(), inst.get_query_params())
        .expect("Failed to format query params");
    get(&url, token).await
}

fn format_path(template: &str, params: &HashMap<String, String>) -> String {
    let mut path = template.to_string();
    for (key, value) in params {
        let placeholder = format!("{{{}}}", key);
        path = path.replace(&placeholder, value);
    }
    path
}

pub async fn get_with_query_and_path<T, U>(mut inst: T, token: &str) -> Result<U, ErrorWrapper>
where
    T: Query + PathQuery + Endpoint,
    U: DeserializeOwned + Debug,
{
    let url_with_path_params = &format_path(&inst.endpoint(), &inst.get_path_params());
    let url = T::format_to_query_params(&url_with_path_params, inst.get_query_params())
        .expect("Failed to format query params");
    get(&url, token).await
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
