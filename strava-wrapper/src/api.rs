use crate::endpoints::{
    ActivitiesEndpoint, AthleteEndpoint, AthletesEndpoint, ClubsEndpoint, GearEndpoint,
    RoutesEndpoint, SegmentsEndpoint, StreamsEndpoint, UploadsEndpoint,
};
use crate::query::{last_rate_limit, RateLimit};
use std::sync::{Arc, RwLock};

/// Entry point into the Strava API wrapper.
///
/// `StravaAPI` is cheap to clone: it only contains an `Arc` reference to the
/// shared token. Mutating the token via [`StravaAPI::set_token`] is visible to
/// all clones, which lets long-running applications refresh the access token
/// without rebuilding the whole call stack.
#[derive(Clone)]
pub struct StravaAPI {
    url: String,
    token: Arc<RwLock<String>>,
}

impl StravaAPI {
    pub fn new(url: &str, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: Arc::new(RwLock::new(token.into())),
        }
    }

    /// Returns a clone of the current access token.
    pub fn token(&self) -> String {
        self.token.read().expect("token lock poisoned").clone()
    }

    /// Replaces the access token. Use after calling
    /// [`auth::refresh_token`](crate::auth::refresh_token) to swap the expired
    /// access token for the newly issued one.
    pub fn set_token(&self, token: impl Into<String>) {
        *self.token.write().expect("token lock poisoned") = token.into();
    }

    /// Latest rate-limit snapshot observed from any Strava response in this
    /// process. Returns `None` until the first request with valid
    /// `X-RateLimit-*` headers lands.
    ///
    /// Strava rate limits are per-application, so the snapshot is shared
    /// across every `StravaAPI` instance in the process.
    pub fn rate_limit(&self) -> Option<RateLimit> {
        last_rate_limit()
    }

    pub fn activities(&self) -> ActivitiesEndpoint {
        ActivitiesEndpoint::new(&self.url, self.token())
    }

    pub fn athlete(&self) -> AthleteEndpoint {
        AthleteEndpoint::new(&self.url, self.token())
    }

    pub fn athletes(&self) -> AthletesEndpoint {
        AthletesEndpoint::new(&self.url, self.token())
    }

    pub fn clubs(&self) -> ClubsEndpoint {
        ClubsEndpoint::new(&self.url, self.token())
    }

    pub fn gear(&self) -> GearEndpoint {
        GearEndpoint::new(&self.url, self.token())
    }

    pub fn routes(&self) -> RoutesEndpoint {
        RoutesEndpoint::new(&self.url, self.token())
    }

    pub fn segments(&self) -> SegmentsEndpoint {
        SegmentsEndpoint::new(&self.url, self.token())
    }

    pub fn streams(&self) -> StreamsEndpoint {
        StreamsEndpoint::new(&self.url, self.token())
    }

    pub fn uploads(&self) -> UploadsEndpoint {
        UploadsEndpoint::new(&self.url, self.token())
    }
}
