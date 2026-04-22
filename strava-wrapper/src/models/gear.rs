//! Gear resource types (bikes, shoes).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryGear {
    pub id: Option<String>,
    pub resource_state: Option<i32>,
    pub primary: Option<bool>,
    pub name: Option<String>,
    pub distance: Option<f32>,
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
