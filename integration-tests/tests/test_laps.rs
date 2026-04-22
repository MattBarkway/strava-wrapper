use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_list_activity_laps() {
    let server = MockServer::start();

    let raw_json = json!([
        {
            "id": 1,
            "lap_index": 1,
            "name": "Lap 1",
            "distance": 1000.0,
            "elapsed_time": 300,
            "moving_time": 290,
            "max_speed": 8.5,
            "average_speed": 6.2
        }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/activities/123/laps");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.activities().laps().id(123).send().await.unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name.as_deref(), Some("Lap 1"));
    assert_eq!(result[0].distance, Some(1000.0));
    mock.assert();
}
