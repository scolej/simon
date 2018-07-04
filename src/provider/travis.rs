use super::ProviderService;
use actix::{Actor, Addr, Context, Handler, Syn};
use dotenv::dotenv;
use error::SimonError;
use model::{BuildQuery, BuildResponse};
use reqwest::{self, header};
use std::env;

const URL: &str = "https://api.travis-ci.org";

pub struct TravisApi {
    provider_service: Addr<Syn, ProviderService>,
}

header! {(TravisVersion, "Travis-API-Version") => [String]}
impl TravisApi {
    pub fn new(provider_service: Addr<Syn, ProviderService>) -> TravisApi {
        TravisApi { provider_service }
    }
    fn headers(&self) -> header::Headers {
        let mut headers = header::Headers::new();
        dotenv().ok();
        let travis_token = env::var("TRAVIS_TOKEN").expect("Could not locate token for Travis");
        headers.set(header::Authorization("token ".to_owned() + &travis_token));
        headers.set(header::UserAgent::new("API Explorer"));
        headers.set(TravisVersion("3".to_owned()));
        headers
    }

    fn build_status(&self, query: BuildQuery) -> Result<BuildResponse, SimonError> {
        let client = reqwest::Client::new();
        let api = format!(
            "{}/repo/{}%2F{}/branch/{}",
            URL, query.namespace, query.project, query.branch
        );

        let headers = self.headers();
        let mut res = client
            .get(&api)
            .headers(headers)
            .query(&[("include", "build.commit")])
            .send()?;
        let result: model::TravisResponse = res.json()?;
        Ok(result.into())
    }
}

impl Actor for TravisApi {
    type Context = Context<Self>;
}

impl Handler<BuildQuery> for TravisApi {
    type Result = ();

    fn handle(&mut self, msg: BuildQuery, ctx: &mut Context<Self>) -> Self::Result {
        if let Some(response) = self.build_status(msg).ok() {
            self.provider_service.do_send(response);
        }
    }
}

mod model {
    use model::{Build, BuildId, BuildResponse, BuildStatus};
    use serde_json;
    use std::time::Duration;

    impl From<TravisResponse> for BuildResponse {
        fn from(f: TravisResponse) -> Self {
            let status = if f.last_build.state.eq("passed") {
                BuildStatus::Passed
            } else {
                BuildStatus::Failed
            };
            let build = Build {
                id: BuildId {
                    number: f.last_build.id as u16,
                    branch: f.name,
                },
                commit: String::new(),
                status: status,
                elapsed_time: Duration::from_secs(f.last_build.duration),
            };
            BuildResponse { build }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TravisResponse {
        #[serde(rename = "@type")]
        travis_response_type: String,
        #[serde(rename = "@href")]
        href: String,
        #[serde(rename = "@representation")]
        representation: String,
        name: String,
        repository: Repository,
        default_branch: bool,
        exists_on_github: bool,
        last_build: LastBuild,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LastBuild {
        #[serde(rename = "@type")]
        last_build_type: String,
        #[serde(rename = "@href")]
        href: String,
        #[serde(rename = "@representation")]
        representation: String,
        id: i64,
        number: String,
        state: String,
        duration: u64,
        event_type: String,
        previous_state: String,
        pull_request_title: Option<serde_json::Value>,
        pull_request_number: Option<serde_json::Value>,
        started_at: String,
        finished_at: String,
        private: bool,
        commit: Commit,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Repository {
        #[serde(rename = "@type")]
        repository_type: String,
        #[serde(rename = "@href")]
        href: String,
        #[serde(rename = "@representation")]
        representation: String,
        id: i64,
        name: String,
        slug: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Commit {
        #[serde(rename = "@type")]
        commit_type: String,
        #[serde(rename = "@representation")]
        representation: String,
        id: i64,
        sha: String,
        #[serde(rename = "ref")]
        commit_ref: String,
        message: String,
        compare_url: String,
        committed_at: String,
        committer: Author,
        author: Author,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Author {
        name: String,
        avatar_url: String,
    }
}
