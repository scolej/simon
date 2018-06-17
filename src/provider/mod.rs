use error::SimonError;
/// Continuous Integration providers. These are the services that perform the
/// build pipeline and provide an interface that this tool will query.
use model::{Build, BuildQuery, BuildResponse};

pub mod travis;

pub trait ProviderApi {
    fn build_status(&self, query: BuildQuery) -> Result<BuildResponse, SimonError>;
}
/// The service managing the requests to the continuous integration providers
/// and the accumulation of the build status. It the endpoint to find the
/// current status of each build.
pub struct ProviderService {
    config: Configuration,
}

struct Configuration {
    builds: Vec<BuildConfig>,
}

struct BuildConfig {
    provider: CiProvider,
}
enum CiProvider {
    Travis,
}

impl ProviderService {
    pub fn new() -> ProviderService {
        ProviderService {
            config: Configuration { builds: vec![] },
        }
    }
    pub fn start(&self) {
        println!("Starting provider service");
    }

    // pub fn get_build_status(&self) -> Vec<Build> {}
}
