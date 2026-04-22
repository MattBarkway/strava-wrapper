use httpmock::MockServer;
use serde_json::json;
use std::sync::Mutex;
use strava_wrapper::api::StravaAPI;
use strava_wrapper::query::{last_rate_limit, ErrorWrapper, RateLimit, Sendable, ID};

// All rate-limit tests in this file share the process-wide snapshot, so they
// must serialize to avoid stomping on each other's assertions. A plain Mutex
// is enough — the lock is only held for one HTTP round-trip per test.
static LOCK: Mutex<()> = Mutex::new(());

#[tokio::test]
async fn test_rate_limit_parsed_from_success() {
    let _guard = LOCK.lock().unwrap();
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athlete");
        then.status(200)
            .header("x-ratelimit-limit", "100,1000")
            .header("x-ratelimit-usage", "42,450")
            .json_body(json!({ "id": 1 }));
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    api.athlete().get().send().await.unwrap();

    let rl = api
        .rate_limit()
        .expect("rate limit should have been recorded");
    assert_eq!(rl.short_term_usage, 42);
    assert_eq!(rl.short_term_limit, 100);
    assert_eq!(rl.long_term_usage, 450);
    assert_eq!(rl.long_term_limit, 1000);
    assert_eq!(rl.short_term_remaining(), 58);
    assert_eq!(rl.long_term_remaining(), 550);
    mock.assert();
}

#[tokio::test]
async fn test_rate_limit_attached_to_api_error() {
    let _guard = LOCK.lock().unwrap();
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method("GET").path("/v3/athlete");
        then.status(429)
            .header("x-ratelimit-limit", "100,1000")
            .header("x-ratelimit-usage", "100,720")
            .json_body(json!({ "message": "Rate Limit Exceeded" }));
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    let err = api.athlete().get().send().await.unwrap_err();

    match err {
        ErrorWrapper::Api {
            status, rate_limit, ..
        } => {
            assert_eq!(status.as_u16(), 429);
            let rl = rate_limit.expect("rate limit on 429");
            assert_eq!(rl.short_term_usage, 100);
            assert_eq!(rl.short_term_remaining(), 0);
        }
        other => panic!("expected Api error, got {other:?}"),
    }

    // And the snapshot is also stored globally.
    let stored = last_rate_limit().expect("global snapshot");
    assert_eq!(stored.short_term_usage, 100);
}

#[tokio::test]
async fn test_rate_limit_none_when_headers_absent() {
    let _guard = LOCK.lock().unwrap();
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method("GET").path("/v3/activities/1");
        then.status(200).body("not-valid-json");
    });

    let api = StravaAPI::new(&server.base_url(), "foo");
    // Parse error path — headers were never sent, so RateLimit::from_headers
    // returns None and we don't spuriously store a stale entry.
    let _ = api.activities().get().id(1).send().await;

    // Note: we can't assert rate_limit() is None without risk of flakiness,
    // because an earlier test may have populated the global slot. Instead,
    // verify the parser returns None for this response directly.
    let no_headers = reqwest::header::HeaderMap::new();
    assert!(RateLimit::from_headers(&no_headers).is_none());
}

#[test]
fn test_rate_limit_parses_pairs() {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-ratelimit-limit", "100,1000".parse().unwrap());
    headers.insert("x-ratelimit-usage", "  7 , 42 ".parse().unwrap());

    let rl = RateLimit::from_headers(&headers).expect("parsed");
    assert_eq!(rl.short_term_usage, 7);
    assert_eq!(rl.long_term_usage, 42);
    assert_eq!(rl.short_term_remaining(), 93);
}

#[test]
fn test_rate_limit_rejects_malformed() {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-ratelimit-limit", "abc,def".parse().unwrap());
    headers.insert("x-ratelimit-usage", "1,2".parse().unwrap());
    assert!(RateLimit::from_headers(&headers).is_none());

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-ratelimit-limit", "100".parse().unwrap()); // missing second value
    headers.insert("x-ratelimit-usage", "1,2".parse().unwrap());
    assert!(RateLimit::from_headers(&headers).is_none());
}
