use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::Sendable;

#[tokio::test]
async fn test_star_segment() {
    let server = MockServer::start();

    let response = json!({
        "id": 789,
        "name": "Test Climb"
    });

    let mock = server.mock(|when, then| {
        when.method("PUT")
            .path("/v3/segments/789/starred")
            .json_body(json!({ "starred": true }));
        then.status(200).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.segments().star(789, true).send().await.unwrap();

    assert_eq!(result.id, Some(789));
    mock.assert();
}

#[tokio::test]
async fn test_unstar_segment() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method("PUT")
            .path("/v3/segments/789/starred")
            .json_body(json!({ "starred": false }));
        then.status(200).json_body(json!({ "id": 789 }));
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    api.segments().star(789, false).send().await.unwrap();

    mock.assert();
}
