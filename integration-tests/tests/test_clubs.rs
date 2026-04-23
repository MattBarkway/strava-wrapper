use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_get_club() {
    let server = MockServer::start();

    let raw_json = json!({
        "id": 456,
        "resource_state": 3,
        "name": "Test Club",
        "activity_types": ["Ride"],
        "member_count": 10
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/clubs/456");
        then.status(200).json_body(raw_json.clone());
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.clubs().get().id(456).send().await.unwrap();

    assert_eq!(result.id, Some(456));
    assert_eq!(result.name.as_deref(), Some("Test Club"));
    mock.assert();
}

#[tokio::test]
async fn test_list_club_members() {
    let server = MockServer::start();

    let raw_json = json!([
        { "resource_state": 2, "firstname": "Ada", "lastname": "Lovelace", "admin": false, "owner": false }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/clubs/456/members");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.clubs().members().id(456).send().await.unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].firstname.as_deref(), Some("Ada"));
    mock.assert();
}

#[tokio::test]
async fn test_list_club_activities() {
    let server = MockServer::start();

    let raw_json = json!([
        {
            "athlete": { "id": 42 },
            "name": "Club Ride",
            "distance": 1000.0,
            "moving_time": 600,
            "elapsed_time": 650,
            "total_elevation_gain": 20.0,
            "sport_type": "Ride",
            "workout_type": null
        }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/clubs/456/activities");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.clubs().activities().id(456).send().await.unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name.as_deref(), Some("Club Ride"));
    mock.assert();
}

#[tokio::test]
async fn test_list_club_admins() {
    let server = MockServer::start();

    let raw_json = json!([
        { "id": 1, "firstname": "Grace", "lastname": "Hopper" }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/clubs/456/admins");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.clubs().admins().id(456).send().await.unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].firstname.as_deref(), Some("Grace"));
    mock.assert();
}
