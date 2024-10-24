use crate::query;
use crate::query::ErrorWrapper;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Gear {
    id: String,
    resource_state: i32,
    distance: i32,
    brand_name: String,
    model_name: String,
    frame_type: i32,
    description: String,
}

pub async fn get(token: &str, id: &str) -> Result<Gear, ErrorWrapper> {
    query::get(&format!("gear/{}", id), token).await
}
