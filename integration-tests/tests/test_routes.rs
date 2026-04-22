use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_get_route() {
    let server = MockServer::start();

    let raw_json = json!({
        "id": 999,
        "name": "Weekend Loop",
        "distance": 45000.0,
        "elevation_gain": 600.0,
        "private": false,
        "starred": true,
        "type": 1,
        "sub_type": 2
    });

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/routes/999");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.routes().get().id(999).send().await.unwrap();

    assert_eq!(result.id, Some(999));
    assert_eq!(result.name.as_deref(), Some("Weekend Loop"));
    mock.assert();
}

const SAMPLE_GPX: &str = r#"<?xml version="1.0"?>
<gpx version="1.1"><trk><trkseg><trkpt lat="51.5" lon="-0.1"/></trkseg></trk></gpx>"#;

const SAMPLE_TCX: &str = r#"<?xml version="1.0"?>
<TrainingCenterDatabase><Courses><Course><Name>Test</Name></Course></Courses></TrainingCenterDatabase>"#;

#[tokio::test]
async fn test_export_route_gpx() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/routes/999/export_gpx");
        then.status(200)
            .header("content-type", "application/gpx+xml")
            .body(SAMPLE_GPX);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.routes().export().gpx().id(999).send().await.unwrap();

    assert!(result.contains("<gpx"));
    assert!(result.contains("trkpt"));
    mock.assert();
}

#[tokio::test]
async fn test_export_route_tcx() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/routes/999/export_tcx");
        then.status(200)
            .header("content-type", "application/vnd.garmin.tcx+xml")
            .body(SAMPLE_TCX);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.routes().export().tcx().id(999).send().await.unwrap();

    assert!(result.contains("<TrainingCenterDatabase>"));
    mock.assert();
}
