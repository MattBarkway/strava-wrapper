//! Segment and segment-effort resource types.

use super::activity::MetaActivity;
use super::athlete::MetaAthlete;
use super::common::{LatLng, PolylineMap};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorerResponse {
    pub segments: Option<Vec<ExplorerSegment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorerSegment {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub climb_category: Option<i32>,
    pub climb_category_desc: Option<String>,
    pub avg_grade: Option<f32>,
    pub start_latlng: Option<LatLng>,
    pub end_latlng: Option<LatLng>,
    pub elev_difference: Option<f32>,
    pub distance: Option<f32>,
    pub points: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarySegment {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub activity_type: Option<String>,
    pub distance: Option<f32>,
    pub average_grade: Option<f32>,
    pub maximum_grade: Option<f32>,
    pub elevation_high: Option<f32>,
    pub elevation_low: Option<f32>,
    pub start_latlng: Option<LatLng>,
    pub end_latlng: Option<LatLng>,
    pub climb_category: Option<i32>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub private: Option<bool>,
    pub athlete_pr_effort: Option<SummaryPRSegmentEffort>,
    pub athlete_segment_stats: Option<SummarySegmentEffort>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedSegment {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub activity_type: Option<String>,
    pub distance: Option<f32>,
    pub average_grade: Option<f32>,
    pub maximum_grade: Option<f32>,
    pub elevation_high: Option<f32>,
    pub elevation_low: Option<f32>,
    pub start_latlng: Option<LatLng>,
    pub end_latlng: Option<LatLng>,
    pub climb_category: Option<i32>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub private: Option<bool>,
    pub athlete_pr_effort: Option<SummaryPRSegmentEffort>,
    pub athlete_segment_stats: Option<SummarySegmentEffort>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub total_elevation_gain: Option<f32>,
    pub map: Option<PolylineMap>,
    pub effort_count: Option<i32>,
    pub athlete_count: Option<i32>,
    pub hazardous: Option<bool>,
    pub star_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryPRSegmentEffort {
    pub pr_activity_id: Option<i64>,
    pub pr_elapsed_time: Option<i32>,
    pub pr_date: Option<DateTime<Utc>>,
    pub effort_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarySegmentEffort {
    pub id: Option<i64>,
    pub activity_id: Option<i64>,
    pub elapsed_time: Option<i32>,
    pub start_date: Option<DateTime<Utc>>,
    pub start_date_local: Option<DateTime<Utc>>,
    pub distance: Option<f32>,
    pub is_kom: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedSegmentEffort {
    pub id: Option<i64>,
    pub activity_id: Option<i64>,
    pub elapsed_time: Option<i32>,
    pub start_date: Option<DateTime<Utc>>,
    pub start_date_local: Option<DateTime<Utc>>,
    pub distance: Option<f32>,
    pub is_kom: Option<bool>,
    pub name: Option<String>,
    pub activity: Option<MetaActivity>,
    pub athlete: Option<MetaAthlete>,
    pub moving_time: Option<i32>,
    pub start_index: Option<i32>,
    pub end_index: Option<i32>,
    pub average_cadence: Option<f32>,
    pub average_watts: Option<f32>,
    pub device_watts: Option<bool>,
    pub average_heartrate: Option<f32>,
    pub max_heartrate: Option<f32>,
    pub segment: Option<SummarySegment>,
    pub kom_rank: Option<i32>,
    pub pr_rank: Option<i32>,
    pub hidden: Option<bool>,
}
