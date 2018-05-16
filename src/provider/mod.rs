/// Continuous Integration providers. These are the services that perform the
/// build pipeline and provide an interface that this tool will query.
use model::{BuildQuery, BuildResponse};

trait ProviderApi {
    fn build_status(BuildQuery) -> BuildResponse;
}
