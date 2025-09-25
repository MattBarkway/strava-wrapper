use crate::models::Zones;
use crate::query::{get_with_query_and_path, Endpoint, ErrorWrapper, PathQuery, Query, Sendable};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query};

#[derive(Debug, Clone, Endpoint, Query, PathQuery)]
pub struct GetAthleteZones {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<Zones>> for GetAthleteZones {
    async fn send(mut self) -> Result<Vec<Zones>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}
