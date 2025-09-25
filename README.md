# Strava API Wrapper

An asynchronous wrapper for the [Strava API](https://developers.strava.com/), written in Rust.

This library provides a builder-style interface for accessing Strava API endpoints.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
strava-api-wrapper = "0.1.0"
```

---

## Usage

```rust
use strava_api::StravaAPI;

let api = StravaAPI::new("https://www.strava.com/api", "YOUR_ACCESS_TOKEN");

// Get an activity by ID
let activity = api
    .activities()
    .get()
    .id(123)
    .send()
    .await?;
```
```rust
// Get comments for activity by ID
let comments = api.activities()
    .comments()
    .get()
    .id(123)
    .per_page(10)
    .page(2)
    .send()
    .await?;
```
---

## Testing

```bash
cargo test
```
