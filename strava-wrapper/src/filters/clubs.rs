use crate::models::Lap;
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, Page, PathQuery, PerPage, Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, Page, PathQuery, PerPage, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, Page, PerPage)]
pub struct ListClubActivities {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}
#[async_trait]
impl Sendable<Vec<Lap>> for ListClubActivities {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, Page, PerPage)]
pub struct ListClubAdmins {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<Lap>> for ListClubAdmins {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct GetClub {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<Lap>> for GetClub {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, Page, PerPage)]
pub struct GetClubMembers {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<Lap>> for GetClubMembers {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}
