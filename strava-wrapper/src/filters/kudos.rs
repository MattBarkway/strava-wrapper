use crate::models::SummaryAthlete;
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, Page, PathQuery, PerPage, Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, Page, PathQuery, PerPage, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, Page, PerPage)]
pub struct ListActivityKudoers {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<SummaryAthlete>> for ListActivityKudoers {
    async fn send(self) -> Result<Vec<SummaryAthlete>, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}
