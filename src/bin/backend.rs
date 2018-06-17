extern crate simon;
use simon::BuildQuery;
use simon::provider::{ProviderApi, ProviderService};

fn main() {
    println!("Checking the backend");
    simon::provider::start_backend();
    // let service = ProviderService::new().start()
    // let travis = simon::provider::travis::TravisApi {};
    // let query = BuildQuery {
    //     branch: "master".to_owned(),
    //     project: "made-up".to_owned(),
    //     namespace: "maccoda".to_owned(),
    // };
    // travis.build_status(query).unwrap();
}
