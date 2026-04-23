//! Zone / heart-rate / power range types and activity-zones data.

use serde::{Deserialize, Serialize};

/// Time-in-zone breakdown returned from `GET /activities/{id}/zones`.
/// One entry per zone type (heart rate, pace, etc.) that the activity has data for.
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

/// The athlete's own configured zones, returned from `GET /athlete/zones`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zones {
    pub heart_rate: Option<HeartRateZoneRanges>,
    pub power: Option<PowerZoneRanges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRateZoneRanges {
    pub custom_zones: Option<bool>,
    pub zones: Option<ZoneRanges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerZoneRanges {
    pub zones: Option<ZoneRanges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneRanges {
    pub zones: Option<Vec<ZoneRange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneRange {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedZoneDistribution {
    pub ranges: Option<Vec<TimedZoneRange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedZoneRange {
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub time: Option<i32>,
}
