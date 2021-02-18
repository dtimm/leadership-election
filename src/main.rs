use clap::Clap;
use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

mod raft;

static LEADER: AtomicBool = AtomicBool::new(true);

#[derive(Clap)]
#[clap(version = "0.1", author = "David Timm <dtimm@vmware.com>")]
struct Opts {
    #[clap(short, long, default_value = "8080")]
    port: u16,

    #[clap(short, long)]
    hosts: String,
}

#[tokio::main]
async fn main() {
    println!("Starting Leadership Election...");
    let opts: Opts = Opts::parse();

    println!("on port {}", opts.port);

    let hosts: Vec<&str> = opts.hosts.split(',').collect();
    for host in hosts {
        println!("connecting to host {}", host)
    }

    let cfg = &raft::AgentConfig{
        timeout: Duration::from_millis(5000),
    };
    let agent = raft::Agent::init(cfg);

    let addr = SocketAddr::from(([127, 0, 0, 1], opts.port));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });
    let server = Server::bind(&addr).serve(make_svc);

    println!("Leadership Election server is running");

    // LEADER.store(false, Ordering::SeqCst);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut body: &str = "This node is the leader";
    let mut status = StatusCode::OK;
    if !LEADER.load(Ordering::SeqCst) {
        status = StatusCode::LOCKED;
        body = "This node is not the leader";
    }

    let mut resp = Response::new(body.into());
    *resp.status_mut() = status;

    return Ok(resp);
}
