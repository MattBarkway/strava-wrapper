use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::{Client, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::sync::{OnceLock, RwLock};

pub const API_URL: &str = "https://www.strava.com/api";

/// Process-wide reqwest client. Reused across all requests so the connection
/// pool and TLS session cache don't get rebuilt for every call.
fn http_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
}

/// Counters from Strava's `X-RateLimit-Usage` and `X-RateLimit-Limit`
/// response headers. Strava tracks two windows per application:
///
/// - **short term** — 15-minute rolling window
/// - **long term**  — 24-hour rolling window
///
/// Strava's default quotas are 100 (short) / 1000 (long); elevated apps get
/// higher limits. Both counters are per-application, not per-token, so a
/// process-wide "last seen" snapshot is an accurate view of consumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimit {
    pub short_term_usage: u32,
    pub short_term_limit: u32,
    pub long_term_usage: u32,
    pub long_term_limit: u32,
}

impl RateLimit {
    /// Parse both headers from a response. Returns `None` if either header
    /// is missing or malformed (e.g. mock servers without the headers set).
    pub fn from_headers(headers: &HeaderMap) -> Option<Self> {
        let limit = headers.get("x-ratelimit-limit")?.to_str().ok()?;
        let usage = headers.get("x-ratelimit-usage")?.to_str().ok()?;
        let (short_term_limit, long_term_limit) = parse_pair(limit)?;
        let (short_term_usage, long_term_usage) = parse_pair(usage)?;
        Some(Self {
            short_term_usage,
            short_term_limit,
            long_term_usage,
            long_term_limit,
        })
    }

    /// Remaining calls before the 15-minute quota trips (saturating).
    pub fn short_term_remaining(&self) -> u32 {
        self.short_term_limit.saturating_sub(self.short_term_usage)
    }

    /// Remaining calls before the daily quota trips (saturating).
    pub fn long_term_remaining(&self) -> u32 {
        self.long_term_limit.saturating_sub(self.long_term_usage)
    }
}

fn parse_pair(s: &str) -> Option<(u32, u32)> {
    let mut parts = s.split(',');
    let a = parts.next()?.trim().parse().ok()?;
    let b = parts.next()?.trim().parse().ok()?;
    Some((a, b))
}

fn rate_limit_slot() -> &'static RwLock<Option<RateLimit>> {
    static SLOT: OnceLock<RwLock<Option<RateLimit>>> = OnceLock::new();
    SLOT.get_or_init(|| RwLock::new(None))
}

/// Most recent rate-limit snapshot observed from any request in this process.
/// `None` until the first response with the expected headers lands.
pub fn last_rate_limit() -> Option<RateLimit> {
    rate_limit_slot().read().ok().and_then(|g| *g)
}

fn record_rate_limit(headers: &HeaderMap) -> Option<RateLimit> {
    let rl = RateLimit::from_headers(headers)?;
    if let Ok(mut slot) = rate_limit_slot().write() {
        *slot = Some(rl);
    }
    Some(rl)
}

#[cfg(test)]
pub(crate) fn clear_rate_limit_for_testing() {
    if let Ok(mut slot) = rate_limit_slot().write() {
        *slot = None;
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorWrapper {
    Network(reqwest::Error),
    #[non_exhaustive]
    Parse {
        error: serde_json::Error,
        body: String,
    },
    #[non_exhaustive]
    Api {
        status: StatusCode,
        response: ErrorResponse,
        /// Rate-limit snapshot from the response that triggered this error.
        /// `None` for mock servers that don't set `X-RateLimit-*` headers.
        /// Especially useful on 429 responses to decide a back-off window.
        rate_limit: Option<RateLimit>,
    },
    Url(String),
}

impl fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorWrapper::Network(e) => write!(f, "network error: {}", e),
            ErrorWrapper::Parse { error, .. } => {
                write!(f, "failed to parse response: {}", error)
            }
            ErrorWrapper::Api {
                status, response, ..
            } => {
                write!(f, "Strava API error {}: {}", status, response.message)
            }
            ErrorWrapper::Url(msg) => write!(f, "URL error: {}", msg),
        }
    }
}

