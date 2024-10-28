use crate::filters::{ActivityFilter, CommentFilter, CreateActivityFilter, GetActivity, KudosFilter};
use crate::query;
use crate::query::{ErrorWrapper};
use crate::models::{CreateActivity, Lap, Zones};

pub fn get() -> GetActivity {
    GetActivity { path: "activities/{id}".to_string(), path_params: Vec::new(), query: Vec::new() }
}

pub fn create(activity: CreateActivity) -> CreateActivityFilter {
    CreateActivityFilter {
        path: "activities".to_string(),
        payload: activity,
    }
}

pub fn list() -> ActivityFilter {
    ActivityFilter {
        path: "/athlete/activities".to_string(),
        query: Vec::new(),
    }
}

pub fn comments() -> CommentFilter {
    CommentFilter {
        path: "/activities/{id}/comments".to_string(),
        path_params: Vec::new(),
        query: Vec::new(),
    }
}

pub fn kudos() -> KudosFilter {
    KudosFilter {
        path: "/activities/{id}/kudos".to_string(),
        path_params: Vec::new(),
        query: Vec::new(),
    }
}

pub async fn laps(token: &str, id: u64) -> Result<Vec<Lap>, ErrorWrapper> {
    query::get(&format!("activities/{}/laps", id), token).await
}

pub async fn zones(token: &str, id: u64) -> Result<Zones, ErrorWrapper> {
    query::get(&format!("/activities/{}/zones", id), token).await
}
