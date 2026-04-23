use crate::query::{parse_error_body, ErrorWrapper};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub const AUTH_URL: &str = "https://www.strava.com/oauth/token";
pub const DEAUTH_URL: &str = "https://www.strava.com/oauth/deauthorize";

fn auth_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
}

/// Exchange an authorization code for an access token, hitting Strava's
/// production OAuth endpoint.
pub async fn get_token(
    client_id: u32,
    client_secret: &str,
    code: &str,
) -> Result<TokenResponse, ErrorWrapper> {
    get_token_at(AUTH_URL, client_id, client_secret, code).await
}

/// Same as [`get_token`] but against a caller-supplied URL. Useful for
/// integration tests (pointing at httpmock) or for staging/dev OAuth hosts.
pub async fn get_token_at(
    url: &str,
    client_id: u32,
    client_secret: &str,
    code: &str,
) -> Result<TokenResponse, ErrorWrapper> {
    let response = auth_client()
        .post(url)
        .form(&TokenRequest {
            client_id,
            client_secret: client_secret.to_string(),
            code: code.to_string(),
            grant_type: "authorization_code".into(),
        })
        .send()
        .await?;

    handle_token_response(response).await
}

/// Swap an expiring access token for a fresh one using a stored refresh token.
pub async fn refresh_token(
    client_id: u32,
    client_secret: &str,
    refresh_token: &str,
) -> Result<TokenResponse, ErrorWrapper> {
    refresh_token_at(AUTH_URL, client_id, client_secret, refresh_token).await
}

/// Same as [`refresh_token`] but against a caller-supplied URL.
pub async fn refresh_token_at(
    url: &str,
    client_id: u32,
    client_secret: &str,
    refresh_token: &str,
) -> Result<TokenResponse, ErrorWrapper> {
    let response = auth_client()
        .post(url)
        .form(&RefreshRequest {
            client_id,
            client_secret: client_secret.to_string(),
            refresh_token: refresh_token.to_string(),
            grant_type: "refresh_token".into(),
        })
        .send()
        .await?;

    handle_token_response(response).await
}

/// Revoke an access token.
pub async fn deauthorize(access_token: &str) -> Result<(), ErrorWrapper> {
    deauthorize_at(DEAUTH_URL, access_token).await
}

/// Same as [`deauthorize`] but against a caller-supplied URL.
pub async fn deauthorize_at(url: &str, access_token: &str) -> Result<(), ErrorWrapper> {
    let response = auth_client()
        .post(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let status = response.status();
    if status.is_success() {
        Ok(())
    } else {
        let body = response.text().await?;
        Err(ErrorWrapper::Api {
            status,
            response: parse_error_body(&body),
            rate_limit: None,
        })
    }
}

async fn handle_token_response(response: reqwest::Response) -> Result<TokenResponse, ErrorWrapper> {
    let status = response.status();
    let body = response.text().await?;
    if status.is_success() {
        serde_json::from_str::<TokenResponse>(&body)
            .map_err(|error| ErrorWrapper::Parse { error, body })
    } else {
        Err(ErrorWrapper::Api {
            status,
            response: parse_error_body(&body),
            rate_limit: None,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TokenRequest {
    client_id: u32,
    client_secret: String,
    code: String,
    grant_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RefreshRequest {
    client_id: u32,
    client_secret: String,
    refresh_token: String,
    grant_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub expires_at: u64,
    pub expires_in: u64,
    pub refresh_token: String,
}
