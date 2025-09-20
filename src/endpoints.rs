use crate::filters::{ActivityFilter, CommentFilter, CreateActivityFilter, GetActivity, KudosFilter};
use crate::models::CreateActivity;

pub struct CommentsEndpoint {
    token: String,
}

impl CommentsEndpoint {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }
    pub fn get(&self) -> CommentFilter {
        CommentFilter::new(self.token.clone(), "/activities/{id}/comments".into())
    }
}

pub struct ActivitiesEndpoint {
    token: String,
}

impl ActivitiesEndpoint {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }
    pub fn list(&self) -> ActivityFilter {
        ActivityFilter::new(self.token.clone(), "/athlete/activities".into())
    }

    pub fn get(&self) -> GetActivity {
        GetActivity::new(self.token.clone(), "/activities/{id}".into())
    }

    pub fn create(&self, payload: CreateActivity) -> CreateActivityFilter {
        CreateActivityFilter::new(self.token.clone(), "/activities".into(), payload)
    }
}

pub struct KudosEndpoint {
    token: String,
}

impl KudosEndpoint {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }
    pub fn get(&self) -> KudosFilter {
        KudosFilter::new(self.token.clone(), "/activities/{id}/kudos".into())
    }
}