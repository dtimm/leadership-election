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

    static test_cfg: AgentConfig = AgentConfig {
        timeout: Duration::from_millis(50),
    };

    #[tokio::test]
    async fn test_starts_as_follower() {
        let agent = Agent::init(&test_cfg);
        assert_eq!(agent.role.load(Ordering::SeqCst), Role::Follower);
        // assert!(agent.is_role(Role::Follower));
    }

    #[tokio::test]
    async fn test_switches_to_candidate_after_timeout() {
        let agent = Agent::init(&test_cfg);
        tokio::spawn(async move {
            agent.wait().await;
        });
        sleep(Duration::from_millis(55));

        assert_eq!(agent.role.load(Ordering::SeqCst), Role::Candidate);
    }
}