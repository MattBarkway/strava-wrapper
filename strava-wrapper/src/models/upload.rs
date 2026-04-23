//! Upload resource types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upload {
    pub id: Option<i64>,
    pub id_str: Option<String>,
    pub external_id: Option<String>,
    pub error: Option<String>,
    pub status: Option<String>,
    pub activity_id: Option<i64>,
}