impl std::error::Error for ErrorWrapper {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorWrapper::Network(e) => Some(e),
            ErrorWrapper::Parse { error, .. } => Some(error),
            ErrorWrapper::Api { .. } | ErrorWrapper::Url(_) => None,
        }
    }
}

impl From<reqwest::Error> for ErrorWrapper {
    fn from(e: reqwest::Error) -> Self {
        ErrorWrapper::Network(e)
    }
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

pub async fn get<T>(path: &str, token: &str) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
{
    let response = http_client()
        .get(path)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    handle_response::<T>(response).await
}

pub async fn get_raw(path: &str, token: &str) -> Result<String, ErrorWrapper> {
    let response = http_client()
        .get(path)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let status = response.status();
    let rate_limit = record_rate_limit(response.headers());
    let body = response.text().await?;
    if status.is_success() {
        Ok(body)
    } else {
        Err(ErrorWrapper::Api {
            status,
            response: parse_error_body(&body),
            rate_limit,
        })
    }
}

pub async fn post<T, B>(path: &str, token: &str, body: B) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
    B: Serialize + Debug,
{
    let response = http_client()
        .post(path)
        .header("Authorization", format!("Bearer {}", token))
        .json(&body)
        .send()
        .await?;
    handle_response::<T>(response).await
}

async fn handle_response<T>(response: reqwest::Response) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
{
    let status = response.status();
    let rate_limit = record_rate_limit(response.headers());
    let body = response.text().await?;
    if status.is_success() {
        serde_json::from_str::<T>(&body).map_err(|error| ErrorWrapper::Parse { error, body })
    } else {
        Err(ErrorWrapper::Api {
            status,
            response: parse_error_body(&body),
            rate_limit,
        })
    }
}

pub(crate) fn parse_error_body(body: &str) -> ErrorResponse {
    serde_json::from_str::<ErrorResponse>(body).unwrap_or_else(|_| ErrorResponse {
        errors: Vec::new(),
        message: body.to_string(),
    })
}

#[async_trait]
pub trait Sendable<U> {
    async fn send(self) -> Result<U, ErrorWrapper>;
}

pub trait Query: Sized + Clone {
    fn format_to_query_params(
        url: &str,
        params: Vec<(String, String)>,
    ) -> Result<String, ErrorWrapper> {
        Url::parse_with_params(url, params.iter())
            .map(|u| u.to_string())
            .map_err(|e| ErrorWrapper::Url(e.to_string()))
    }

