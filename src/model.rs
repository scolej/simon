/// Modelled representation of the build data to exchange between front end and
/// API
use std::time::Duration;

pub struct BuildQuery {
    pub branch: String,
    pub project: String,
    pub namespace: String,
}

pub struct BuildResponse {
    pub build: Build,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BuildStatus {
    InProgress,
    Failed,
    Passed,
}

// FIXME: Probably do not want these to be public accessors
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Build {
    pub id: BuildId,
    pub commit: String,
    pub status: BuildStatus,
    pub elapsed_time: Duration,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct BuildId {
    pub branch: String,
    pub number: u16, // By the way, this should probably be a string, to facilitate things like matrix build numbers.
}
