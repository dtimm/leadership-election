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

    fn run(&mut self) {
        let swap = async {
            sleep(self.timeout);
            self.role = Role::Candidate;
        };

        tokio::spawn(swap);
    }

    fn is_role(&self, role: Role) -> bool {
        self.role == role
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
        let mut agent = Agent::init(&test_cfg);
        agent.run();
        assert!(agent.is_role(Role::Follower));
    }

    #[test]
    fn test_switches_to_candidate_after_timeout() {
        let mut agent = Agent::init(&test_cfg);
        agent.run();
        sleep(Duration::from_millis(55));

        assert!(agent.is_role(Role::Candidate));
    }
}