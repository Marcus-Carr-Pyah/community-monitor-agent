//! HTTP execution will be implemented only after safe DNS/IP pinning is designed.
//! This crate intentionally performs no network requests in the initial scaffold.

use monitor_agent_protocol::HttpRequestSpec;
use monitor_agent_policy::{validate_request, PolicyError};

pub fn validate_check(
    request: &HttpRequestSpec,
) -> Result<(), PolicyError> {
    validate_request(request, monitor_agent_protocol::AgentMode::Public)
}
