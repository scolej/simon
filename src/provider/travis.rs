use super::ProviderApi;
use model::{Build, BuildId, BuildQuery, BuildResponse, BuildStatus};
use reqwest::{self, header};
use failure::Error;
use std::convert::From;
use std::time::Duration;
use dotenv::dotenv;
use std::env;

const URL: &str = "https://api.travis-ci.org";
pub struct TravisApi;

header!{(TravisVersion, "Travis-API-Version") => [String]}
impl TravisApi {
    fn headers(&self) -> header::Headers {
        let mut headers = header::Headers::new();
        dotenv().ok();
        let travis_token = env::var("TRAVIS_TOKEN").expect("Could not locate token for Travis");
        headers.set(header::Authorization("token ".to_owned() + &travis_token));
        headers.set(header::UserAgent::new("API Explorer"));
        headers.set(TravisVersion("3".to_owned()));
        headers
    }
}

impl ProviderApi for TravisApi {
    fn build_status(&self, query: BuildQuery) -> Result<BuildResponse, Error> {
        let client = reqwest::Client::new();
        let api = format!(
            "{}/repo/{}%2F{}/branch/{}",
            URL, query.namespace, query.project, query.branch
        );

        let headers = self.headers();
        println!("Hitting {}", api);
        let mut res = client.get(&api).headers(headers).send()?;
        println!("{:?}", res);
        let result: TravisResponse = res.json()?;
        println!("{:?}", result);
        Ok(result.into())
    }
}

// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::TravisResponse;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: TravisResponse = serde_json::from_str(&json).unwrap();
// }

impl From<TravisResponse> for BuildResponse {
    fn from(f: TravisResponse) -> Self {
        let status = if f.last_build.state.eq("passed") {
            BuildStatus::Passed
        } else {
            BuildStatus::Failed
        };
        // TODO: Convert the ISO formated time to get duration
        let build = Build {
            id: BuildId {
                number: f.last_build.id as u16,
                branch: f.name,
            },
            // TODO: Need to work out where this comes from
            commit: String::new(),
            status: status,
            elapsed_time: Duration::from_secs(60),
        };
        BuildResponse { build }
    }
}

extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct TravisResponse {
    #[serde(rename = "@type")] travis_response_type: String,
    #[serde(rename = "@href")] href: String,
    #[serde(rename = "@representation")] representation: String,
    name: String,
    repository: Repository,
    default_branch: bool,
    exists_on_github: bool,
    last_build: LastBuild,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastBuild {
    #[serde(rename = "@type")] last_build_type: String,
    #[serde(rename = "@href")] href: String,
    #[serde(rename = "@representation")] representation: String,
    id: i64,
    number: String,
    state: String,
    duration: i64,
    event_type: String,
    previous_state: String,
    pull_request_title: Option<serde_json::Value>,
    pull_request_number: Option<serde_json::Value>,
    started_at: String,
    finished_at: String,
    private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    #[serde(rename = "@type")] repository_type: String,
    #[serde(rename = "@href")] href: String,
    #[serde(rename = "@representation")] representation: String,
    id: i64,
    name: String,
    slug: String,
}
