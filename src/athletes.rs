use crate::models::Activity;
use crate::query;
use crate::query::ErrorWrapper;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Athlete {
    pub id: i64,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub sex: String,
    pub premium: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DataPoint {
    max: i32,
    min: i32,
    time: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    distribution_buckets: Vec<DataPoint>,
    r#type: String,
    resource_state: i32,
    sensor_based: bool,
}

pub async fn get(token: &str) -> Result<Athlete, ErrorWrapper> {
    query::get("athlete", token).await
}

pub async fn activities(token: &str) -> Result<Vec<Activity>, ErrorWrapper> {
    query::get("athlete/activities", token).await
}

pub async fn zones(token: &str) -> Result<Vec<Zone>, ErrorWrapper> {
    query::get("athlete/zones", token).await
}

pub async fn stats(token: &str, id: &str) -> Result<Vec<Zone>, ErrorWrapper> {
    query::get(&format!("/athletes/{}/stats", id), token).await
}

pub async fn update() {
    // TODO query::put
}

pub async fn clubs(token: &str) -> Result<Vec<Zone>, ErrorWrapper> {
    query::get("/athlete/clubs", token).await
}
