use crate::models::Activity;
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, PathQuery, Query, Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID, IncludeAllEfforts)]
pub struct GetActivity {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl GetActivity {
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
impl Sendable<GetActivity, Activity> for GetActivity {
    async fn send(mut self) -> Result<Activity, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

// #[derive(Debug, Clone)]
// pub struct GetActivity {
//     token: String,
//     path: String,
//     query: Vec<(String, String)>,
//     path_params: Vec<(String, String)>,
// }
// 
// impl GetActivity {
//     pub fn new(token: String, path: String) -> Self {
//         Self {
//             token,
//             path,
//             query: Vec::new(),
//             path_params: Vec::new(),
//         }
//     }
// }
