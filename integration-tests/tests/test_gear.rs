use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{GearID, Sendable};

#[tokio::test]
async fn test_get_gear() {
    let server = MockServer::start();

    let raw_json = json!({
        "id": "b12345678",
        "resource_state": 3,
        "primary": true,
        "name": "Canyon Ultimate",
        "distance": 5000.0,
        "brand_name": "Canyon",
        "model_name": "Ultimate",
        "frame_type": 3
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/gear/b12345678");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.gear().get().id("b12345678").send().await.unwrap();

    assert_eq!(result.id.as_deref(), Some("b12345678"));
    assert_eq!(result.name.as_deref(), Some("Canyon Ultimate"));
    assert_eq!(result.brand_name.as_deref(), Some("Canyon"));
    mock.assert();
}
