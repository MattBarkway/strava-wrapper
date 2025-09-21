use crate::models::{User};
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, Page, PathQuery, Query, PerPage,
    Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, Page, PerPage, PathQuery, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, Page, PerPage)]
pub struct ListActivityKudoers {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl ListActivityKudoers {
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
impl Sendable<ListActivityKudoers, Vec<User>> for ListActivityKudoers {
    async fn send(mut self) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}
