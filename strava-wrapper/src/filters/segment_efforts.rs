use crate::models::DetailedSegmentEffort;
use crate::query::{
    get_with_query_and_path, After, Before, Endpoint, ErrorWrapper, Page, PathQuery, PerPage,
    Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{After, Before, Endpoint, Page, PathQuery, PerPage, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetSegmentEffort {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<DetailedSegmentEffort> for GetSegmentEffort {
    async fn send(self) -> Result<DetailedSegmentEffort, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, Before, After, Page, PerPage)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct ListSegmentEfforts {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl ListSegmentEfforts {
    /// Required: the identifier of the segment to list efforts for.
    pub fn segment_id(mut self, id: u64) -> Self {
        self.query.push(("segment_id".to_string(), id.to_string()));
        self
    }
}

#[async_trait]
impl Sendable<Vec<DetailedSegmentEffort>> for ListSegmentEfforts {
    async fn send(self) -> Result<Vec<DetailedSegmentEffort>, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}
