use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::models::Activity;
use strava_wrapper::query::{IncludeAllEfforts, Sendable, ID};

fn activity_json() -> serde_json::Value {
    json!({
        "id": 123,
        "resource_state": 3,
        "external_id": null,
        "upload_id": null,
        "athlete": { "id": 42, "resource_state": 1 },
        "name": "Morning Ride",
        "distance": 12345.6,
        "moving_time": 3600,
        "elapsed_time": 3700,
        "total_elevation_gain": 150.0,
        "type": "Ride",
        "sport_type": "Ride",
        "start_date": "2024-01-01T08:00:00Z",
        "start_date_local": "2024-01-01T08:00:00",
        "timezone": "(GMT+00:00) Europe/London",
        "utc_offset": 0.0,
        "achievement_count": 0,
        "kudos_count": 5,
        "comment_count": 1,
        "athlete_count": 1,
        "photo_count": 0,
        "map": { "id": "a123", "polyline": null, "resource_state": 2 },
        "trainer": false,
        "commute": false,
        "manual": false,
        "private": false,
        "flagged": false,
        "gear_id": null,
        "from_accepted_tag": null,
        "average_speed": 6.2,
        "max_speed": 12.3,
        "device_watts": null,
        "has_heartrate": true,
        "pr_count": 0,
        "total_photo_count": 0,
        "has_kudoed": false,
        "workout_type": null,
        "description": null,
        "calories": 400.0,
        "segment_efforts": null
    })
}

#[tokio::test]
async fn test_get_activity() {
    let server = MockServer::start();
    let raw_json = activity_json();
    let expected: Activity = serde_json::from_value(raw_json.clone()).unwrap();

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/activities/123");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.activities().get().id(123).send().await.unwrap();

    assert_eq!(result, expected);
    mock.assert();
}

#[tokio::test]
async fn test_get_activity_include_all_efforts() {
    let server = MockServer::start();
    let raw_json = activity_json();
    let expected: Activity = serde_json::from_value(raw_json.clone()).unwrap();

    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/activities/123")
            .query_param("include_all_efforts", "true");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api
        .activities()
        .get()
        .id(123)
        .include_all_efforts(true)
        .send()
        .await
        .unwrap();

    assert_eq!(result, expected);
    mock.assert();
}
