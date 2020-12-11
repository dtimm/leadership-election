mod agent;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use log::{debug, info};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    info!("Starting Leadership Election...");
    let agent = agent::Agent { leader: true };
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(|req| agent.serve(req))) });
    let server = Server::bind(&addr).serve(make_svc);

    debug!("Leadership Election server is running");

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
