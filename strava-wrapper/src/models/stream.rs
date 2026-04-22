//! Activity / segment / route stream types.

use super::common::LatLng;
use serde::{Deserialize, Serialize};

/// The `key_by_type=true` shape of a stream response: every channel is
/// optional, only those requested via `keys=...` will be populated.
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
pub struct BaseStream {
    pub original_size: Option<i32>,
    pub resolution: Option<String>,
    pub series_type: Option<String>,
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
