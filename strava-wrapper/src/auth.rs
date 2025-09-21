use serde::{Deserialize, Serialize};

const AUTH_URL: &str = "https://www.strava.com/oauth/token";
pub async fn get_token(
    client_id: u32,
    client_secret: &str,
    code: &str,
) -> Result<TokenResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(AUTH_URL)
        .form(&TokenRequest {
            client_id,
            client_secret: client_secret.to_string(),
            code: code.to_string(),
            grant_type: "authorization_code".into(),
        })
        .send()
        .await?;

    if !response.status().is_success() {
        // you could map this into your own Error type
        panic!("Request failed: {}", response.status());
    }

    response.json::<TokenResponse>().await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TokenRequest {
    client_id: u32,
    client_secret: String,
    code: String,
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
