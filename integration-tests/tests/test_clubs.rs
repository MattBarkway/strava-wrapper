mod test {
    use httpmock::MockServer;
    use serde_json::json;
    use strava_wrapper::api::StravaAPI;
    use strava_wrapper::models::SimpleAthlete;
    use strava_wrapper::query::{Sendable, ID};

    #[tokio::test]
    async fn test_get_club() {
        let server = MockServer::start();

        let raw_json = json!([{
            // TODO
        }]);
        let expected: SimpleAthlete = serde_json::from_value(raw_json.clone()).unwrap();

        let mock = server.mock(|when, then| {
            when.method("GET").path("/v3/clubs/{id}");
            then.status(200).json_body(raw_json);
        });

        let api = StravaAPI::new(&server.base_url(), "foo");

        let result = api.clubs().get().id(123).send().await.unwrap();

        assert_eq!(result, expected);

        mock.assert();
    }

    #[tokio::test]
    async fn test_list_club_members() {
        let server = MockServer::start();

        let raw_json = json!([{
            // TODO
        }]);
        let expected: SimpleAthlete = serde_json::from_value(raw_json.clone()).unwrap();

        let mock = server.mock(|when, then| {
            when.method("GET").path("/v3/clubs/{id}");
            then.status(200).json_body(raw_json);
        });

        let api = StravaAPI::new(&server.base_url(), "foo");

        let result = api.clubs().members().id(123).send().await.unwrap();

        assert_eq!(result, expected);

        mock.assert();
    }

    #[tokio::test]
    async fn test_list_club_activities() {
        let server = MockServer::start();

        let raw_json = json!([{
            // TODO
        }]);
        let expected: SimpleAthlete = serde_json::from_value(raw_json.clone()).unwrap();

        let mock = server.mock(|when, then| {
            when.method("GET").path("/v3/clubs/{id}");
            then.status(200).json_body(raw_json);
        });

        let api = StravaAPI::new(&server.base_url(), "foo");

        let result = api.clubs().activities().id(123).send().await.unwrap();

        assert_eq!(result, expected);

        mock.assert();
    }

    #[tokio::test]
    async fn test_list_club_admins() {
        let server = MockServer::start();

        let raw_json = json!([{
            // TODO
        }]);
        let expected: SimpleAthlete = serde_json::from_value(raw_json.clone()).unwrap();

        let mock = server.mock(|when, then| {
            when.method("GET").path("/v3/clubs/{id}");
            then.status(200).json_body(raw_json);
        });

        let api = StravaAPI::new(&server.base_url(), "foo");

        let result = api.clubs().admins().id(123).send().await.unwrap();

        assert_eq!(result, expected);

        mock.assert();
    }
}
