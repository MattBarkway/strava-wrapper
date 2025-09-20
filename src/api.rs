use crate::endpoints::{ActivitiesEndpoint, CommentsEndpoint, KudosEndpoint};

#[derive(Clone)]
pub struct StravaAPI {
    token: String,
}

impl StravaAPI {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }

    pub fn comments(&self) -> CommentsEndpoint {
        CommentsEndpoint::new(self.token.clone())
    }

    pub fn activities(&self) -> ActivitiesEndpoint {
        ActivitiesEndpoint::new(self.token.clone())
    }

    pub fn kudos(&self) -> KudosEndpoint {
        KudosEndpoint ::new(self.token.clone())
    }
}