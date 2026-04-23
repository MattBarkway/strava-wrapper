use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_get_athlete_stats() {
    let server = MockServer::start();

    let raw_json = json!({
        "biggest_ride_distance": 150000.0,
        "biggest_climb_elevation_gain": 1200.0,
        "recent_ride_totals": {
            "count": 5,
            "distance": 300000.0,
            "moving_time": 36000.0,
            "elapsed_time": 37000.0,
            "elevation_gain": 2500.0,
            "achievement_count": 3
        },
        "ytd_ride_totals": {
            "count": 50,
            "distance": 3000000.0,
            "moving_time": 360000.0,
            "elapsed_time": 370000.0,
            "elevation_gain": 25000.0,
            "achievement_count": 30
        }
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athletes/42/stats");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.athletes().stats().id(42).send().await.unwrap();

    assert_eq!(result.biggest_ride_distance, Some(150000.0));
    assert!(result.recent_ride_totals.is_some());
    mock.assert();
}

#[tokio::test]
async fn test_list_athlete_routes() {
    let server = MockServer::start();

    let raw_json = json!([
        {
            "id": 1,
            "name": "Commute",
            "distance": 15000.0,
            "elevation_gain": 150.0,
            "private": false,
            "starred": true
        }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athletes/42/routes");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.athletes().routes().id(42).send().await.unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name.as_deref(), Some("Commute"));
    mock.assert();
}
