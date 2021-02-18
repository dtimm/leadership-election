use std::time::Duration;
use std::thread::sleep;

#[derive(PartialEq)]
enum Role {
    Follower,
    Candidate,
    Leader,
}

struct Agent {
    role: Role,
    timeout: Duration,
}

struct AgentConfig {
    timeout: Duration,
}

impl Agent {
    fn init(cfg: &AgentConfig) -> Agent {
        let agent = Agent {
            role: Role::Follower,
            timeout: cfg.timeout,
        };

        return agent
    }

    fn is_role(self, role: Role) -> bool {
        self.role == role
    }

    async fn chill(&mut self) {
        sleep(self.timeout);
        if self.is_role(Role::Follower) {
            self.role = Role::Candidate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static test_cfg: AgentConfig = AgentConfig {
        timeout: Duration::from_millis(50),
    };

    #[test]
    fn test_starts_as_follower() {
        let agent = Agent::init(&test_cfg);
        assert!(agent.is_role(Role::Follower));
    }

    #[test]
    fn test_switches_to_candidate_after_timeout() {
        let agent = Agent::init(&test_cfg);
        sleep(Duration::from_millis(55));

        assert!(agent.is_role(Role::Candidate));
    }
}