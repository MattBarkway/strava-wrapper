//! Route resource types.

use super::athlete::SummaryAthlete;
use super::common::{LatLng, PolylineMap};
use super::segment::SummarySegment;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub athlete: Option<SummaryAthlete>,
    pub description: Option<String>,
    pub distance: Option<f32>,
    pub elevation_gain: Option<f32>,
    pub id: Option<i64>,
    pub id_str: Option<String>,
    pub map: Option<PolylineMap>,
    pub name: Option<String>,
    pub private: Option<bool>,
    pub starred: Option<bool>,
    pub timestamp: Option<i64>,
    pub r#type: Option<i32>,
    pub sub_type: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub estimated_moving_time: Option<i32>,
    pub segments: Option<Vec<SummarySegment>>,
    pub waypoints: Option<Vec<Waypoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    pub latlng: Option<LatLng>,
    pub target_latlng: Option<LatLng>,
    pub categories: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub distance_into_route: Option<i32>,
}
