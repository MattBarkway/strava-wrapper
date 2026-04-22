use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::filters::uploads::UploadDataType;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_upload_activity() {
    let server = MockServer::start();

    let response = json!({
        "id": 12345,
        "id_str": "12345",
        "external_id": "garmin_push_7890",
        "error": null,
        "status": "Your activity is being processed.",
        "activity_id": null
    });

    let mock = server.mock(|when, then| {
        when.method("POST")
            .path("/v3/uploads")
            .header_exists("content-type");
        then.status(201).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api
        .uploads()
        .upload(vec![0x00, 0x01, 0x02], UploadDataType::Fit)
        .name("Morning Ride")
        .description("felt good")
        .external_id("garmin_push_7890")
        .send()
        .await
        .unwrap();

    assert_eq!(result.id, Some(12345));
    assert_eq!(
        result.status.as_deref(),
        Some("Your activity is being processed.")
    );
    mock.assert();
}

#[tokio::test]
async fn test_get_upload_status() {
    let server = MockServer::start();

    let response = json!({
        "id": 12345,
        "id_str": "12345",
        "external_id": null,
        "error": null,
        "status": "Your activity is ready.",
        "activity_id": 99
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/uploads/12345");
        then.status(200).json_body(response);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.uploads().get().id(12345).send().await.unwrap();

    assert_eq!(result.activity_id, Some(99));
    mock.assert();
}
