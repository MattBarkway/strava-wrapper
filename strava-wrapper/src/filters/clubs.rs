use crate::models::{ClubActivity, ClubAthlete, DetailedClub, SummaryAthlete};
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, Page, PathQuery, PerPage, Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, Page, PathQuery, PerPage, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, Page, PerPage)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct ListClubActivities {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<ClubActivity>> for ListClubActivities {
    async fn send(self) -> Result<Vec<ClubActivity>, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, Page, PerPage)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct ListClubAdmins {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<SummaryAthlete>> for ListClubAdmins {
    async fn send(self) -> Result<Vec<SummaryAthlete>, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetClub {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<DetailedClub> for GetClub {
    async fn send(self) -> Result<DetailedClub, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, Page, PerPage)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetClubMembers {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<ClubAthlete>> for GetClubMembers {
    async fn send(self) -> Result<Vec<ClubAthlete>, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}
