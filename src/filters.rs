use crate::models::{Activity, Comment, CreateActivity, User};
use crate::query;
use crate::query::{
    get_with_query, get_with_query_and_path, AfterCursor, EndPoint, ErrorWrapper,
    IncludeAllEfforts, Page, PageSize, PathQuery, PerPage, Query, Sendable, TimeFilter, ID,
};
use async_trait::async_trait;
use std::fmt::Debug;
use strava_macros::{EndPoint, PathQuery, Query};

#[derive(Debug, Clone, Query, EndPoint)]
pub struct ActivityFilter {
    token: String,
    path: String,
    query: Vec<(String, String)>,
}

impl ActivityFilter {
    pub fn new(token: String, path: String) -> Self {
        Self {token, path, query: Vec::new()}
    }
}

#[async_trait]
impl Sendable<ActivityFilter, Vec<Activity>> for ActivityFilter {
    async fn send(mut self, token: &str) -> Result<Vec<Activity>, ErrorWrapper> {
        get_with_query(self.clone(), token).await
    }
}

impl TimeFilter for ActivityFilter {}
impl Page for ActivityFilter {}
impl PerPage for ActivityFilter {}

#[derive(Debug, Clone, Query, PathQuery, EndPoint)]
pub struct GetActivity {
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl GetActivity {
    pub fn new(token: String, path: String) -> Self {
        Self {token, path, query: Vec::new(), path_params: Vec::new()}
    }
}

impl ID for GetActivity {}

impl IncludeAllEfforts for GetActivity {}

#[async_trait]
impl Sendable<GetActivity, Activity> for GetActivity {
    async fn send(mut self, token: &str) -> Result<Activity, ErrorWrapper> {
        get_with_query_and_path(self.clone(), token).await
    }
}

#[derive(Debug, Clone, Query, PathQuery, EndPoint)]
pub struct CommentFilter {
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}
impl CommentFilter {
    pub fn new(token: String, path: String) -> Self {
        Self { token, path, query: Vec::new(), path_params: Vec::new() }
    }
}

#[async_trait]
impl Sendable<CommentFilter, Vec<Comment>> for CommentFilter {
    async fn send(mut self, token: &str) -> Result<Vec<Comment>, ErrorWrapper> {
        get_with_query(self.clone(), token).await
    }
}

impl Page for CommentFilter {}

impl PageSize for CommentFilter {}

impl AfterCursor for CommentFilter {}

#[derive(Debug, Clone, Query, PathQuery, EndPoint)]
pub struct KudosFilter {
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}
impl KudosFilter {
    pub fn new(token: String, path: String) -> Self {
        Self {
            token,
            path,
            query: Vec::new(),
            path_params: Vec::new(),
        }
    }
}

#[async_trait]
impl Sendable<KudosFilter, Vec<User>> for KudosFilter {
    async fn send(mut self, token: &str) -> Result<Vec<User>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), token).await
    }
}
impl ID for KudosFilter {}
impl Page for KudosFilter {}
impl PerPage for KudosFilter {}

#[derive(Debug, EndPoint)]
pub struct CreateActivityFilter {
    token: String,
    path: String,
    payload: CreateActivity,
}

impl CreateActivityFilter {
    pub fn new(token: String, path: String, payload: CreateActivity) -> Self {
        Self {token, path, payload}
    }
}

#[async_trait]
impl Sendable<CreateActivityFilter, Activity> for CreateActivityFilter {
    async fn send(self, token: &str) -> Result<Activity, ErrorWrapper> {
        query::post(&self.path(), token, self.payload).await
    }
}
