//! Athlete resource types.

use super::club::SummaryClub;
use super::gear::SummaryGear;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleAthlete {
    pub id: i64,
    pub resource_state: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaAthlete {
    pub id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
