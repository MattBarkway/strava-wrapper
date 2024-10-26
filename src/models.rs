use serde::{Deserialize, Serialize};

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
    pub device_watts: bool,
    pub has_heartrate: bool,
    pub pr_count: i32,
    pub total_photo_count: i32,
    pub has_kudoed: bool,
    pub workout_type: Option<i32>,
    pub description: Option<String>,
    pub calories: Option<f64>,
    pub segment_efforts: Option<Vec<SegmentEffort>>,
}

#[derive(Serialize, Debug)]
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

#[derive(Debug, Deserialize)]
struct SimpleActivity {
    id: i64,
    resource_state: i32,
}

#[derive(Debug, Deserialize)]
pub struct Lap {
    id: i64,
    resource_state: i32,
    name: String,
    activity: SimpleActivity,
    athlete: SimpleAthlete,
    elapsed_time: i32,
    moving_time: i32,
    start_date: String,
    start_date_local: String,
    distance: f64,
    start_index: i32,
    end_index: i32,
    total_elevation_gain: f64,
    average_speed: f64,
    max_speed: f64,
    average_cadence: f64,
    device_watts: bool,
    average_watts: f64,
    lap_index: i32,
    split: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Zones {
    score: i32,
    sensor_based: bool,
    custom_zones: bool,
    max: i32,
    distribution_buckets: String,
    #[serde(rename = "type")]
    data_type: String,
    points: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleAthlete {
    pub id: i64,
    pub resource_state: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub id: String,
    pub polyline: Option<String>,
    pub resource_state: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentEffort {}

#[derive(Debug, Deserialize)]
pub struct Comment {
    id: i64,
    activity_id: i64,
    post_id: Option<i64>,
    resource_state: i32,
    text: String,
    mentions_metadata: Option<String>,
    created_at: String,
    athlete: User,
    cursor: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    firstname: String,
    lastname: String,
}