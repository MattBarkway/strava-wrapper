//! Activity resource types plus the `ActivityType` / `SportType` enums.

use super::athlete::{MetaAthlete, SimpleAthlete};
use super::common::{LatLng, Map, PhotosSummary, PolylineMap};
use super::gear::SummaryGear;
use super::segment::DetailedSegmentEffort;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityTotal {
    pub count: Option<i32>,
    pub distance: Option<f64>,
    pub moving_time: Option<f64>,
    pub elapsed_time: Option<f64>,
    pub elevation_gain: Option<f64>,
    pub achievement_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityStats {
    pub biggest_ride_distance: Option<f64>,
    pub biggest_climb_elevation_gain: Option<f64>,
    pub recent_ride_totals: Option<ActivityTotal>,
    pub recent_run_totals: Option<ActivityTotal>,
    pub recent_swim_totals: Option<ActivityTotal>,
    pub ytd_ride_totals: Option<ActivityTotal>,
    pub ytd_run_totals: Option<ActivityTotal>,
    pub ytd_swim_totals: Option<ActivityTotal>,
    pub all_ride_totals: Option<ActivityTotal>,
    pub all_run_totals: Option<ActivityTotal>,
    pub all_swim_totals: Option<ActivityTotal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum ActivityType {
    AlpineSki,
    BackcountrySki,
    Canoeing,
    Crossfit,
    EBikeRide,
    Elliptical,
    Golf,
    Handcycle,
    Hike,
    IceSkate,
    InlineSkate,
    Kayaking,
    Kitesurf,
    NordicSki,
    Ride,
    RockClimbing,
    RollerSki,
    Rowing,
    Run,
    Sail,
    Skateboard,
    Snowboard,
    Snowshoe,
    Soccer,
    StairStepper,
    StandUpPaddling,
    Surfing,
    Swim,
    Velomobile,
    VirtualRide,
    VirtualRun,
    Walk,
    WeightTraining,
    Wheelchair,
    Windsurf,
    Workout,
    Yoga,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum SportType {
    AlpineSki,
    BackcountrySki,
    Badminton,
    Canoeing,
    Crossfit,
    EBikeRide,
    Elliptical,
    EMountainBikeRide,
    Golf,
    GravelRide,
    Handcycle,
    HighIntensityIntervalTraining,
    Hike,
    IceSkate,
    InlineSkate,
    Kayaking,
    Kitesurf,
    MountainBikeRide,
    NordicSki,
    Pickleball,
    Pilates,
    Racquetball,
    Ride,
    RockClimbing,
    RollerSki,
    Rowing,
    Run,
    Sail,
    Skateboard,
    Snowboard,
    Snowshoe,
    Soccer,
    Squash,
    StairStepper,
    StandUpPaddling,
    Surfing,
    Swim,
    TableTennis,
    Tennis,
    TrailRun,
    Velomobile,
    VirtualRide,
    VirtualRow,
    VirtualRun,
    Walk,
    WeightTraining,
    Wheelchair,
    Windsurf,
    Workout,
    Yoga,
    #[serde(other)]
    Unknown,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    pub id: i64,
    pub resource_state: u8,
    pub external_id: Option<String>,
    pub upload_id: Option<i64>,
    pub athlete: SimpleAthlete,
    pub name: String,
    pub distance: f64,
    pub moving_time: i32,
    pub elapsed_time: i32,
    pub total_elevation_gain: f64,
    #[serde(rename = "type")]
    pub activity_type: String,
    pub sport_type: String,
    pub start_date: String,
    pub start_date_local: String,
    pub timezone: String,
    pub utc_offset: f64,
    pub achievement_count: i32,
    pub kudos_count: i32,
    pub comment_count: i32,
    pub athlete_count: i32,
    pub photo_count: i32,
    pub map: Map,
    pub trainer: bool,
    pub commute: bool,
    pub manual: bool,
    pub private: bool,
    pub flagged: bool,
    pub gear_id: Option<String>,
    pub from_accepted_tag: Option<bool>,
    pub average_speed: f64,
    pub max_speed: f64,
    pub device_watts: Option<bool>,
    pub has_heartrate: bool,
    pub pr_count: i32,
    pub total_photo_count: i32,
    pub has_kudoed: bool,
    pub workout_type: Option<i32>,
    pub description: Option<String>,
    pub calories: Option<f64>,
    pub segment_efforts: Option<Vec<SegmentEffort>>,
}

/// Placeholder held by [`Activity::segment_efforts`]. The richer
/// [`DetailedSegmentEffort`](super::segment::DetailedSegmentEffort) is used
/// elsewhere; the shape here is intentionally empty because this field is
/// always returned empty or absent on the `GET /activities/{id}` endpoint
/// unless `include_all_efforts=true` is set.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentEffort {}

#[derive(Debug, Clone, Serialize)]
pub struct CreateActivity {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
    pub sport_type: String,
    pub start_date_local: String,
    pub elapsed_time: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commute: Option<i32>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UpdatableActivity {
    pub commute: Option<bool>,
    pub trainer: Option<bool>,
    pub hide_from_home: Option<bool>,
    pub description: Option<String>,
    pub name: Option<String>,
    #[deprecated(note = "Use sport_type instead")]
    #[serde(default)]
    pub r#type: Option<ActivityType>,
    pub sport_type: Option<SportType>,
    pub gear_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryActivity {
    pub id: Option<i64>,
    pub external_id: Option<String>,
    pub upload_id: Option<i64>,
    pub athlete: Option<MetaAthlete>,
    pub name: Option<String>,
    pub distance: Option<f32>,
    pub moving_time: Option<i32>,
    pub elapsed_time: Option<i32>,
    pub total_elevation_gain: Option<f32>,
    pub elev_high: Option<f32>,
    pub elev_low: Option<f32>,
    #[deprecated(note = "Use sport_type instead")]
    #[serde(default)]
    pub r#type: Option<ActivityType>,
    pub sport_type: Option<SportType>,
    pub start_date: Option<DateTime<Utc>>,
    pub start_date_local: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub start_latlng: Option<LatLng>,
    pub end_latlng: Option<LatLng>,
    pub achievement_count: Option<i32>,
    pub kudos_count: Option<i32>,
    pub comment_count: Option<i32>,
    pub athlete_count: Option<i32>,
    pub photo_count: Option<i32>,
    pub total_photo_count: Option<i32>,
    pub map: Option<PolylineMap>,
    pub trainer: Option<bool>,
    pub commute: Option<bool>,
    pub manual: Option<bool>,
    pub private: Option<bool>,
    pub flagged: Option<bool>,
    pub workout_type: Option<i32>,
    pub upload_id_str: Option<String>,
    pub average_speed: Option<f32>,
    pub max_speed: Option<f32>,
    pub has_kudoed: Option<bool>,
    pub hide_from_home: Option<bool>,
    pub gear_id: Option<String>,
    pub kilojoules: Option<f32>,
    pub average_watts: Option<f32>,
    pub device_watts: Option<bool>,
    pub max_watts: Option<i32>,
    pub weighted_average_watts: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedActivity {
    pub id: Option<i64>,
    pub external_id: Option<String>,
    pub upload_id: Option<i64>,
    pub athlete: Option<MetaAthlete>,
    pub name: Option<String>,
    pub distance: Option<f32>,
    pub moving_time: Option<i32>,
    pub elapsed_time: Option<i32>,
    pub total_elevation_gain: Option<f32>,
    pub elev_high: Option<f32>,
    pub elev_low: Option<f32>,
    #[deprecated(note = "Use sport_type instead")]
    #[serde(default)]
    pub r#type: Option<ActivityType>,
    pub sport_type: Option<SportType>,
    pub start_date: Option<DateTime<Utc>>,
    pub start_date_local: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub start_latlng: Option<LatLng>,
    pub end_latlng: Option<LatLng>,
    pub achievement_count: Option<i32>,
    pub kudos_count: Option<i32>,
    pub comment_count: Option<i32>,
    pub athlete_count: Option<i32>,
    pub photo_count: Option<i32>,
    pub total_photo_count: Option<i32>,
    pub map: Option<PolylineMap>,
    pub trainer: Option<bool>,
    pub commute: Option<bool>,
    pub manual: Option<bool>,
    pub private: Option<bool>,
    pub flagged: Option<bool>,
    pub workout_type: Option<i32>,
    pub upload_id_str: Option<String>,
    pub average_speed: Option<f32>,
    pub max_speed: Option<f32>,
    pub has_kudoed: Option<bool>,
    pub hide_from_home: Option<bool>,
    pub gear_id: Option<String>,
    pub kilojoules: Option<f32>,
    pub average_watts: Option<f32>,
    pub device_watts: Option<bool>,
    pub max_watts: Option<i32>,
    pub weighted_average_watts: Option<i32>,
    pub description: Option<String>,
    pub photos: Option<PhotosSummary>,
    pub gear: Option<SummaryGear>,
    pub calories: Option<f32>,
    pub segment_efforts: Option<Vec<DetailedSegmentEffort>>,
    pub device_name: Option<String>,
    pub embed_token: Option<String>,
    pub splits_metric: Option<Vec<Split>>,
    pub splits_standard: Option<Vec<Split>>,
    pub laps: Option<Vec<Lap>>,
    pub best_efforts: Option<Vec<DetailedSegmentEffort>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaActivity {
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Split {
    pub average_speed: Option<f32>,
    pub distance: Option<f32>,
    pub elapsed_time: Option<i32>,
    pub elevation_difference: Option<f32>,
    pub pace_zone: Option<i32>,
    pub moving_time: Option<i32>,
    pub split: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lap {
    pub id: Option<i64>,
    pub activity: Option<MetaActivity>,
    pub athlete: Option<MetaAthlete>,
    pub average_cadence: Option<f32>,
    pub average_speed: Option<f32>,
    pub distance: Option<f32>,
    pub elapsed_time: Option<i32>,
    pub start_index: Option<i32>,
    pub end_index: Option<i32>,
    pub lap_index: Option<i32>,
    pub max_speed: Option<f32>,
    pub moving_time: Option<i32>,
    pub name: Option<String>,
    pub pace_zone: Option<i32>,
    pub split: Option<i32>,
    pub start_date: Option<DateTime<Utc>>,
    pub start_date_local: Option<DateTime<Utc>>,
    pub total_elevation_gain: Option<f32>,
}
