use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::models::{CreateActivity, UpdatableActivity};
use strava_wrapper::query::Sendable;

#[tokio::test]
async fn test_create_activity() {
    let server = MockServer::start();

    let response = json!({
        "id": 999,
        "name": "Morning Ride",
        "sport_type": "Ride",
        "elapsed_time": 1800
    });

    let mock = server.mock(|when, then| {
        when.method("POST")
            .path("/v3/activities")
            .header("authorization", "Bearer foo");
        then.status(201).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let body = CreateActivity {
        name: "Morning Ride".to_string(),
        activity_type: None,
        sport_type: "Ride".to_string(),
        start_date_local: "2024-01-01T08:00:00".to_string(),
        elapsed_time: 1800,
        description: None,
        distance: None,
        trainer: None,
        commute: None,
    };
    let result = api.activities().create(body).send().await.unwrap();

    assert_eq!(result.id, Some(999));
    assert_eq!(result.name.as_deref(), Some("Morning Ride"));
    mock.assert();
}

#[tokio::test]
async fn test_update_activity() {
    let server = MockServer::start();

    let response = json!({
        "id": 42,
        "name": "Evening Ride",
        "commute": true
    });

    let mock = server.mock(|when, then| {
        when.method("PUT")
            .path("/v3/activities/42")
            .json_body(json!({
                "commute": true,
                "trainer": null,
                "hide_from_home": null,
                "description": null,
                "name": "Evening Ride",
                "type": null,
                "sport_type": null,
                "gear_id": null
            }));
        then.status(200).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let update = UpdatableActivity {
        name: Some("Evening Ride".to_string()),
        commute: Some(true),
        ..Default::default()
    };
    let result = api.activities().update(42, update).send().await.unwrap();

    assert_eq!(result.name.as_deref(), Some("Evening Ride"));
    mock.assert();
}
