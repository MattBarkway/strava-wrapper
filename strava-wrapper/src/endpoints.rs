use crate::filters::activities::{CreateActivityRequest, GetActivity, UpdateActivityRequest};
use crate::filters::activity_zones::ListActivityZones;
use crate::filters::athlete::{GetAthlete, ListAthleteActivities, ListAthleteClubs, UpdateAthlete};
use crate::filters::athlete_zones::GetAthleteZones;
use crate::filters::clubs::{GetClub, GetClubMembers, ListClubActivities, ListClubAdmins};
use crate::filters::comments::ListActivityComments;
use crate::filters::gear::GetGear;
use crate::filters::kudos::ListActivityKudoers;
use crate::filters::laps::ListActivityLaps;
use crate::filters::routes::{ExportGPXRoute, ExportTCXRoute, GetRoute, ListAthleteRoutes};
use crate::filters::segment_efforts::{GetSegmentEffort, ListSegmentEfforts};
use crate::filters::segments::{ExploreSegments, GetSegment, ListStarredSegments, StarSegment};
use crate::filters::stats::GetAthleteStats;
use crate::filters::streams::{
    GetActivityStreams, GetRouteStreams, GetSegmentEffortStreams, GetSegmentStreams,
};
use crate::filters::uploads::{GetUpload, UploadActivity, UploadDataType};
use crate::models::{CreateActivity, UpdatableActivity};
use crate::query::Endpoint;

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

    pub fn create(&self, body: CreateActivity) -> CreateActivityRequest {
        CreateActivityRequest::new(&self.url, &self.token, body)
    }

    pub fn update(&self, id: u64, body: UpdatableActivity) -> UpdateActivityRequest {
        UpdateActivityRequest::new(&self.url, &self.token, id, body)
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

    pub fn activities(&self) -> ListAthleteActivities {
        ListAthleteActivities::new(&self.url, &self.token, "v3/athlete/activities")
    }

    /// Update the authenticated athlete's weight.
    pub fn update(&self, weight: f32) -> UpdateAthlete {
        UpdateAthlete::new(&self.url, &self.token, weight)
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
        GetAthleteStats::new(&self.url, &self.token, "v3/athletes/{id}/stats")
    }

    pub fn routes(&self) -> ListAthleteRoutes {
        ListAthleteRoutes::new(&self.url, &self.token, "v3/athletes/{id}/routes")
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
        ExportTCXRoute::new(&self.url, &self.token, "v3/routes/{id}/export_tcx")
    }

    pub fn gpx(&self) -> ExportGPXRoute {
        ExportGPXRoute::new(&self.url, &self.token, "v3/routes/{id}/export_gpx")
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
        ExploreSegments::new(&self.url, &self.token, "v3/segments/explore")
    }

    pub fn starred(&self) -> ListStarredSegments {
        ListStarredSegments::new(&self.url, &self.token, "v3/segments/starred")
    }

    pub fn get(&self) -> GetSegment {
        GetSegment::new(&self.url, &self.token, "v3/segments/{id}")
    }

    /// Star or unstar a segment. Pass `true` to star, `false` to unstar.
    pub fn star(&self, id: u64, starred: bool) -> StarSegment {
        StarSegment::new(&self.url, &self.token, id, starred)
    }
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
        GetSegmentEffort::new(&self.url, &self.token, "v3/segment_efforts/{id}")
    }

    pub fn list(&self) -> ListSegmentEfforts {
        ListSegmentEfforts::new(&self.url, &self.token, "v3/segment_efforts")
    }
}

pub struct StreamsEndpoint {
    url: String,
    token: String,
}

impl StreamsEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }

    pub fn activity(&self) -> GetActivityStreams {
        GetActivityStreams::new(&self.url, &self.token, "v3/activities/{id}/streams")
    }

    pub fn route(&self) -> GetRouteStreams {
        GetRouteStreams::new(&self.url, &self.token, "v3/routes/{id}/streams")
    }

    pub fn segment_effort(&self) -> GetSegmentEffortStreams {
        GetSegmentEffortStreams::new(&self.url, &self.token, "v3/segment_efforts/{id}/streams")
    }

    pub fn segment(&self) -> GetSegmentStreams {
        GetSegmentStreams::new(&self.url, &self.token, "v3/segments/{id}/streams")
    }
}

pub struct UploadsEndpoint {
    url: String,
    token: String,
}

impl UploadsEndpoint {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
        }
    }

    /// Start a new activity upload. `file` is the raw file bytes;
    /// `data_type` identifies the format (e.g. `Fit`, `GpxGz`).
    pub fn upload(&self, file: Vec<u8>, data_type: UploadDataType) -> UploadActivity {
        UploadActivity::new(&self.url, &self.token, file, data_type)
    }

    /// Poll the processing status of a previously submitted upload.
    pub fn get(&self) -> GetUpload {
        GetUpload::new(&self.url, &self.token, "v3/uploads/{id}")
    }
}
