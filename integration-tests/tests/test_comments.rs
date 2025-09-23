mod test {
    use httpmock::MockServer;
    use serde_json::json;
    use strava_wrapper::api::StravaAPI;
    use strava_wrapper::models::Comment;
    use strava_wrapper::query::{Sendable, ID};

    #[tokio::test]
    async fn test_get_comments() {
        let server = MockServer::start();

        let raw_json = json!([{
            "id": 1,
            "activity_id": 123,
            "text": "Nice ride!",
            "resource_state": 2,
            "post_id": null,
            "mentions_metadata": null,
            "created_at": "2025-01-01T00:00:00Z",
            "cursor": null,
            "athlete": { "firstname": "John", "lastname": "D" },
            "reaction_count": 0,
            "has_reacted": false
        }]);
        let expected: Vec<Comment> = serde_json::from_value(raw_json.clone()).unwrap();

        let mock = server.mock(|when, then| {
            when.method("GET").path("/v3/activities/123/comments");
            then.status(200).json_body(raw_json);
        });

        let api = StravaAPI::new(&server.base_url(), "foo");

        let result = api
            .activities()
            .comments()
            .id(123)
            .send()
            .await
            .unwrap();

        assert_eq!(result, expected);

        mock.assert();
    }
}
