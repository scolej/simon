use actix::{self, Actor, Addr, Context, Message, Syn, Handler, Arbiter};
use error::SimonError;
/// Continuous Integration providers. These are the services that perform the
/// build pipeline and provide an interface that this tool will query.
use model::{BuildQuery, BuildResponse};
use tokio;
use tokio::prelude::*;
use tokio::timer::Interval;
use std::time::{Duration, Instant};
use std::cell::RefCell;
use std::sync::Arc;

pub mod travis;

pub fn start_backend() {
    let system = actix::System::new("backend");
    let addr: Addr<Syn, ProviderService> = ProviderService.start();

    let addr2 = Arc::new(addr.clone());
    let task = Interval::new(Instant::now(), Duration::from_millis(100))
        .take(10)
        .for_each( move |_| {
            let addr2: Addr<Syn, _> = travis::TravisApi::new(addr2.as_ref().to_owned()).start();
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));
    Arbiter::handle().spawn(task);
//    let addr2: Addr<Syn, _> = travis::TravisApi::new(addr.clone()).start();
    system.run();
}
pub trait ProviderApi {
    fn build_status(&self, query: BuildQuery) -> Result<BuildResponse, SimonError>;
}

impl Message for BuildResponse {
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

    // pub fn get_build_status(&self) -> Vec<Build> {}
}
