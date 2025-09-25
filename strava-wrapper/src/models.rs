use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityTotal {
    pub count: Option<i32>,
    pub distance: Option<f32>,
    pub moving_time: Option<i32>,
    pub elapsed_time: Option<i32>,
    pub elevation_gain: Option<f32>,
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
pub struct ActivityZone {
    pub score: Option<i32>,
    pub distribution_buckets: Option<Vec<TimedZoneDistribution>>,
    #[serde(rename = "type")]
    pub zone_type: Option<String>,
    pub sensor_based: Option<bool>,
    pub points: Option<i32>,
    pub custom_zones: Option<bool>,
    pub max: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubActivity {
    pub athlete: Option<MetaAthlete>,
    pub name: Option<String>,
    pub distance: Option<f32>,
    pub moving_time: Option<i32>,
    pub elapsed_time: Option<i32>,
    pub total_elevation_gain: Option<f32>,
    #[deprecated(note = "Use `sport_type` instead")]
    #[serde(default)]
    pub r#type: Option<ActivityType>,
    pub sport_type: Option<SportType>,
    pub workout_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubAthlete {
    pub resource_state: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub member: Option<String>,
    pub admin: Option<bool>,
    pub owner: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleComment {
    pub id: Option<i64>,
    pub activity_id: Option<i64>,
    pub text: Option<String>,
    pub athlete: Option<SummaryAthlete>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: Option<String>,
    pub field: Option<String>,
    pub resource: Option<String>,
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct Comment {
    id: i64,
    activity_id: i64,
    post_id: Option<i64>,
    resource_state: i32,
    text: String,
    mentions_metadata: Option<String>,
    created_at: String,
    athlete: User,
    cursor: Option<String>,
    reaction_count: i32,
    has_reacted: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct User {
    firstname: String,
    lastname: String,
    resource_state: Option<i32>,
}

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
pub struct Fault {
    pub errors: Option<Vec<Error>>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRateZoneRanges {
    pub custom_zones: Option<bool>,
    pub zones: Option<ZoneRanges>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatLng(pub f64, pub f64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaActivity {
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaAthlete {
    pub id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaClub {
    pub id: Option<i64>,
    pub resource_state: Option<i32>,
    pub name: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolylineMap {
    pub id: Option<String>,
    pub polyline: Option<String>,
    pub summary_polyline: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerZoneRanges {
    pub zones: Option<ZoneRanges>,
}

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
#[serde(rename_all = "PascalCase")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSet {
    pub time: Option<TimeStream>,
    pub distance: Option<DistanceStream>,
    pub latlng: Option<LatLngStream>,
    pub altitude: Option<AltitudeStream>,
    pub velocity_smooth: Option<SmoothVelocityStream>,
    pub heartrate: Option<HeartrateStream>,
    pub cadence: Option<CadenceStream>,
    pub watts: Option<PowerStream>,
    pub temp: Option<TemperatureStream>,
    pub moving: Option<MovingStream>,
    pub grade_smooth: Option<SmoothGradeStream>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryGear {
    pub id: Option<String>,
    pub resource_state: Option<i32>,
    pub primary: Option<bool>,
    pub name: Option<String>,
    pub distance: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryPRSegmentEffort {
    pub pr_activity_id: Option<i64>,
    pub pr_elapsed_time: Option<i32>,
    pub pr_date: Option<DateTime<Utc>>,
    pub effort_count: Option<i32>,
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
pub struct TimedZoneDistribution {
    pub ranges: Option<Vec<TimedZoneRange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct Upload {
    pub id: Option<i64>,
    pub id_str: Option<String>,
    pub external_id: Option<String>,
    pub error: Option<String>,
    pub status: Option<String>,
    pub activity_id: Option<i64>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneRange {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneRanges {
    pub zones: Option<Vec<ZoneRange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zones {
    pub heart_rate: Option<HeartRateZoneRanges>,
    pub power: Option<PowerZoneRanges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AltitudeStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedGear {
    pub id: Option<String>,
    pub resource_state: Option<i32>,
    pub primary: Option<bool>,
    pub name: Option<String>,
    pub distance: Option<f32>,
    pub brand_name: Option<String>,
    pub model_name: Option<String>,
    pub frame_type: Option<i32>,
    pub description: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartrateStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatLngStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<LatLng>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmoothGradeStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmoothVelocityStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<f32>>,
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
pub struct SummaryAthlete {
    pub id: Option<i64>,
    pub resource_state: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub profile_medium: Option<String>,
    pub profile: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub sex: Option<String>,
    #[deprecated(note = "Use summit instead")]
    pub premium: Option<bool>,
    pub summit: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryClub {
    pub id: Option<i64>,
    pub resource_state: Option<i32>,
    pub name: Option<String>,
    pub profile_medium: Option<String>,
    pub cover_photo: Option<String>,
    pub cover_photo_small: Option<String>,
    #[deprecated(note = "Use activity_types instead")]
    pub sport_type: Option<String>,
    pub activity_types: Option<Vec<ActivityType>>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub private: Option<bool>,
    pub member_count: Option<i32>,
    pub featured: Option<bool>,
    pub verified: Option<bool>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
    pub data: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedZoneRange {
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub time: Option<i32>,
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
pub struct DetailedAthlete {
    pub id: Option<i64>,
    pub resource_state: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub profile_medium: Option<String>,
    pub profile: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub sex: Option<String>,
    #[deprecated(note = "Use summit instead")]
    pub premium: Option<bool>,
    pub summit: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub follower_count: Option<i32>,
    pub friend_count: Option<i32>,
    pub measurement_preference: Option<String>,
    pub ftp: Option<i32>,
    pub weight: Option<f32>,
    pub clubs: Option<Vec<SummaryClub>>,
    pub bikes: Option<Vec<SummaryGear>>,
    pub shoes: Option<Vec<SummaryGear>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedClub {
    pub id: Option<i64>,
    pub resource_state: Option<i32>,
    pub name: Option<String>,
    pub profile_medium: Option<String>,
    pub cover_photo: Option<String>,
    pub cover_photo_small: Option<String>,
    #[deprecated(note = "Use activity_types instead")]
    pub sport_type: Option<String>,
    pub activity_types: Option<Vec<ActivityType>>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub private: Option<bool>,
    pub member_count: Option<i32>,
    pub featured: Option<bool>,
    pub verified: Option<bool>,
    pub url: Option<String>,
    pub membership: Option<String>,
    pub admin: Option<bool>,
    pub owner: Option<bool>,
    pub following_count: Option<i32>,
}
