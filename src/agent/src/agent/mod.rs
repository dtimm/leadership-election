use hyper::service::Service;
use hyper::{Body, Request, Response, StatusCode};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Agent {
    leader: bool,
}

impl Service<Request<Body>> for Agent {
    type Response = Response<Body>;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, _req: Request<Body>) -> Self::Future {
        let mut body: &str = "This node is the leader";
        let mut status = StatusCode::OK;
        if !self.leader {
            status = StatusCode::LOCKED;
            body = "This node is not the leader";
        }

        let resp = Response::builder().status(status).body(Body::from(body));

        Box::pin(async { resp })
    }
}

pub struct MakeAgent {
    leader: bool,
}

impl<T> Service<T> for MakeAgent {
    type Response = Agent;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let leader = self.leader.clone();
        let fut = async move { Ok(Agent { leader }) };
        Box::pin(fut)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn leader_serves_200() {
//         let req = Request::builder()
//             .uri("http://test")
//             .body(Body::from(""))
//             .unwrap();

//         let agent = Agent { leader: true };
//         let resp = agent.call(req).await;
//         assert_eq!(resp.unwrap().status(), 200);
//     }

//     #[tokio::test]
//     async fn follower_serves_423() {
//         let req = Request::builder()
//             .uri("http://test")
//             .body(Body::from(""))
//             .unwrap();

//         let agent = Agent { leader: false };
//         let resp = agent.call(req).await;
//         assert_eq!(resp.unwrap().status(), 423);
//     }
// }
