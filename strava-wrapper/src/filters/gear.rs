use crate::models::DetailedGear;
use crate::query::{
    get_with_query_and_path, Endpoint, ErrorWrapper, GearID, PathQuery, Query, Sendable,
};
use async_trait::async_trait;
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, GearID, PathQuery, Query};

#[derive(Debug, Clone, Endpoint, Query, PathQuery, GearID)]
pub struct GetGear {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<DetailedGear> for GetGear {
    async fn send(self) -> Result<DetailedGear, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}
