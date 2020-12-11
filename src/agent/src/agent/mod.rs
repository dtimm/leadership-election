use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;

pub struct Agent {
    leader: bool,
}

impl Agent {
    pub async fn serve(&self, _req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let mut response = Response::new(Body::empty());

        if self.leader {
            *response.body_mut() = Body::from("This node is the leader");
        } else {
            *response.body_mut() = Body::from("This node is not the leader");
            *response.status_mut() = StatusCode::LOCKED;
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn leader_serves_200() {
        let req = Request::builder()
            .uri("http://test")
            .body(Body::from(""))
            .unwrap();

        let agent = Agent { leader: true };
        let resp = agent.serve(req).await;
        assert_eq!(resp.unwrap().status(), 200);
    }

    #[tokio::test]
    async fn follower_serves_423() {
        let req = Request::builder()
            .uri("http://test")
            .body(Body::from(""))
            .unwrap();

        let agent = Agent { leader: false };
        let resp = agent.serve(req).await;
        assert_eq!(resp.unwrap().status(), 423);
    }
}
