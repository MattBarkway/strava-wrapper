use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::filters::streams::StreamKey;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_get_activity_streams() {
    let server = MockServer::start();

    let response = json!({
        "time": {
            "original_size": 3,
            "resolution": "high",
            "series_type": "distance",
            "data": [0, 1, 2]
        },
        "heartrate": {
            "original_size": 3,
            "resolution": "high",
            "series_type": "distance",
            "data": [120, 130, 140]
        }
    });

    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/activities/123/streams")
            .query_param("keys", "time,heartrate")
            .query_param("key_by_type", "true");
        then.status(200).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api
        .streams()
        .activity()
        .id(123)
        .keys(&[StreamKey::Time, StreamKey::Heartrate])
        .send()
        .await
        .unwrap();

    assert!(result.time.is_some());
    assert!(result.heartrate.is_some());
    mock.assert();
}

#[tokio::test]
async fn test_get_route_streams() {
    let server = MockServer::start();

    let response = json!({
        "altitude": {
            "original_size": 2,
            "resolution": "high",
            "series_type": "distance",
            "data": [10.0, 15.5]
        }
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/routes/321/streams");
        then.status(200).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.streams().route().id(321).send().await.unwrap();

    assert!(result.altitude.is_some());
    mock.assert();
}

#[tokio::test]
async fn test_get_segment_effort_streams() {
    let server = MockServer::start();

    let response = json!({
        "watts": {
            "original_size": 3,
            "resolution": "high",
            "series_type": "distance",
            "data": [200, 210, 220]
        }
    });

    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/segment_efforts/500/streams")
            .query_param("keys", "watts")
            .query_param("key_by_type", "true");
        then.status(200).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api
        .streams()
        .segment_effort()
        .id(500)
        .keys(&[StreamKey::Watts])
        .send()
        .await
        .unwrap();

    assert!(result.watts.is_some());
    mock.assert();
}

#[tokio::test]
async fn test_get_segment_streams() {
    let server = MockServer::start();

    let response = json!({
        "distance": {
            "original_size": 2,
            "resolution": "high",
            "series_type": "distance",
            "data": [0.0, 100.0]
        }
    });

    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/v3/segments/456/streams")
            .query_param("keys", "distance")
            .query_param("key_by_type", "true");
        then.status(200).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api
        .streams()
        .segment()
        .id(456)
        .keys(&[StreamKey::Distance])
        .send()
        .await
        .unwrap();

    assert!(result.distance.is_some());
    mock.assert();
}
