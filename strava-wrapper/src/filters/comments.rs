use crate::models::Comment;
use crate::query::{
    get_with_query_and_path, AfterCursor, Endpoint, ErrorWrapper, PageSize, PathQuery, Query,
    Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{AfterCursor, Endpoint, PageSize, PathQuery, Query, ID};


#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, PageSize, AfterCursor)]
pub struct ListActivityComments {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl ListActivityComments {
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
impl Sendable<ListActivityComments, Vec<Comment>> for ListActivityComments {
    async fn send(mut self) -> Result<Vec<Comment>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}
