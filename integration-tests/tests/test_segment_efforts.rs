use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_get_segment_effort() {
    let server = MockServer::start();

    let raw_json = json!({
        "id": 500,
        "activity_id": 123,
        "elapsed_time": 180,
        "moving_time": 175,
        "distance": 1000.0,
        "start_date": "2024-01-01T09:00:00Z",
        "start_date_local": "2024-01-01T09:00:00Z",
        "is_kom": false,
        "name": "Sprint Stretch",
        "average_watts": 280.0,
        "device_watts": true,
        "kom_rank": null,
        "pr_rank": 2,
        "hidden": false
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/segment_efforts/500");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.segments().efforts().get().id(500).send().await.unwrap();

    assert_eq!(result.id, Some(500));
    assert_eq!(result.name.as_deref(), Some("Sprint Stretch"));
    mock.assert();
}

#[tokio::test]
async fn test_list_segment_efforts() {
    let server = MockServer::start();

    let raw_json = json!([
        {
            "id": 1,
            "activity_id": 200,
            "elapsed_time": 250,
            "distance": 1200.0
        },
        {
            "id": 2,
            "activity_id": 201,
            "elapsed_time": 240,
            "distance": 1200.0
        }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/segment_efforts")
            .query_param("segment_id", "789")
            .query_param("per_page", "50");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    // note: .segment_id() is an inherent method on ListSegmentEfforts; we still
    // need PerPage from the prelude to push the per_page query param.
    use strava_wrapper::query::PerPage;
    let result = api
        .segments()
        .efforts()
        .list()
        .segment_id(789)
        .per_page(50)
        .send()
        .await
        .unwrap();

    assert_eq!(result.len(), 2);
    mock.assert();
}
