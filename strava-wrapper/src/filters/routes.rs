use crate::models::Route;
use crate::query::{
    get_raw_with_query_and_path, get_with_query_and_path, Endpoint, ErrorWrapper, Page, PathQuery,
    PerPage, Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, Page, PathQuery, PerPage, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetRoute {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Route> for GetRoute {
    async fn send(self) -> Result<Route, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, Page, PerPage)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct ListAthleteRoutes {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<Route>> for ListAthleteRoutes {
    async fn send(self) -> Result<Vec<Route>, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct ExportTCXRoute {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<String> for ExportTCXRoute {
    async fn send(self) -> Result<String, ErrorWrapper> {
        let token = self.token.clone();
        get_raw_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct ExportGPXRoute {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<String> for ExportGPXRoute {
    async fn send(self) -> Result<String, ErrorWrapper> {
        let token = self.token.clone();
        get_raw_with_query_and_path(self, &token).await
    }
}
