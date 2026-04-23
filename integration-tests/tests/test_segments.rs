use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_get_segment() {
    let server = MockServer::start();

    let raw_json = json!({
        "id": 789,
        "name": "Box Hill",
        "activity_type": "Ride",
        "distance": 2500.0,
        "average_grade": 5.0,
        "maximum_grade": 9.0,
        "elevation_high": 180.0,
        "elevation_low": 50.0,
        "start_latlng": [51.25, -0.31],
        "end_latlng": [51.26, -0.30],
        "climb_category": 4,
        "city": "Mickleham",
        "country": "United Kingdom",
        "private": false,
        "effort_count": 10000,
        "athlete_count": 5000,
        "hazardous": false,
        "star_count": 1200,
        "total_elevation_gain": 130.0
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/segments/789");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.segments().get().id(789).send().await.unwrap();

    assert_eq!(result.id, Some(789));
    assert_eq!(result.name.as_deref(), Some("Box Hill"));
    assert_eq!(result.climb_category, Some(4));
    mock.assert();
}

#[tokio::test]
async fn test_list_starred_segments() {
    let server = MockServer::start();

    let raw_json = json!([
        {
            "id": 1,
            "name": "Local Climb",
            "activity_type": "Ride",
            "distance": 500.0,
            "average_grade": 4.5,
            "maximum_grade": 7.0,
            "elevation_high": 100.0,
            "elevation_low": 70.0,
            "climb_category": 3
        }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/segments/starred");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.segments().starred().send().await.unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name.as_deref(), Some("Local Climb"));
    mock.assert();
}

#[tokio::test]
async fn test_explore_segments() {
    let server = MockServer::start();

    let raw_json = json!({
        "segments": [
            {
                "id": 42,
                "name": "Discovered Climb",
                "climb_category": 2,
                "climb_category_desc": "3",
                "avg_grade": 6.3,
                "start_latlng": [51.5, -0.1],
                "end_latlng": [51.51, -0.09],
                "elev_difference": 45.0,
                "distance": 1500.0,
                "points": "abc"
            }
        ]
    });

    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/segments/explore")
            .query_param("bounds", "51.4,-0.2,51.6,0.1")
            .query_param("activity_type", "riding")
            .query_param("min_cat", "1")
            .query_param("max_cat", "5");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api
        .segments()
        .explore()
        .bounds(51.4, -0.2, 51.6, 0.1)
        .activity_type("riding")
        .min_cat(1)
        .max_cat(5)
        .send()
        .await
        .unwrap();

    let segments = result.segments.expect("segments");
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].name.as_deref(), Some("Discovered Climb"));
    mock.assert();
}
