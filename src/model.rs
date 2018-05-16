/// Modelled representation of the build data to exchange between front end and
/// API
use std::time::Duration;

pub struct BuildQuery {
    branch: String,
    project: String,
}

pub struct BuildResponse {
    build: Build,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BuildStatus {
    InProgress,
    Failed,
    Passed,
}

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
