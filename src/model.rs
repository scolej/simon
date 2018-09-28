use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BuildStatus {
    InProgress,
    Failed,
    Passed,
}

/// An in-progress or completed build of a project/pipeline etc.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Build {
    /// Name of the thing being built.
    pub name: String,
    /// Identifier for the build. Eg: a sequential build number.
    pub identifier: String,
    /// Status of the build.
    pub status: BuildStatus,
    /// Hash of the commit being built.
    pub commit: Option<String>,
    /// Branch being built.
    pub branch: Option<String>,
}
