use crate::endpoints::{ActivitiesEndpoint, AthleteEndpoint, AthletesEndpoint, ClubsEndpoint, GearEndpoint, RoutesEndpoint, SegmentsEndpoint};

#[derive(Clone)]
pub struct StravaAPI {
    url: String,
    token: String,
}

impl StravaAPI {
    pub fn new(url: &str, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }

    pub fn activities(&self) -> ActivitiesEndpoint {
        ActivitiesEndpoint::new(&self.url, &self.token)
    }

    pub fn athlete(&self) -> AthleteEndpoint {
        AthleteEndpoint::new(&self.url, &self.token)
    }

    pub fn athletes(&self) -> AthletesEndpoint {
        AthletesEndpoint::new(&self.url, &self.token)
    }

    pub fn clubs(&self) -> ClubsEndpoint {
        ClubsEndpoint::new(&self.url, &self.token)
    }

    pub fn gear(&self) -> GearEndpoint {
        GearEndpoint::new(&self.url, &self.token)
    }

    pub fn routes(&self) -> RoutesEndpoint {
        RoutesEndpoint::new(&self.url, &self.token)
    }

    pub fn segments(&self) -> SegmentsEndpoint {
        SegmentsEndpoint::new(&self.url, &self.token)
    }
}
