use crate::filters::activities::GetActivity;
use crate::filters::athlete::{GetAthlete, ListAthleteClubs};
use crate::filters::comments::ListActivityComments;
use crate::filters::kudos::ListActivityKudoers;
use crate::filters::laps::ListActivityLaps;
use crate::filters::activity_zones::ListActivityZones;
use crate::filters::athlete_zones::GetAthleteZones;
use crate::filters::clubs::{GetClub, GetClubMembers, ListAthleteClubs, ListClubActivities, ListClubAdmins};
use crate::filters::gear::GetGear;
use crate::filters::routes::{ExportGPXRoute, ExportTCXRoute, GetRoute, ListAthleteRoutes};
use crate::filters::segment_efforts::{GetSegmentEffort, ListSegmentEfforts};
use crate::filters::segments::{ExploreSegments, GetSegment, ListStarredSegments};
use crate::filters::stats::GetAthleteStats;
use crate::query::Endpoint;
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
    pub fn get(&self) -> GetActivity {
        GetActivity::new(&self.url, &self.token, "v3/activities/{id}")
    }


    pub fn comments(&self) -> ListActivityComments {
        ListActivityComments::new(&self.url, &self.token, "v3/activities/{id}/comments")
    }

    pub fn kudos(&self) -> ListActivityKudoers {
        ListActivityKudoers::new(&self.url, &self.token, "v3/activities/{id}/kudos")
    }
    pub fn laps(&self) -> ListActivityLaps {
        ListActivityLaps::new(&self.url, &self.token, "v3/activities/{id}/laps")
    }

    pub fn zones(&self) -> ListActivityZones {
        ListActivityZones::new(&self.url, &self.token, "v3/activities/{id}/zones")
    }


    pub fn update(&self) -> () {
        todo!()
    }

    pub fn create(&self) -> () {
        todo!()
    }
}
pub struct AthleteEndpoint {
    url: String,
    token: String,
}

impl AthleteEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }
    pub fn get(&self) -> GetAthlete {
        GetAthlete::new(&self.url, &self.token, "v3/athlete")
    }

    pub fn zones(&self) -> GetAthleteZones {
        GetAthleteZones::new(&self.url, &self.token, "v3/athlete/zones")
    }

    pub fn clubs(&self) -> ListAthleteClubs {
        ListAthleteClubs::new(&self.url, &self.token, "v3/athlete/clubs")
    }
    pub fn update(&self) -> () {
        todo!()
    }
    
}
pub struct AthletesEndpoint {
    url: String,
    token: String,
}

impl AthletesEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }


    pub fn stats(&self) -> GetAthleteStats {
        // TODO should athletes be different resource to athlete? probs
        GetAthleteStats::new(&self.url, &self.token, "v3/athletes/{id}/stats")
    }
}

pub struct ClubsEndpoint {
    url: String,
    token: String,
}

impl ClubsEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }
    pub fn activities(&self) -> ListClubActivities {
        ListClubActivities::new(&self.url, &self.token, "v3/clubs/{id}/activities")
    }

    pub fn admins(&self) -> ListClubAdmins {
        ListClubAdmins::new(&self.url, &self.token, "v3/clubs/{id}/admins")
    }

    pub fn get(&self) -> GetClub {
        GetClub::new(&self.url, &self.token, "v3/clubs/{id}")
    }
    pub fn members(&self) -> GetClubMembers {
        GetClubMembers::new(&self.url, &self.token, "v3/clubs/{id}/members")
    }
}


pub struct GearEndpoint {
    url: String,
    token: String,
}

impl GearEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }
    pub fn get(&self) -> GetGear {
        GetGear::new(&self.url, &self.token, "v3/gear/{id}")
    }
}

pub struct RoutesEndpoint {
    url: String,
    token: String,
}

impl RoutesEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }
    pub fn export(&self) -> ExportRoute {
        ExportRoute::new(&self.url, &self.token)
    }

    pub fn get(&self) -> GetRoute {
        GetRoute::new(&self.url, &self.token, "v3/routes/{id}")
    }

    // TODO go through and validate args/URLs from here onwards
    pub fn list(&self) -> ListAthleteRoutes {
        ListAthleteRoutes::new(&self.url, &self.token, "v3/athlete")
    }
}

pub struct ExportRoute {
    url: String,
    token: String,
}

impl ExportRoute {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }
    pub fn tcx(&self) -> ExportTCXRoute {
        ExportTCXRoute::new(&self.url, &self.token, "v3/routes/{id}/export_gpx")
    }

    pub fn gpx(&self) -> ExportGPXRoute {
        ExportGPXRoute::new(&self.url, &self.token, "v3/routes/{id}/export_tcx")
    }
}


pub struct SegmentsEndpoint {
    url: String,
    token: String,
}

impl SegmentsEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }
    pub fn efforts(&self) -> SegmentEffort {
        SegmentEffort::new(&self.url, &self.token)
    }

    pub fn explore(&self) -> ExploreSegments {
        ExploreSegments::new(&self.url, &self.token, "v3/athlete")
    }


    pub fn starred(&self) -> ListStarredSegments {
        ListStarredSegments::new(&self.url, &self.token, "v3/athlete")
    }

    pub fn get(&self) -> GetSegment {
        GetSegment::new(&self.url, &self.token, "v3/athlete")
    }

    pub fn star() -> () {todo!()}
}


pub struct SegmentEffort {
    url: String,
    token: String,
}
impl SegmentEffort {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }

    pub fn get(&self) -> GetSegmentEffort {
        GetSegmentEffort::new(&self.url, &self.token, "v3/athlete")
    }

    pub fn list(&self) -> ListSegmentEfforts {
        ListSegmentEfforts::new(&self.url, &self.token, "v3/athlete")
    }
}