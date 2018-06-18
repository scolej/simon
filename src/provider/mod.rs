use actix::{self, Actor, Addr, Arbiter, Context, Handler, Message, Syn};
use error::SimonError;
/// Continuous Integration providers. These are the services that perform the
/// build pipeline and provide an interface that this tool will query.
use model::{BuildQuery, BuildResponse};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Interval;

pub mod travis;

pub fn start_backend() {
    let system = actix::System::new("backend");
    let addr: Addr<Syn, ProviderService> = ProviderService.start();

    let addr2 = addr.clone();
    let travis = travis::TravisApi::new(addr2);
    let travis_addr: Arc<Addr<Syn, _>> = Arc::new(travis.start());
    let task = Interval::new(Instant::now(), Duration::new(3, 0))
        .take(10)
        .for_each(move |_| {
            let query = BuildQuery {
                branch: "master".to_owned(),
                project: "made-up".to_owned(),
                namespace: "maccoda".to_owned(),
            };
            travis_addr.as_ref().do_send(query);
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));
    Arbiter::handle().spawn(task);
//    let addr2: Addr<Syn, _> = travis::TravisApi::new(addr.clone()).start();
    system.run();
}

impl Message for BuildResponse {
    type Result = ();
}

impl Message for BuildQuery {
    type Result = ();
}

/// The service managing the requests to the continuous integration providers
/// and the accumulation of the build status. It the endpoint to find the
/// current status of each build.
pub struct ProviderService;

impl Actor for ProviderService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("The main service is running");
    }
}

impl Handler<BuildResponse> for ProviderService {
    type Result = ();

    fn handle(&mut self, msg: BuildResponse, ctx: &mut Context<Self>) -> Self::Result {
        println!("Received {:#?}", msg);
    }
}

impl ProviderService {
    pub fn new() -> ProviderService {
        ProviderService {}
    }
}
