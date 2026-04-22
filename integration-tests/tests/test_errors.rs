use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{ErrorWrapper, Sendable, ID};

/// 401 Unauthorized — should surface ErrorWrapper::Api with the parsed error body.
#[tokio::test]
async fn test_api_error_unauthorized() {
    let server = MockServer::start();

    let err_body = json!({
        "message": "Authorization Error",
        "errors": [
            { "resource": "Athlete", "field": "access_token", "code": "invalid" }
        ]
    });

    let _mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athlete");
        then.status(401).json_body(err_body);
    });

    let api = StravaAPI::new(&server.base_url(), "bad-token");
    let err = api.athlete().get().send().await.unwrap_err();

    match err {
        ErrorWrapper::Api {
            status, response, ..
        } => {
            assert_eq!(status.as_u16(), 401);
            assert_eq!(response.message, "Authorization Error");
            assert_eq!(response.errors.len(), 1);
            assert_eq!(response.errors[0].field, "access_token");
        }
        other => panic!("expected ErrorWrapper::Api, got {other:?}"),
    }
}

/// When the error body isn't valid JSON, Api variant still returns but
/// the raw body lands in `message`.
#[tokio::test]
async fn test_api_error_non_json_body() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athlete");
        then.status(503).body("Service Unavailable");
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let err = api.athlete().get().send().await.unwrap_err();

    match err {
        ErrorWrapper::Api {
            status, response, ..
        } => {
            assert_eq!(status.as_u16(), 503);
            assert_eq!(response.message, "Service Unavailable");
            assert!(response.errors.is_empty());
        }
        other => panic!("expected ErrorWrapper::Api, got {other:?}"),
    }
}

/// 200 OK with body that can't deserialize to the target type — Parse variant
/// with the raw body kept for debugging.
#[tokio::test]
async fn test_parse_error_preserves_body() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method("GET").path("/v3/activities/123");
        then.status(200).body("not valid json");
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let err = api.activities().get().id(123).send().await.unwrap_err();

    match err {
        ErrorWrapper::Parse { body, .. } => {
            assert_eq!(body, "not valid json");
        }
        other => panic!("expected ErrorWrapper::Parse, got {other:?}"),
    }
}

/// Transport-level failure (connection refused) — Network variant.
#[tokio::test]
async fn test_network_error_unreachable() {
    // Port 1 on localhost is almost certainly not listening.
    let api = StravaAPI::new("http://127.0.0.1:1", "foo");
    let err = api.athlete().get().send().await.unwrap_err();

    match err {
        ErrorWrapper::Network(_) => {}
        other => panic!("expected ErrorWrapper::Network, got {other:?}"),
    }
}

/// Relative URL without a base — the Url::parse_with_params step fails.
#[tokio::test]
async fn test_url_error_malformed_base() {
    let api = StravaAPI::new("not-a-url", "foo");
    let err = api.athlete().get().send().await.unwrap_err();

    match err {
        ErrorWrapper::Url(msg) => {
            assert!(!msg.is_empty());
        }
        other => panic!("expected ErrorWrapper::Url, got {other:?}"),
    }
}

/// Error types should compose with `?` via the standard Error trait.
#[tokio::test]
async fn test_error_implements_std_error() {
    let api = StravaAPI::new("http://127.0.0.1:1", "foo");
    let result: Result<(), Box<dyn std::error::Error>> = async {
        api.athlete().get().send().await?;
        Ok(())
    }
    .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(!err.to_string().is_empty());
}