    fn get_query_params(self) -> Vec<(String, String)>;
}

pub trait Endpoint: Sized + Clone {
    fn new(url: impl Into<String>, token: impl Into<String>, path: impl Into<String>) -> Self
    where
        Self: Sized;

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

pub trait Before {
    fn before(self, before: u64) -> Self;
}

pub trait After {
    fn after(self, after: u64) -> Self;
}

pub trait ID {
    fn id(self, id: u64) -> Self;
}

/// String-valued `{id}` path param. Use on resources where Strava assigns
/// non-numeric identifiers (e.g. gear ids like `"b12345678"`).
pub trait GearID {
    fn id(self, id: impl Into<String>) -> Self;
}

pub trait AfterCursor {
    fn after_cursor(self, cursor: String) -> Self;
}

pub trait IncludeAllEfforts {
    fn include_all_efforts(self, should_include: bool) -> Self;
}

fn format_path(template: &str, params: &HashMap<String, String>) -> String {
    let mut path = template.to_string();
    for (key, value) in params {
        let placeholder = format!("{{{}}}", key);
        path = path.replace(&placeholder, value);
    }
    path
}

pub async fn get_with_query_and_path<T, U>(inst: T, token: &str) -> Result<U, ErrorWrapper>
where
    T: Query + PathQuery + Endpoint,
    U: DeserializeOwned + Debug,
{
    let url_with_path_params = format_path(&inst.endpoint(), &inst.get_path_params());
    let url = T::format_to_query_params(&url_with_path_params, inst.get_query_params())?;
    get(&url, token).await
}

pub async fn get_raw_with_query_and_path<T>(inst: T, token: &str) -> Result<String, ErrorWrapper>
where
    T: Query + PathQuery + Endpoint,
{
    let url_with_path_params = format_path(&inst.endpoint(), &inst.get_path_params());
    let url = T::format_to_query_params(&url_with_path_params, inst.get_query_params())?;
    get_raw(&url, token).await
}

pub async fn put_json<T, B>(path: &str, token: &str, body: &B) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
    B: Serialize + ?Sized,
{
    let response = http_client()
        .put(path)
        .header("Authorization", format!("Bearer {}", token))
        .json(body)
        .send()
        .await?;
    handle_response::<T>(response).await
}

pub async fn put_form<T, B>(path: &str, token: &str, body: &B) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
    B: Serialize + ?Sized,
{
    let response = http_client()
        .put(path)
        .header("Authorization", format!("Bearer {}", token))
        .form(body)
        .send()
        .await?;
    handle_response::<T>(response).await
}

pub async fn post_form<T, B>(path: &str, token: &str, body: &B) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
    B: Serialize + ?Sized,
{
    let response = http_client()
        .post(path)
        .header("Authorization", format!("Bearer {}", token))
        .form(body)
        .send()
        .await?;
    handle_response::<T>(response).await
}

pub async fn post_multipart<T>(
    path: &str,
    token: &str,
    form: reqwest::multipart::Form,
) -> Result<T, ErrorWrapper>
where
    T: DeserializeOwned + Debug,
{
    let response = http_client()
        .post(path)
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await?;
    handle_response::<T>(response).await
}

pub async fn put_json_with_path<T, U, B>(inst: T, token: &str, body: &B) -> Result<U, ErrorWrapper>
where
    T: Query + PathQuery + Endpoint,
    U: DeserializeOwned + Debug,
    B: Serialize + ?Sized,
{
    let url_with_path_params = format_path(&inst.endpoint(), &inst.get_path_params());
    let url = T::format_to_query_params(&url_with_path_params, inst.get_query_params())?;
    put_json(&url, token, body).await
}

pub async fn put_form_with_path<T, U, B>(inst: T, token: &str, body: &B) -> Result<U, ErrorWrapper>
where
    T: Query + PathQuery + Endpoint,
    U: DeserializeOwned + Debug,
    B: Serialize + ?Sized,
{
    let url_with_path_params = format_path(&inst.endpoint(), &inst.get_path_params());
    let url = T::format_to_query_params(&url_with_path_params, inst.get_query_params())?;
    put_form(&url, token, body).await
}

pub async fn post_form_with_path<T, U, B>(inst: T, token: &str, body: &B) -> Result<U, ErrorWrapper>
where
    T: Query + PathQuery + Endpoint,
    U: DeserializeOwned + Debug,
    B: Serialize + ?Sized,
{
    let url_with_path_params = format_path(&inst.endpoint(), &inst.get_path_params());
    let url = T::format_to_query_params(&url_with_path_params, inst.get_query_params())?;
    post_form(&url, token, body).await
}

pub async fn post_multipart_with_path<T, U>(
    inst: T,
    token: &str,
    form: reqwest::multipart::Form,
) -> Result<U, ErrorWrapper>
where
    T: Query + PathQuery + Endpoint,
    U: DeserializeOwned + Debug,
{
    let url_with_path_params = format_path(&inst.endpoint(), &inst.get_path_params());
    let url = T::format_to_query_params(&url_with_path_params, inst.get_query_params())?;
    post_multipart(&url, token, form).await
}
