# Strava API Wrapper

An asynchronous wrapper for the [Strava API](https://developers.strava.com/), written in Rust.

Builder-style, trait-driven, and covers the full public v3 surface: activities, athletes, clubs, gear, routes, segments, segment efforts, streams, and uploads, plus the OAuth token and deauthorize flows.

---

## Installation

```toml
[dependencies]
strava-wrapper = "0.1.0"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

---

## Quick start

```rust
use strava_wrapper::api::StravaAPI;
use strava_wrapper::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = StravaAPI::new("https://www.strava.com/api", "YOUR_ACCESS_TOKEN");

    // Read the authenticated athlete.
    let me = api.athlete().get().send().await?;

    // Fetch an activity by id.
    let activity = api.activities().get().id(123).send().await?;

    // Paginate.
    let comments = api
        .activities()
        .comments()
        .id(123)
        .per_page(10)
        .page(2)
        .send()
        .await?;

    Ok(())
}
```

## OAuth

```rust
use strava_wrapper::auth::{get_token, refresh_token, deauthorize};

// 1. Exchange authorization code for an access token.
let token = get_token(client_id, &client_secret, &auth_code).await?;

// 2. Later, swap a soon-to-expire access token for a new one.
let refreshed = refresh_token(client_id, &client_secret, &token.refresh_token).await?;
api.set_token(refreshed.access_token);

// 3. Revoke access on logout.
deauthorize(&api.token()).await?;
```

## Writes

```rust
use strava_wrapper::models::{CreateActivity, UpdatableActivity};

// Create a manual activity.
api.activities().create(CreateActivity {
    name: "Commute".into(),
    sport_type: "Ride".into(),
    start_date_local: "2024-06-01T08:00:00".into(),
    elapsed_time: 1800,
    activity_type: None,
    description: None,
    distance: None,
    trainer: None,
    commute: Some(1),
}).send().await?;

// Update fields on an existing activity.
api.activities().update(456, UpdatableActivity {
    name: Some("Evening Ride".into()),
    commute: Some(true),
    ..Default::default()
}).send().await?;

// Star a segment.
api.segments().star(789, true).send().await?;

// Update weight.
api.athlete().update(70.5).send().await?;
```

## Streams

```rust
use strava_wrapper::filters::streams::StreamKey;

let streams = api
    .streams()
    .activity()
    .id(123)
    .keys(&[StreamKey::Time, StreamKey::Heartrate, StreamKey::Watts])
    .send()
    .await?;
```

## Uploads

```rust
use strava_wrapper::filters::uploads::UploadDataType;

let bytes = std::fs::read("ride.fit")?;
let upload = api
    .uploads()
    .upload(bytes, UploadDataType::Fit)
    .name("Morning Ride")
    .send()
    .await?;

// Poll processing status.
let status = api.uploads().get().id(upload.id.unwrap()).send().await?;
```

---

## Errors

All network calls return `Result<T, ErrorWrapper>`. `ErrorWrapper` is an enum with `Network`, `Parse`, `Api`, and `Url` variants and implements `Display` + `std::error::Error`, so it composes with `?` in your own error types.

---

## Testing

```bash
cargo test --all
```
