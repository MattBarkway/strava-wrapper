use std::error::Error;
use std::fmt::Debug;
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
pub const TOKEN: &str = "";
pub const API_URL: &str = "https://www.strava.com/api/v3/";

use url::Url;

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

pub async fn post<T, B>(path: &str, token: &str, body: B) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
    B: Serialize + Debug, // Ensure the body can be serialized to JSON
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

pub struct Filter {
    pub query: Vec<(String, String)>,
    pub path_params: Vec<(String, String)>,
}

pub trait EndPoint {
    fn path(&self) -> String;
}

#[async_trait]
pub trait Sendable<T, U>: EndPoint {
    async fn send(self, token: &str) -> Result<U, ErrorWrapper>;
}

pub trait Query: Sized + Clone {
    fn format_to_query_params(url: &str, params: Vec<(String, String)>) -> Result<String, Box<dyn Error>> {
        Ok(Url::parse_with_params(url, params.iter())?.to_string())
    }

    fn query(self) -> Vec<(String, String)>;
}

pub trait PathQuery: Sized {
    fn format_from_tuples(template: &str, values: Vec<(String, String)>) -> String {
        let mut result = template.to_string();
        for (key, value) in values {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value);
        }
        result
    }

    fn path_params(&self) -> Vec<(String, String)>;
}

pub trait Page: Query {
    fn page(self, number: u32) -> Self {
        self.clone().query().push(("page".to_string(), number.to_string()));
        self
    }
}
pub trait PerPage: Query {
    fn per_page(self, number: u32) -> Self {
        self.clone().query()
            .push(("per_page".to_string(), number.to_string()));
        self
    }
}

pub trait PageSize: Query {
    fn page_size(mut self, number: u32) -> Self {
        self.clone().query()
            .push(("page_size".to_string(), number.to_string()));
        self
    }
}

pub trait ID: PathQuery {
    fn id(mut self, id: u64) -> Self {
        self.path_params().push(("after_cursor".to_string(), id.to_string()));
        self
    }
}

pub trait AfterCursor: Query {
    fn after_cursor(mut self, cursor: String) -> Self {
        self.clone().query().push(("after_cursor".to_string(), cursor));
        self
    }
}

pub trait IncludeAllEfforts: Query {
    fn include_all_efforts(mut self, should_include: bool) -> Self {
        self.clone().query().push(("include_all_efforts".to_string(), should_include.to_string()));
        self
    }
}

pub trait TimeFilter: Query {
    fn before(mut self, timestamp: i64) -> Self {
        self.clone().query()
            .push(("before".to_string(), timestamp.to_string()));
        self
    }

    fn after(mut self, timestamp: i64) -> Self {
        self.clone().query()
            .push(("after".to_string(), timestamp.to_string()));
        self
    }
}

pub async fn get_with_query<T, U>(inst: T, token: &str) -> Result<U, ErrorWrapper>
where
    T: Query + Sendable<T, U>,
    U: DeserializeOwned + Debug,
{
    let url = T::format_to_query_params(&format!("{}/{}", API_URL, inst.path()), inst.query())
        .expect("Failed to format query params");
    get(&url, token).await
}

pub async fn get_with_query_and_path<T, U>(inst: T, token: &str) -> Result<U, ErrorWrapper>
where
T: Query + PathQuery + EndPoint,
U: DeserializeOwned + Debug,
{
    let formatted_path = T::format_from_tuples(&inst.path(), inst.path_params());
    let url = T::format_to_query_params(&format!("{}/{}", API_URL, formatted_path), inst.query())
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
