use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const AUTH_URL: &str = "https://www.strava.com/oauth/token";
pub async fn get_token(
    client_id: u32,
    client_secret: &str,
    code: &str,
) -> Result<String, StatusCode> {
    let client = reqwest::Client::new();
    let response = client
        .post(AUTH_URL)
        .form(&TokenRequest {
            client_id,
            client_secret: client_secret.to_string(),
            code: code.to_string(),
            grant_type: "authorization_code".to_string(),
        })
        .send()
        .await
        .map_err(|err| panic!("Request failed to send: {}", err))
        .unwrap();
    Ok(response.text().await.unwrap())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TokenRequest {
    client_id: u32,
    client_secret: String,
    code: String,
    grant_type: String,
}
