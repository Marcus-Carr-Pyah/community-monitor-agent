//! Baseline local policy validation.
//!
//! IMPORTANT: This is not yet a production SSRF defense. A production executor
//! must validate DNS answers and ensure the actual connected peer IP is one of
//! the approved addresses, preventing DNS rebinding and resolver time-of-check
//! / time-of-use issues.

use monitor_agent_protocol::{AgentMode, HttpRequestSpec, JobEnvelope, PROTOCOL_VERSION};
use std::net::IpAddr;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PolicyError {
    #[error("unsupported protocol version")]
    UnsupportedProtocol,
    #[error("unsupported HTTP method")]
    UnsupportedMethod,
    #[error("only HTTP and HTTPS URLs are allowed")]
    InvalidScheme,
    #[error("URL credentials are not allowed")]
    UrlCredentials,
    #[error("port is not allowed by baseline policy")]
    DisallowedPort,
    #[error("IP literal is prohibited by baseline policy")]
    ProhibitedIpLiteral,
    #[error("job limit exceeds local maximum")]
    LimitExceeded,
    #[error("redirects are disabled")]
    RedirectsDisabled,
    #[error("job is addressed to a different agent")]
    WrongAgent,
    #[error("private jobs require private agent mode")]
    ModeMismatch,
}

#[derive(Debug, Clone)]
pub struct LocalLimits {
    pub max_timeout_ms: u32,
    pub max_response_bytes: u64,
}

impl Default for LocalLimits {
    fn default() -> Self {
        Self {
            max_timeout_ms: 5_000,
            max_response_bytes: 1_048_576,
        }
    }
}

pub fn validate_url_syntax(url: &Url, mode: AgentMode) -> Result<(), PolicyError> {
    match url.scheme() {
        "http" | "https" => {}
        _ => return Err(PolicyError::InvalidScheme),
    }

    if !url.username().is_empty() || url.password().is_some() {
        return Err(PolicyError::UrlCredentials);
    }

    let port = url
        .port_or_known_default()
        .ok_or(PolicyError::DisallowedPort)?;
    let port_allowed = match mode {
        AgentMode::Public => matches!(port, 80 | 443),
        AgentMode::Private => matches!(port, 80 | 443),
    };
    if !port_allowed {
        return Err(PolicyError::DisallowedPort);
    }

    if let Some(host) = url.host_str() {
        if let Ok(ip) = host.parse::<IpAddr>() {
            if is_prohibited_ip_literal(ip) {
                return Err(PolicyError::ProhibitedIpLiteral);
            }
        }
    }
    Ok(())
}

fn is_prohibited_ip_literal(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            ip.is_private()
                || ip.is_loopback()
                || ip.is_link_local()
                || ip.is_broadcast()
                || ip.is_unspecified()
                || ip.is_multicast()
                || ip.octets()[0] == 0
                || ip.octets()[0] >= 224
        }
        IpAddr::V6(ip) => {
            ip.is_loopback()
                || ip.is_unspecified()
                || ip.is_multicast()
                || ip.is_unique_local()
                || ip.is_unicast_link_local()
        }
    }
}

pub fn validate_job(
    job: &JobEnvelope,
    expected_agent_id: uuid::Uuid,
    limits: &LocalLimits,
) -> Result<(), PolicyError> {
    if job.protocol_version != PROTOCOL_VERSION {
        return Err(PolicyError::UnsupportedProtocol);
    }
    if job.agent_id != expected_agent_id {
        return Err(PolicyError::WrongAgent);
    }
    if job.limits.timeout_ms == 0
        || job.limits.timeout_ms > limits.max_timeout_ms
        || job.limits.max_response_bytes > limits.max_response_bytes
    {
        return Err(PolicyError::LimitExceeded);
    }
    if job.limits.max_redirects != 0 {
        return Err(PolicyError::RedirectsDisabled);
    }
    if job.policy.mode == AgentMode::Private && job.policy.version == 0 {
        return Err(PolicyError::ModeMismatch);
    }
    validate_request(&job.request, job.policy.mode)
}

pub fn validate_request(request: &HttpRequestSpec, mode: AgentMode) -> Result<(), PolicyError> {
    if !matches!(request.method.as_str(), "GET" | "HEAD") {
        return Err(PolicyError::UnsupportedMethod);
    }
    validate_url_syntax(&request.url, mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use monitor_agent_protocol::HttpRequestSpec;

    fn request(raw: &str) -> HttpRequestSpec {
        HttpRequestSpec {
            method: "GET".to_string(),
            url: Url::parse(raw).expect("valid test URL"),
        }
    }

    #[test]
    fn accepts_basic_public_https_url_syntax() {
        assert!(
            validate_request(&request("https://example.com/health"), AgentMode::Public).is_ok()
        );
    }

    #[test]
    fn rejects_ftp_scheme() {
        assert_eq!(
            validate_request(&request("ftp://example.com/file"), AgentMode::Public),
            Err(PolicyError::InvalidScheme)
        );
    }

    #[test]
    fn rejects_url_credentials() {
        assert_eq!(
            validate_request(
                &request("https://user:pass@example.com/"),
                AgentMode::Public
            ),
            Err(PolicyError::UrlCredentials)
        );
    }

    #[test]
    fn rejects_nonstandard_port() {
        assert_eq!(
            validate_request(&request("https://example.com:8443/"), AgentMode::Public),
            Err(PolicyError::DisallowedPort)
        );
    }

    #[test]
    fn rejects_loopback_literal() {
        assert_eq!(
            validate_request(&request("http://127.0.0.1/"), AgentMode::Public),
            Err(PolicyError::ProhibitedIpLiteral)
        );
    }

    #[test]
    fn rejects_ipv6_loopback_literal() {
        assert_eq!(
            validate_request(&request("http://[::1]/"), AgentMode::Public),
            Err(PolicyError::ProhibitedIpLiteral)
        );
    }
}
