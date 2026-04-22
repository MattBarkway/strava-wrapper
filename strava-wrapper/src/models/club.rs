//! Club resource types.

use super::activity::{ActivityType, SportType};
use super::athlete::MetaAthlete;
use serde::{Deserialize, Serialize};

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
pub struct MetaClub {
    pub id: Option<i64>,
    pub resource_state: Option<i32>,
    pub name: Option<String>,
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
