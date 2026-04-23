use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::Sendable;

/// set_token updates the Arc<RwLock<String>> in-place, so subsequent
/// endpoint calls send the new token even on the same StravaAPI instance.
#[tokio::test]
async fn test_set_token_updates_subsequent_requests() {
    let server = MockServer::start();

    let old_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/athlete")
            .header("authorization", "Bearer old-token");
        then.status(200).json_body(json!({ "id": 1 }));
    });

    let new_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/athlete")
            .header("authorization", "Bearer new-token");
        then.status(200).json_body(json!({ "id": 2 }));
    });

    let api = StravaAPI::new(&server.base_url(), "old-token");

    let first = api.athlete().get().send().await.unwrap();
    assert_eq!(first.id, Some(1));

    api.set_token("new-token");
    assert_eq!(api.token(), "new-token");

    let second = api.athlete().get().send().await.unwrap();
    assert_eq!(second.id, Some(2));

    old_mock.assert();
    new_mock.assert();
}

/// Clones of StravaAPI share the underlying Arc<RwLock<String>>, so a
/// token update on one handle is visible to every clone.
#[tokio::test]
async fn test_set_token_visible_across_clones() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/athlete")
            .header("authorization", "Bearer refreshed");
        then.status(200).json_body(json!({ "id": 99 }));
    });

    let primary = StravaAPI::new(&server.base_url(), "stale");
    let clone = primary.clone();

    // Update on the clone; the primary should see it too.
    clone.set_token("refreshed");
    assert_eq!(primary.token(), "refreshed");

    let result = primary.athlete().get().send().await.unwrap();
    assert_eq!(result.id, Some(99));
    mock.assert();
}
