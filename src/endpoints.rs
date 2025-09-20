use crate::filters::activities::ActivityFilter;
use crate::filters::comments::CommentFilter;
use crate::filters::kudos::KudosFilter;

// TODO: implement all these...

// API
//   Activities
//     - get
//     - list
//     Kudos
//       - list
//     Comments
//       - list
//     Laps
//       - list
//     Zones
//       - list
//   Athletes
//     - get
//     - zones
//     - stats
//     - update
//   Clubs
//     - get
//     - activities
//     - admins
//     - members
//     - list (for current user)
//   Gear
//     - get
//   Routes
//     - export (GPX|TCX)
//     - get
//     - list
//   Segments
//     Efforts
//       - get
//       - list
//     - explore
//     - starred
//     - get
//     - star
//  maybes:
//  Uploads
//  Streams

pub struct CommentsEndpoint {
    url: String,
    token: String,
}

impl CommentsEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }
    pub fn get(&self) -> CommentFilter {
        CommentFilter::new(&self.url, &self.token, "v3/activities/{id}/comments")
    }
}

pub struct ActivitiesEndpoint {
    url: String,
    token: String,
}

impl ActivitiesEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }

    pub fn comments(&self) -> CommentsEndpoint {
        CommentsEndpoint::new(&self.url, &self.token)
    }

    pub fn kudos(&self) -> KudosEndpoint {
        KudosEndpoint::new(&self.url, &self.token)
    }

    pub fn list(&self) -> ActivityFilter {
        ActivityFilter::new(&self.url, &self.token, "v3/athlete/activities")
    }

    pub fn get(&self) -> ActivityFilter {
        ActivityFilter::new(&self.url, &self.token, "v3/activities/{id}")
    }

    // TODO create() endpoint
}

pub struct KudosEndpoint {
    url: String,
    token: String,
}

impl KudosEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }

    pub fn get(&self) -> KudosFilter {
        KudosFilter::new(&self.url, &self.token, "v3/activities/{id}/kudos")
    }
}
