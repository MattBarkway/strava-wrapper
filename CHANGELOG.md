# Changelog

## 0.1.0

0.0.2 didn't really work — most filters had placeholder return types, any
network error panicked. Breaking across the board, hence 0.1.0.

### Fixed

- Return types on ~15 filters that were `Vec<Lap>` or `Vec<User>` as
  placeholders — clubs, segments, routes, gear, segment efforts,
  athlete clubs all return the right thing now
- `activities().zones()` returns `Vec<ActivityZone>`, not `Vec<Zones>`
- `athlete().zones()` returns `Zones` (single), not `Vec<Zones>`
- GPX/TCX route exports return `String` via a new `get_raw` helper,
  instead of trying to parse XML as JSON
- `tcx()` and `gpx()` endpoint paths were swapped
- `segment_efforts` `get()` and `list()` paths were also swapped
- `post()` no longer hardcodes `API_URL`, so mock servers work
- `API_URL` was `.../api/v3`, but paths already carry `v3/` — duplicate dropped
- Every `panic!` in the HTTP + auth layer is gone; errors propagate
  through `Result<_, ErrorWrapper>` properly
- `Comment` fields are `pub` now, they weren't before (???)
- Single process-wide `reqwest::Client` via `OnceLock`; previously a
  fresh client + TLS handshake per call

### Added

- `ErrorWrapper` is now an enum (`Network` / `Parse` / `Api` / `Url`)
  with `Display` + `std::error::Error` impls
- Write endpoints (previously `todo!()` stubs):
  - `api.activities().create(body)` / `.update(id, body)`
  - `api.athlete().update(weight)`
  - `api.segments().star(id, starred)`
- Streams: `api.streams().activity() / .route() / .segment() /
  .segment_effort()` with a `StreamKey` enum
- Uploads: `api.uploads().upload(bytes, UploadDataType::Fit)` +
  `.get().id(upload_id)` to poll status
- `auth::refresh_token` and `auth::deauthorize`
- `_at` variants for all three OAuth fns (`get_token_at` etc.) that take
  a custom URL, mainly so integration tests can target httpmock
- `StravaAPI::set_token()` / `.token()` backed by `Arc<RwLock<String>>`,
  so refreshes are visible across clones
- `StravaAPI::rate_limit()` reads `X-RateLimit-Usage`/`-Limit` parsed off
  every response; `ErrorWrapper::Api` carries the snapshot on 429s
- `RateLimit` struct with `short_term_remaining()` /
  `long_term_remaining()` helpers
- `GearID` trait + derive — Strava gear IDs are strings like
  `"b12345678"`, not `u64`
- `strava_wrapper::prelude` with the common builder traits
- `#[must_use]` on every filter struct, so forgetting `.send()` warns
- 55 integration tests (was 21) — every endpoint, every `ErrorWrapper`
  variant, OAuth round-trip, rate-limit parsing, token rotation,
  pagination query-params

### Removed

- `User` struct (use `SummaryAthlete`)
- `EndPoint` trait (typo, collided with the real `Endpoint`)
- `Filter`, `TimeFilter`, `SimpleActivity`, `get_with_query` — all unused

### Refactored

- `models.rs` split into 11 submodules (`activity`, `athlete`, `club`,
  `segment`, `stream`, `zone`, etc.). Re-exports preserved, so
  `strava_wrapper::models::Activity` still imports from the flat path.
- Workspace dependencies centralised in the root `Cargo.toml`.

### Still missing

- Webhook push-subscriptions
- Automatic 401 → refresh interception (you get the pieces to do it
  manually; the wrapper doesn't retry for you)
- Model fields marked `Option<T>` that probably shouldn't be (and
  vice versa) — want to audit against real Strava responses rather
  than just the schema
