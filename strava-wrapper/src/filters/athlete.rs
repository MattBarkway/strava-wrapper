use crate::models::{Lap, SimpleAthlete};
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, PathQuery, Query,
    Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query, ID};

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
impl Sendable<GetAthlete, SimpleAthlete> for GetAthlete {
    async fn send(mut self) -> Result<SimpleAthlete, ErrorWrapper> {
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
impl Sendable<ListAthleteClubs, Vec<Lap>> for ListAthleteClubs {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}