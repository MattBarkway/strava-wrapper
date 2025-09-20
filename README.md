# Strava API Rust Wrapper

An asynchronous Rust wrapper for the [Strava API](https://developers.strava.com/).

This library provides a builder-style interface for accessing Strava endpoints like activities, comments, and kudos, with automatic JSON deserialization.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
strava-api-wrapper = "0.1.0"
...
```

---

## Usage

```rust
use strava_api::StravaAPI;

let token = "YOUR_ACCESS_TOKEN";
let api = StravaAPI::new("https://www.strava.com/api/v3", token);

let activity = api
    .activities()
    .get()
    .id(123)
    .send()
    .await?;
```
```rust
// Get comments for activity 123
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

## Supported Endpoints

| Endpoint          | Chain Syntax                               | Returns                  |
| ----------------- |--------------------------------------------| ------------------------ |
| Get Activity      | `api.activities().id(123).send()`          | `Activity`               |
| Activity Comments | `api.activities().get().comments().send()` | `Vec<Comment>`           |
| Activity Kudos    | `api.activities().id(123).kudos().send()`  | `Vec<User>`              |
| Create Activity   | `api.create_activity(payload).send()`      | `Activity`               |

---

## Testing

```bash
cargo test
```
