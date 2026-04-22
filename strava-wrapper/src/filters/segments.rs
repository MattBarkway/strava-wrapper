use crate::models::{DetailedSegment, ExplorerResponse, SummarySegment};
use crate::query::{
    get_with_query_and_path, put_json, Endpoint, ErrorWrapper, PathQuery, Query, Sendable, ID,
};
use async_trait::async_trait;
use serde::Serialize;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery)]
pub struct ExploreSegments {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl ExploreSegments {
    /// Required. A bounding box expressed as `[sw_lat, sw_lng, ne_lat, ne_lng]`.
    pub fn bounds(mut self, sw_lat: f64, sw_lng: f64, ne_lat: f64, ne_lng: f64) -> Self {
        self.query.push((
            "bounds".to_string(),
            format!("{},{},{},{}", sw_lat, sw_lng, ne_lat, ne_lng),
        ));
        self
    }

    /// Filter by "running" or "riding".
    pub fn activity_type(mut self, activity_type: &str) -> Self {
        self.query
            .push(("activity_type".to_string(), activity_type.to_string()));
        self
    }

    pub fn min_cat(mut self, cat: u32) -> Self {
        self.query.push(("min_cat".to_string(), cat.to_string()));
        self
    }

    pub fn max_cat(mut self, cat: u32) -> Self {
        self.query.push(("max_cat".to_string(), cat.to_string()));
        self
    }
}

#[async_trait]
impl Sendable<ExplorerResponse> for ExploreSegments {
    async fn send(self) -> Result<ExplorerResponse, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ListStarredSegments {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<SummarySegment>> for ListStarredSegments {
    async fn send(self) -> Result<Vec<SummarySegment>, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct GetSegment {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<DetailedSegment> for GetSegment {
    async fn send(self) -> Result<DetailedSegment, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Serialize)]
struct StarSegmentBody {
    starred: bool,
}

/// Builder for `PUT /segments/{id}/starred`. `.send()` stars or unstars
/// the segment based on the `starred` flag.
pub struct StarSegment {
    url: String,
    token: String,
    id: u64,
    starred: bool,
}

impl StarSegment {
    pub fn new(url: impl Into<String>, token: impl Into<String>, id: u64, starred: bool) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
            id,
            starred,
        }
    }
}

#[async_trait]
impl Sendable<DetailedSegment> for StarSegment {
    async fn send(self) -> Result<DetailedSegment, ErrorWrapper> {
        let url = format!("{}/v3/segments/{}/starred", self.url, self.id);
        put_json(
            &url,
            &self.token,
            &StarSegmentBody {
                starred: self.starred,
            },
        )
        .await
    }
}
