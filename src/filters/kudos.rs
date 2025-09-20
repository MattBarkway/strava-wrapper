use crate::models::{User};
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, Page, PageSize, PathQuery, Query,
    Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KudosFilter {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl KudosFilter {
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

impl Endpoint for KudosFilter {
    fn endpoint(&self) -> String {
        format!("{}/{}", self.url, self.path)
    }
}

#[async_trait]
impl Sendable<KudosFilter, Vec<User>> for KudosFilter {
    async fn send(mut self) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

impl Query for KudosFilter {
    fn get_query_params(self) -> Vec<(String, String)> {
        self.query
    }
}

impl PathQuery for KudosFilter {
    fn get_path_params(&self) -> HashMap<String, String> {
        self.path_params.iter().cloned().collect()
    }
}

impl ID for KudosFilter {
    fn id(mut self, id: u64) -> Self {
        self.path_params.push(("id".to_string(), id.to_string()));
        self
    }
}

impl Page for KudosFilter {
    fn page(mut self, number: u32) -> Self {
        self.query.push(("page".to_string(), number.to_string()));
        self
    }
}

impl PageSize for KudosFilter {
    fn page_size(mut self, size: u32) -> Self {
        self.query.push(("per_page".to_string(), size.to_string()));
        self
    }
}
