extern crate simon;
use simon::provider::ProviderApi;
use simon::BuildQuery;

fn main() {
    println!("Checking the backend");
    let travis = simon::provider::travis::TravisApi {};
    let query = BuildQuery {
        branch: "master".to_owned(),
        project: "made-up".to_owned(),
        namespace: "maccoda".to_owned(),
    };
    travis.build_status(query).unwrap();
}
