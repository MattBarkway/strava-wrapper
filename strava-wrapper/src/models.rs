//! Serde-derived types mirroring Strava's API responses and request bodies.
//!
//! Types are grouped by resource (activity, athlete, club, …) in dedicated
//! submodules and flat-re-exported from this module so that existing imports
//! like `strava_wrapper::models::Activity` continue to work.

pub mod activity;
pub mod athlete;
pub mod club;
pub mod comment;
pub mod common;
pub mod gear;
pub mod route;
pub mod segment;
pub mod stream;
pub mod upload;
pub mod zone;

pub use activity::*;
pub use athlete::*;
pub use club::*;
pub use comment::*;
pub use common::*;
pub use gear::*;
pub use route::*;
pub use segment::*;
pub use stream::*;
pub use upload::*;
pub use zone::*;
