mod server;

extern crate hyper;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::{error, info};
use std::convert::Infallible;
use std::net::SocketAddr;

pub struct LeadershipServer();

impl LeadershipServer {
    async fn start(&self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

        let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(serve)) });

        let server = Server::bind(&addr).serve(make_svc);

        if let Err(e) = server.await {
            error!("server error: {}", e);
        }
    }

    fn stop(&self) {}
}

async fn serve(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use hyper::{Client, Uri};

    #[tokio::test]
    async fn it_serves_200() {
        let client = Client::new();
        let uri = "http://localhost:8080".parse::<Uri>().unwrap();

        let server = LeadershipServer();
        server.start();

        let resp = client.get(uri).await.unwrap();

        assert_eq!(resp.status(), 200);

        server.stop()
    }
}
