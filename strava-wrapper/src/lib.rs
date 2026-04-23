pub mod api;
pub mod auth;
pub mod endpoints;
pub mod filters;
pub mod models;
pub mod query;

/// Re-exports of the builder traits commonly imported from user code.
///
/// `use strava_wrapper::prelude::*;` brings `Sendable` plus the typed
/// builder traits (`ID`, `Page`, `PerPage`, …) into scope so that
/// `api.activities().get().id(123).send().await` compiles without a dozen
/// individual imports.
pub mod prelude {
    pub use crate::query::{
        After, AfterCursor, Before, Endpoint, ErrorWrapper, GearID, IncludeAllEfforts, Page,
        PageSize, PathQuery, PerPage, Query, RateLimit, Sendable, ID,
    };
}
