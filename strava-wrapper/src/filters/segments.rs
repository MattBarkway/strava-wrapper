use crate::models::User;
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, PathQuery, Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ExploreSegments {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<User>> for ExploreSegments {
    async fn send(mut self) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
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
impl Sendable<Vec<User>> for ListStarredSegments {
    async fn send(mut self) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
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
impl Sendable<Vec<User>> for GetSegment {
    async fn send(mut self) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}
