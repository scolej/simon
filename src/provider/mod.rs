/// Continuous Integration providers. These are the services that perform the
/// build pipeline and provide an interface that this tool will query.
use model::{BuildQuery, BuildResponse};
use failure::Error;

pub mod travis;

pub trait ProviderApi {
    fn build_status(&self, query: BuildQuery) -> Result<BuildResponse, Error>;
}

#[derive(Fail, Debug)]
#[fail(display = "API call failed for: {}", provider)]
struct ApiError {
    provider: String,
}
