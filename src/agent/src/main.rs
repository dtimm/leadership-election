use log::{debug, info};
use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::sync::atomic::{AtomicBool, Ordering};

static LEADER: AtomicBool = AtomicBool::new(true);

#[tokio::main]
async fn main() {
    info!("Starting Leadership Election...");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });
    let server = Server::bind(&addr).serve(make_svc);

    debug!("Leadership Election server is running");

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
