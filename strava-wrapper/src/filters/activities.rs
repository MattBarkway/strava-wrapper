use crate::models::Activity;
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, IncludeAllEfforts, PathQuery, Query, Sendable,
    ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, IncludeAllEfforts, PathQuery, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, IncludeAllEfforts)]
pub struct GetActivity {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Activity> for GetActivity {
    async fn send(mut self) -> Result<Activity, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}
