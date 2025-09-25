use crate::models::User;
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, PathQuery, Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct GetRoute {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<User>> for GetRoute {
    async fn send(mut self) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ListAthleteRoutes {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<User>> for ListAthleteRoutes {
    async fn send(mut self) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ExportTCXRoute {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<User>> for ExportTCXRoute {
    async fn send(mut self) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ExportGPXRoute {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<User>> for ExportGPXRoute {
    async fn send(mut self) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}
