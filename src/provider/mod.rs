use actix::{self, Actor, Addr, Arbiter, Context, Handler, Message, Syn};
/// Continuous Integration providers. These are the services that perform the
/// build pipeline and provide an interface that this tool will query.
use model::{BuildQuery, BuildResponse, BuildConfig, CiProvider};
use std::sync::Arc;
use std::rc::Rc;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Interval;

pub mod travis;

pub fn start_backend(builds: Vec<BuildConfig>) {
    let system = actix::System::new("backend");
    let addr: Addr<Syn, ProviderService> = ProviderService.start();

    let addr2 = Rc::new(addr.clone());
    let ci_addr = builds.iter().map(move |build| {
        match build.provider {
            CiProvider::Travis => {
                let travis = travis::TravisApi::new(addr2.as_ref().to_owned());
                (Arc::new(travis.start()), build.query.clone())
            }
        }
    });
    let tasks = ci_addr.map(move |(x, query): (Arc<Addr<Syn, _>>, BuildQuery)| {
        Interval::new(Instant::now(), Duration::new(3, 0))
            .take(10)
            .for_each(move |_| {
                x.as_ref().do_send(query.clone());
                Ok(())
            })
            .map_err(|e| panic!("interval errored; err={:?}", e))
    });

    tasks.for_each(|task| Arbiter::handle().spawn(task));

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
