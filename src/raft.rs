extern crate atomic;
extern crate tokio;

use atomic::{Atomic, Ordering};
use std::time::Duration;
use tokio::time::timeout;
use tokio::sync::oneshot;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Role {
    Follower,
    Candidate,
    Leader,
}

pub struct Agent {
    role: Atomic<Role>,
    timeout: Duration,
}

pub struct AgentConfig {
    pub timeout: Duration,
    pub servers: Vec<String>,
}

impl Agent {
    pub fn init(cfg: &AgentConfig) -> Agent {
        let agent = Agent {
            role: Atomic::new(Role::Follower),
            timeout: cfg.timeout,
        };

        return agent
    }

    async fn wait(&self) -> &Agent {
        let (_tx, rx) = oneshot::channel::<i64>();
        if let Err(_) = timeout(self.timeout, rx).await {
            self.role.store(Role::Candidate, Ordering::SeqCst)
        };
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::convert::Infallible;
    use std::net::SocketAddr;
    use hyper::{Body, Request, Response, Server};
    use hyper::service::{make_service_fn, service_fn};

    #[tokio::test]
    async fn test_starts_as_follower() {
        let cfg = AgentConfig{
            timeout: Duration::from_millis(5),
            servers: vec!(String::from("localhost:8080")),
        };
        let agent = Agent::init(&cfg);
        assert_eq!(agent.role.load(Ordering::SeqCst), Role::Follower);
    }

    #[tokio::test]
    async fn test_switches_to_candidate_after_timeout() {
        let cfg = AgentConfig{
            timeout: Duration::from_millis(5),
            servers: vec!(String::from("localhost:8080")),
        };
        let agent = Agent::init(&cfg);
        let thread = tokio::spawn(async move {
            agent.wait().await;
            agent
        });
        sleep(Duration::from_millis(55));

        let agent = thread.await.ok().unwrap();
        assert_eq!(agent.role.load(Ordering::SeqCst), Role::Candidate);
    }

    #[tokio::test]
    async fn test_holds_election_once_candidate() {
        let cfg = AgentConfig{
            timeout: Duration::from_millis(5),
            servers: vec!(String::from("localhost:8080"), String::from("localhost:8081")),
        };

        let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
        let (mut tx, rx) = oneshot::channel::<i64>();
        let handle_election = move |_req: Request<Body>| -> Result<Response<Body>, Infallible> {
            drop(rx);
            Ok(Response::new(Body::from("Success")))
        };

        let make_service = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(handle_election))
        });
        let server = Server::bind(&addr).serve(make_service);
        let server_thread = tokio::spawn(async move {
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        });

        let agent = Agent::init(&cfg);
        let thread = tokio::spawn(async move {
            agent.wait().await;
            agent
        });
        sleep(Duration::from_millis(55));

        let agent = thread.await.ok().unwrap();
        assert_eq!(agent.role.load(Ordering::SeqCst), Role::Candidate);

        tx.closed().await;
    }
}