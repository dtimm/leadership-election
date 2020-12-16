mod agent;

// use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use log::{debug, info};
// use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    info!("Starting Leadership Election...");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = Server::bind(&addr).serve(agent::MakeAgent { leader: true });

    debug!("Leadership Election server is running");

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
