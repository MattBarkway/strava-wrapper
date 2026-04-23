use crate::models::StreamSet;
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, PathQuery, Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query, ID};

/// Identifiers for the `keys` query parameter on Strava stream endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum StreamKey {
    Time,
    Distance,
    Latlng,
    Altitude,
    VelocitySmooth,
    Heartrate,
    Cadence,
    Watts,
    Temp,
    Moving,
    GradeSmooth,
}

impl StreamKey {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Time => "time",
            Self::Distance => "distance",
            Self::Latlng => "latlng",
            Self::Altitude => "altitude",
            Self::VelocitySmooth => "velocity_smooth",
            Self::Heartrate => "heartrate",
            Self::Cadence => "cadence",
            Self::Watts => "watts",
            Self::Temp => "temp",
            Self::Moving => "moving",
            Self::GradeSmooth => "grade_smooth",
        }
    }
}

fn push_keys(query: &mut Vec<(String, String)>, keys: &[StreamKey]) {
    let joined = keys
        .iter()
        .map(|k| k.as_str())
        .collect::<Vec<_>>()
        .join(",");
    query.push(("keys".to_string(), joined));
    query.push(("key_by_type".to_string(), "true".to_string()));
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetActivityStreams {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl GetActivityStreams {
    /// Required. Set the stream types to fetch.
    pub fn keys(mut self, keys: &[StreamKey]) -> Self {
        push_keys(&mut self.query, keys);
        self
    }
}

#[async_trait]
impl Sendable<StreamSet> for GetActivityStreams {
    async fn send(self) -> Result<StreamSet, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetRouteStreams {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<StreamSet> for GetRouteStreams {
    async fn send(self) -> Result<StreamSet, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetSegmentEffortStreams {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl GetSegmentEffortStreams {
    pub fn keys(mut self, keys: &[StreamKey]) -> Self {
        push_keys(&mut self.query, keys);
        self
    }
}

#[async_trait]
impl Sendable<StreamSet> for GetSegmentEffortStreams {
    async fn send(self) -> Result<StreamSet, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetSegmentStreams {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl GetSegmentStreams {
    pub fn keys(mut self, keys: &[StreamKey]) -> Self {
        push_keys(&mut self.query, keys);
        self
    }
}

#[async_trait]
impl Sendable<StreamSet> for GetSegmentStreams {
    async fn send(self) -> Result<StreamSet, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}
