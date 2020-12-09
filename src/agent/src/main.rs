mod server;

use log::{error, info};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    info!("Starting Leadership Election...");

    let server = server::LeadershipServer();
    server.start();
}
