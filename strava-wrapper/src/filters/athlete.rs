use crate::models::{Activity, DetailedAthlete, Lap};
use crate::query::{
    get_with_query_and_path, After, Before, Endpoint, ErrorWrapper, Page, PathQuery, PerPage,
    Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{After, Before, Endpoint, Page, PathQuery, PerPage, Query, ID};

// TODO: "Tokens with profile:read_all scope will receive a detailed athlete representation; all others will receive a summary representation."

#[derive(Debug, Clone, Endpoint, Query, PathQuery)]
pub struct GetAthlete {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<DetailedAthlete> for GetAthlete {
    async fn send(mut self) -> Result<DetailedAthlete, ErrorWrapper> {
        // TODO test GET with no query+path params
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ListAthleteClubs {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<Lap>> for ListAthleteClubs {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, Before, After, Page, PerPage)]
pub struct ListAthleteActivities {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<Activity>> for ListAthleteActivities {
    async fn send(mut self) -> Result<Vec<Activity>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}
