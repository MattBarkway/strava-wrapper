use crate::query;
use crate::query::ErrorWrapper;
use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SimpleMember {
    resource_state: i32,
    firstname: String,
    lastname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Activity {
    resource_state: i32,
    athlete: SimpleMember,
    name: String,
    distance: f64,
    moving_time: i32,
    elapsed_time: i32,
    total_elevation_gain: f64,
    r#type: String,
    sport_type: String,
    workout_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Member {
    resource_state: i32,
    firstname: String,
    lastname: String,
    membership: String,
    admin: bool,
    owner: bool,
}
pub async fn get(token: &str, id: &str) -> Result<Vec<Activity>, ErrorWrapper> {
    query::get(&format!("clubs/{}", id), token).await
}
pub async fn list_activities(token: &str, id: &str) -> Result<Vec<Activity>, ErrorWrapper> {
    query::get(&format!("clubs/{}/activities", id), token).await
}
pub async fn list_admins(token: &str, id: &str) -> Result<Vec<SimpleMember>, ErrorWrapper> {
    query::get(&format!("clubs/{}/admins", id), token).await
}
pub async fn list_members(token: &str, id: &str) -> Result<Vec<Member>, ErrorWrapper> {
    query::get(&format!("clubs/{}/activities", id), token).await
}
