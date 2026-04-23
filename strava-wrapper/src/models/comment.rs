//! Activity-comment resource types.

use super::athlete::SummaryAthlete;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleComment {
    pub id: Option<i64>,
    pub activity_id: Option<i64>,
    pub text: Option<String>,
    pub athlete: Option<SummaryAthlete>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Comment {
    pub id: i64,
    pub activity_id: i64,
    pub post_id: Option<i64>,
    pub resource_state: i32,
    pub text: String,
    pub mentions_metadata: Option<String>,
    pub created_at: String,
    pub athlete: SummaryAthlete,
    pub cursor: Option<String>,
    pub reaction_count: i32,
    pub has_reacted: bool,
}
