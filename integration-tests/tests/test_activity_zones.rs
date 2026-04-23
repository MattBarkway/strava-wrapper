use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_list_activity_zones() {
    let server = MockServer::start();

    let raw_json = json!([
        {
            "score": 10,
            "distribution_buckets": [],
            "type": "heartrate",
            "sensor_based": true,
            "points": 5,
            "custom_zones": false,
            "max": 190
        }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/activities/123/zones");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.activities().zones().id(123).send().await.unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].zone_type.as_deref(), Some("heartrate"));
    mock.assert();
}
