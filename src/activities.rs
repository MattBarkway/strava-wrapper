use crate::filters::{
    ActivityFilter, CommentFilter, CreateActivityFilter, GetActivity, KudosFilter,
};
use crate::models::{CreateActivity, Lap, Zones};
use crate::query;
use crate::query::ErrorWrapper;

pub struct ActivityEndpoints {
    token: String,
}

impl ActivityEndpoints {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl ActivityEndpoints {
    pub fn get(&self) -> GetActivity {
        GetActivity::new(self.token.to_string(), "activities/{id}".to_string())
    }
    pub fn create(&self, activity: CreateActivity) -> CreateActivityFilter {
        CreateActivityFilter::new(self.token.to_string(), "activities".to_string(), activity)
    }
    pub fn list(&self) -> ActivityFilter {
        ActivityFilter::new(self.token.to_string(), "/athlete/activities".to_string())
    }

    pub fn comments(&self) -> CommentFilter {
        CommentFilter::new(self.token.to_string(), "/activities/{id}/comments".to_string())
    }

    pub fn kudos(&self) -> KudosFilter {
        KudosFilter::new(self.token.to_string(), "/activities/{id}/kudos".to_string())
    }
    pub async fn laps(&self, token: &str, id: u64) -> Result<Vec<Lap>, ErrorWrapper> {
        query::get(&format!("activities/{}/laps", id), token).await
    }

    pub async fn zones(token: &str, id: u64) -> Result<Zones, ErrorWrapper> {
        query::get(&format!("/activities/{}/zones", id), token).await
    }
}

