use crate::models::{Lap, Zones};
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, PathQuery, Query,
    Sendable, ID,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query, ID};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ListClubActivities {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl ListClubActivities {
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
impl Sendable<ListClubActivities, Vec<Lap>> for ListClubActivities {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}


#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ListClubAdmins {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl ListClubAdmins {
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
impl Sendable<ListClubAdmins, Vec<Lap>> for ListClubAdmins {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct GetClub {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl GetClub {
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
impl Sendable<GetClub, Vec<Lap>> for GetClub {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}


#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct GetClubMembers {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl GetClubMembers {
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
impl Sendable<GetClubMembers, Vec<Lap>> for GetClubMembers {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}




#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
pub struct ListAthleteClubs {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

impl ListAthleteClubs {
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
impl Sendable<ListAthleteClubs, Vec<Lap>> for ListAthleteClubs {
    async fn send(mut self) -> Result<Vec<Lap>, ErrorWrapper> {
        get_with_query_and_path(self.clone(), &self.token).await
    }
}