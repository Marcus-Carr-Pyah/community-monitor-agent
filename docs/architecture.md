# Architecture

## Responsibilities

The agent is a client of the monitoring control plane. It initiates outbound HTTPS connections; it does not expose an inbound listening port.

- `protocol`: versioned request/job/result data structures.
- `policy`: conservative local policy validation before execution.
- `http-executor`: bounded HTTP execution. The first version intentionally does not follow redirects.
- `agent-core`: configuration and lifecycle primitives.
- `runtime`: CLI and future polling/heartbeat/result-reporting loop.

The Rails control plane owns users, organizations, monitors, scheduling, job leases, incidents, and contribution credits. It must not be trusted to override local agent safety limits.

## Trust modes

### Public agent

Can execute only checks for public destinations after robust validation of DNS answers and the actual connected peer address. It must deny loopback, private, link-local, multicast, reserved, unspecified, and cloud metadata destinations for both IPv4 and IPv6. DNS rebinding must be addressed by resolving, validating, and connecting to an approved address without a second uncontrolled resolution.

### Private agent

Must be explicitly enrolled by an organization and bound to a narrow, customer-approved destination policy. It must not be treated as a public probe. Private mode does not mean unrestricted access to internal networks.

## Initial scope

The starter scaffold does not poll the control plane. It provides types, local validation, and a CLI validation command. Implement transport, identity, secure DNS/IP pinning, lease state, and result submission only with focused tests.
