use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{After, Before, Page, PerPage, Sendable};

#[tokio::test]
async fn test_get_athlete() {
    let server = MockServer::start();

    let raw_json = json!({
        "id": 42,
        "resource_state": 3,
        "firstname": "Ada",
        "lastname": "Lovelace"
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athlete");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.athlete().get().send().await.unwrap();

    assert_eq!(result.id, Some(42));
    assert_eq!(result.firstname.as_deref(), Some("Ada"));
    mock.assert();
}

#[tokio::test]
async fn test_get_athlete_clubs() {
    let server = MockServer::start();

    let raw_json = json!([
        { "id": 7, "resource_state": 2, "name": "Lovelace Riders" }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athlete/clubs");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.athlete().clubs().send().await.unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name.as_deref(), Some("Lovelace Riders"));
    mock.assert();
}

#[tokio::test]
async fn test_list_athlete_activities_basic() {
    let server = MockServer::start();

    let raw_json = json!([
        {
            "id": 1,
            "resource_state": 2,
            "athlete": { "id": 42, "resource_state": 1 },
            "name": "Ride",
            "distance": 1000.0,
            "moving_time": 600,
            "elapsed_time": 650,
            "total_elevation_gain": 10.0,
            "type": "Ride",
            "sport_type": "Ride",
            "start_date": "2024-01-01T08:00:00Z",
            "start_date_local": "2024-01-01T08:00:00",
            "timezone": "UTC",
            "utc_offset": 0.0,
            "achievement_count": 0,
            "kudos_count": 0,
            "comment_count": 0,
            "athlete_count": 1,
            "photo_count": 0,
            "map": { "id": "m", "polyline": null, "resource_state": 2 },
            "trainer": false,
            "commute": false,
            "manual": false,
            "private": false,
            "flagged": false,
            "average_speed": 5.0,
            "max_speed": 10.0,
            "has_heartrate": false,
            "pr_count": 0,
            "total_photo_count": 0,
            "has_kudoed": false
        }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athlete/activities");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.athlete().activities().send().await.unwrap();

    assert_eq!(result.len(), 1);
    mock.assert();
}

/// Asserts every pagination trait (Page, PerPage, Before, After) lands
/// as its expected query param on the wire.
#[tokio::test]
async fn test_list_athlete_activities_pagination() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/athlete/activities")
            .query_param("page", "3")
            .query_param("per_page", "25")
            .query_param("before", "1700000000")
            .query_param("after", "1690000000");
        then.status(200).json_body(json!([]));
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api
        .athlete()
        .activities()
        .page(3)
        .per_page(25)
        .before(1_700_000_000)
        .after(1_690_000_000)
        .send()
        .await
        .unwrap();

    assert!(result.is_empty());
    mock.assert();
}
