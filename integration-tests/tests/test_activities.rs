
mod test {
    use httpmock::MockServer;
    use serde_json::json;
    use strava_wrapper::api::StravaAPI;
    use strava_wrapper::models::{Activity, Comment};
    use strava_wrapper::query::{IncludeAllEfforts, Sendable, ID};

    #[tokio::test]
    async fn test_get_activity() {
        let server = MockServer::start();

        let raw_json = json!([{
            // TODO
        }]);
        let expected: Activity = serde_json::from_value(raw_json.clone()).unwrap();

        let mock = server.mock(|when, then| {
            when.method("GET").path("/v3/activities/123");
            then.status(200).json_body(raw_json);
        });

        let api = StravaAPI::new(&server.base_url(), "foo");

        let result = api
            .activities()
            .get()
            .id(123)
            .send()
            .await
            .unwrap();

        assert_eq!(result, expected);

        mock.assert();
    }

    #[tokio::test]
    async fn test_get_activity_include_all_efforts() {
        let server = MockServer::start();

        let raw_json = json!([{
            // TODO
        }]);
        let expected: Activity = serde_json::from_value(raw_json.clone()).unwrap();

        let mock = server.mock(|when, then| {
            when.method("GET").path("/v3/activities/123");
            then.status(200).json_body(raw_json);
        });

        let api = StravaAPI::new(&server.base_url(), "foo");

        let result = api
            .activities()
            .get()
            .id(123)
            .include_all_efforts(true)
            .send()
            .await
            .unwrap();

        assert_eq!(result, expected);

        mock.assert();
    }
}
