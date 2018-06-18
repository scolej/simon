extern crate simon;
use simon::*;

fn main() {
    println!("Checking the backend");
    let query = BuildQuery {
        branch: "master".to_owned(),
        project: "made-up".to_owned(),
        namespace: "maccoda".to_owned(),
    };
    let builds = vec![BuildConfig{provider: CiProvider::Travis, query: query}];
    simon::provider::start_backend(builds);
    // let service = ProviderService::new().start()
    // let travis = simon::provider::travis::TravisApi {};
    // let query = BuildQuery {
    //     branch: "master".to_owned(),
    //     project: "made-up".to_owned(),
    //     namespace: "maccoda".to_owned(),
    // };
    // travis.build_status(query).unwrap();
}
