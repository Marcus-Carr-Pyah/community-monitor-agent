//! Agent lifecycle and identity primitives. Credential storage and polling are TODOs.

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AgentIdentity {
    pub agent_id: Uuid,
}

impl AgentIdentity {
    pub fn new_for_local_development() -> Self {
        Self {
            agent_id: Uuid::new_v4(),
        }
    }
}
