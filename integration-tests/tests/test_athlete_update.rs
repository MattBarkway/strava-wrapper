use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::Sendable;

#[tokio::test]
async fn test_update_athlete_weight() {
    let server = MockServer::start();

    let response = json!({
        "id": 42,
        "firstname": "Ada",
        "lastname": "Lovelace",
        "weight": 70.5
    });

    let mock = server.mock(|when, then| {
        when.method("PUT").path("/v3/athlete").body("weight=70.5");
        then.status(200).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.athlete().update(70.5).send().await.unwrap();

    assert_eq!(result.weight, Some(70.5));
    mock.assert();
}
