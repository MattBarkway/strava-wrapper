use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::Sendable;

#[tokio::test]
async fn test_get_athlete_zones() {
    let server = MockServer::start();

    let raw_json = json!({
        "heart_rate": {
            "custom_zones": false,
            "zones": {
                "zones": [
                    { "min": 0, "max": 120 },
                    { "min": 120, "max": 150 }
                ]
            }
        },
        "power": null
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athlete/zones");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.athlete().zones().send().await.unwrap();

    assert!(result.heart_rate.is_some());
    assert!(result.power.is_none());
    mock.assert();
}
