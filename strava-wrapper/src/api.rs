use crate::endpoints::{ActivitiesEndpoint};

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
        // create
        // get X
        // update
        // list comments: X
        // list kudos:
        // list laps:
        // list zones:
        //
        ActivitiesEndpoint::new(self.url.clone(), self.token.clone())
    }

    pub fn athletes(&self) -> () {
        // get (current athlete)
        // zones
        // stats
        // update
        todo!();
    }

    pub fn clubs(&self) -> () {
        // activities
        // admins
        // get
        // list members
        // list athlete's clubs
        todo!();
    }

    pub fn gear(&self) -> () {
        // get
        todo!();
    }

    pub fn routes(&self) -> () {
        // export GPX
        // export TCX
        // get
        // list athlete's routes
        todo!();
    }

    pub fn segments(&self) -> () {
        // explore
        // list starred
        // get
        // star
        todo!();
        // goes to SegmentEfforts
    }
}
