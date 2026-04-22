use crate::models::{Activity, DetailedAthlete, SummaryClub};
use crate::query::{
    get_with_query_and_path, put_form, After, Before, Endpoint, ErrorWrapper, Page, PathQuery,
    PerPage, Query, Sendable,
};
use async_trait::async_trait;
use serde::Serialize;
use std::collections::HashMap;
use strava_wrapper_macros::{After, Before, Endpoint, Page, PathQuery, PerPage, Query};

#[derive(Debug, Clone, Endpoint, Query, PathQuery)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetAthlete {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<DetailedAthlete> for GetAthlete {
    async fn send(self) -> Result<DetailedAthlete, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct ListAthleteClubs {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<SummaryClub>> for ListAthleteClubs {
    async fn send(self) -> Result<Vec<SummaryClub>, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, Before, After, Page, PerPage)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct ListAthleteActivities {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Vec<Activity>> for ListAthleteActivities {
    async fn send(self) -> Result<Vec<Activity>, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}

#[derive(Debug, Clone, Serialize)]
struct UpdateAthleteBody {
    weight: f32,
}

/// Builder for `PUT /athlete`. Strava currently only allows updating the
/// athlete's weight via the public API.
#[must_use = "this request is not executed until you call .send().await"]
pub struct UpdateAthlete {
    url: String,
    token: String,
    weight: f32,
}

impl UpdateAthlete {
    pub fn new(url: impl Into<String>, token: impl Into<String>, weight: f32) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
            weight,
        }
    }
}

#[async_trait]
impl Sendable<DetailedAthlete> for UpdateAthlete {
    async fn send(self) -> Result<DetailedAthlete, ErrorWrapper> {
        let url = format!("{}/v3/athlete", self.url);
        put_form(
            &url,
            &self.token,
            &UpdateAthleteBody {
                weight: self.weight,
            },
        )
        .await
    }
}
