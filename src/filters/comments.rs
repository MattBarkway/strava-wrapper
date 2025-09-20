use crate::models::Comment;
use crate::query::{
    get_with_query_and_path, AfterCursor, Endpoint, ErrorWrapper, Page, PageSize, PathQuery, Query,
    Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CommentFilter {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl CommentFilter {
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

impl ID for CommentFilter {
    fn id(mut self, id: u64) -> Self {
        self.path_params.push(("id".to_string(), id.to_string()));
        self
    }
}

impl Endpoint for CommentFilter {
    fn endpoint(&self) -> String {
        format!("{}/{}", self.url, self.path)
    }
}

#[async_trait]
impl Sendable<CommentFilter, Vec<Comment>> for CommentFilter {
    async fn send(mut self) -> Result<Vec<Comment>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

impl Query for CommentFilter {
    fn get_query_params(self) -> Vec<(String, String)> {
        self.query
    }
}

impl Page for CommentFilter {
    fn page(mut self, number: u32) -> Self {
        self.path_params
            .push(("page".to_string(), number.to_string()));
        self
    }
}

impl PathQuery for CommentFilter {
    fn get_path_params(&self) -> HashMap<String, String> {
        self.path_params.iter().cloned().collect()
    }
}

impl PageSize for CommentFilter {
    fn page_size(mut self, size: u32) -> Self {
        self.query.push(("per_page".to_string(), size.to_string()));
        self
    }
}

impl AfterCursor for CommentFilter {
    fn after_cursor(mut self, cursor: String) -> Self {
        self.query
            .push(("after_cursor".to_string(), cursor.to_string()));
        self
    }
}
