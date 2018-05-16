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
struct Build {
    id: BuildId,
    commit: String,
    status: BuildStatus,
    elapsed_time: Duration,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct BuildId {
    branch: String,
    number: u16,
}
