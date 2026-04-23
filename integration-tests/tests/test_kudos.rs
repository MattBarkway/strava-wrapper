use httpmock::MockServer;
use serde_json::json;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{Sendable, ID};

#[tokio::test]
async fn test_list_activity_kudoers() {
    let server = MockServer::start();

    let raw_json = json!([
        { "id": 1, "firstname": "Alan", "lastname": "Turing" },
        { "id": 2, "firstname": "Grace", "lastname": "Hopper" }
    ]);

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/activities/123/kudos");
        then.status(200).json_body(raw_json);
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let result = api.activities().kudos().id(123).send().await.unwrap();

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].firstname.as_deref(), Some("Alan"));
    mock.assert();
}
