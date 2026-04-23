use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::auth::{deauthorize_at, get_token_at, refresh_token_at};
use strava_wrapper::query::ErrorWrapper;

#[tokio::test]
async fn test_get_token_success() {
    let server = MockServer::start();

    let response = json!({
        "token_type": "Bearer",
        "access_token": "access-abc",
        "expires_at": 1_800_000_000,
        "expires_in": 21_600,
        "refresh_token": "refresh-xyz"
    });

    let mock = server.mock(|when, then| {
        when.method("POST").path("/oauth/token");
        then.status(200).json_body(response);
    });

    let url = format!("{}/oauth/token", server.base_url());
    let token = get_token_at(&url, 1234, "secret", "auth-code")
        .await
        .unwrap();

    assert_eq!(token.access_token, "access-abc");
    assert_eq!(token.refresh_token, "refresh-xyz");
    assert_eq!(token.expires_in, 21_600);
    mock.assert();
}

#[tokio::test]
async fn test_get_token_rejects_bad_code() {
    let server = MockServer::start();

    let err_body = json!({
        "message": "Bad Request",
        "errors": [{ "resource": "Application", "field": "code", "code": "invalid" }]
    });

    let _mock = server.mock(|when, then| {
        when.method("POST").path("/oauth/token");
        then.status(400).json_body(err_body);
    });

    let url = format!("{}/oauth/token", server.base_url());
    let err = get_token_at(&url, 1234, "secret", "bad-code")
        .await
        .unwrap_err();

    match err {
        ErrorWrapper::Api {
            status, response, ..
        } => {
            assert_eq!(status.as_u16(), 400);
            assert_eq!(response.errors[0].code, "invalid");
        }
        other => panic!("expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn test_refresh_token_success() {
    let server = MockServer::start();

    let response = json!({
        "token_type": "Bearer",
        "access_token": "access-new",
        "expires_at": 1_800_000_000,
        "expires_in": 21_600,
        "refresh_token": "refresh-new"
    });

    let mock = server.mock(|when, then| {
        when.method("POST").path("/oauth/token");
        then.status(200).json_body(response);
    });

    let url = format!("{}/oauth/token", server.base_url());
    let token = refresh_token_at(&url, 1234, "secret", "old-refresh")
        .await
        .unwrap();

    assert_eq!(token.access_token, "access-new");
    mock.assert();
}

#[tokio::test]
async fn test_deauthorize_success() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method("POST")
            .path("/oauth/deauthorize")
            .header("authorization", "Bearer access-abc");
        then.status(200).body("{}");
    });

    let url = format!("{}/oauth/deauthorize", server.base_url());
    deauthorize_at(&url, "access-abc").await.unwrap();

    mock.assert();
}

#[tokio::test]
async fn test_deauthorize_rejects_bad_token() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method("POST").path("/oauth/deauthorize");
        then.status(401)
            .json_body(json!({ "message": "Unauthorized" }));
    });

    let url = format!("{}/oauth/deauthorize", server.base_url());
    let err = deauthorize_at(&url, "bad-token").await.unwrap_err();

    match err {
        ErrorWrapper::Api { status, .. } => assert_eq!(status.as_u16(), 401),
        other => panic!("expected Api error, got {other:?}"),
    }
}
