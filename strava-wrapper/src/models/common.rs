//! Shared primitives used across multiple resource types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatLng(pub f64, pub f64);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub id: String,
    pub polyline: Option<String>,
    pub resource_state: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolylineMap {
    pub id: Option<String>,
    pub polyline: Option<String>,
    pub summary_polyline: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotosSummary {
    pub count: Option<i32>,
    pub primary: Option<PhotosSummaryPrimary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotosSummaryPrimary {
    pub id: Option<i64>,
    pub source: Option<i32>,
    pub unique_id: Option<String>,
    pub urls: Option<String>,
}

/// Generic Strava error detail (shape returned inside various response bodies).
/// Distinct from [`crate::query::ErrorWrapper`] — this is a serializable
/// field-level error, not the outer transport error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: Option<String>,
    pub field: Option<String>,
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fault {
    pub errors: Option<Vec<Error>>,
    pub message: Option<String>,
}
