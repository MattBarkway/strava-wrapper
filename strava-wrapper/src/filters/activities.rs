use crate::models::{Activity, CreateActivity, DetailedActivity, UpdatableActivity};
use crate::query::{
    get_with_query_and_path, post_form, put_json, Endpoint, ErrorWrapper, IncludeAllEfforts,
    PathQuery, Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, IncludeAllEfforts, PathQuery, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, IncludeAllEfforts)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetActivity {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Activity> for GetActivity {
    async fn send(self) -> Result<Activity, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

/// Builder for `POST /activities`. Construct a [`CreateActivity`] body and
/// call `.send()` to create a manual activity.
#[must_use = "this request is not executed until you call .send().await"]
pub struct CreateActivityRequest {
    url: String,
    token: String,
    body: CreateActivity,
}

impl CreateActivityRequest {
    pub fn new(url: impl Into<String>, token: impl Into<String>, body: CreateActivity) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
            body,
        }
    }
}

#[async_trait]
impl Sendable<DetailedActivity> for CreateActivityRequest {
    async fn send(self) -> Result<DetailedActivity, ErrorWrapper> {
        let url = format!("{}/v3/activities", self.url);
        post_form(&url, &self.token, &self.body).await
    }
}

/// Builder for `PUT /activities/{id}`. Takes the activity id and an
/// [`UpdatableActivity`] body describing the fields to change.
#[must_use = "this request is not executed until you call .send().await"]
pub struct UpdateActivityRequest {
    url: String,
    token: String,
    id: u64,
    body: UpdatableActivity,
}

impl UpdateActivityRequest {
    pub fn new(
        url: impl Into<String>,
        token: impl Into<String>,
        id: u64,
        body: UpdatableActivity,
    ) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
            id,
            body,
        }
    }
}

#[async_trait]
impl Sendable<DetailedActivity> for UpdateActivityRequest {
    async fn send(self) -> Result<DetailedActivity, ErrorWrapper> {
        let url = format!("{}/v3/activities/{}", self.url, self.id);
        put_json(&url, &self.token, &self.body).await
    }
}
