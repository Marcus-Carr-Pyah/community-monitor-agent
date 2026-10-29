//! Versioned wire types shared by the agent and control-plane contract tests.
//! These types do not themselves authenticate or authorize a job.

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

pub const PROTOCOL_VERSION: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobEnvelope {
    pub protocol_version: u16,
    pub job_id: Uuid,
    pub assignment_id: Uuid,
    pub agent_id: Uuid,
    pub issued_at: String,
    pub expires_at: String,
    pub request: HttpRequestSpec,
    pub limits: JobLimits,
    pub policy: PolicyReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpRequestSpec {
    pub method: String,
    pub url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobLimits {
    pub timeout_ms: u32,
    pub max_response_bytes: u64,
    pub max_redirects: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyReference {
    pub mode: AgentMode,
    pub version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentMode {
    Public,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckResult {
    pub protocol_version: u16,
    pub job_id: Uuid,
    pub assignment_id: Uuid,
    pub agent_id: Uuid,
    pub outcome: CheckOutcome,
    pub http_status: Option<u16>,
    pub latency_ms: Option<u64>,
    pub response_bytes: u64,
    pub error_code: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckOutcome {
    Success,
    TargetFailure,
    AgentError,
}
