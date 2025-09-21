use crate::models::{Lap, Zones};
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, PathQuery, Query,
    Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ListActivityLaps {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl ListActivityLaps {
    pub fn new(url: impl Into<String>, token: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
            path: path.into(),
            query: Vec::new(),
            path_params: Vec::new(),
        }
    }
}

#[async_trait]
impl Sendable<ListActivityLaps, Vec<Lap>> for ListActivityLaps {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}
