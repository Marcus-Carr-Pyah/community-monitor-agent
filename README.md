# Community Monitor Agent

An open-source Rust agent for a community-driven HTTP monitoring network.

> **Early development:** This repository is a starter scaffold, not a production-ready monitoring agent. Do not run it with sensitive access or expose it to untrusted jobs until the security controls described in `docs/security.md` are implemented and reviewed.

## Goals

- Small, auditable Rust agent for Linux, Windows, and Docker.
- Outbound-only HTTPS communication with a monitoring control plane.
- Strictly bounded HTTP checks.
- Separate public-agent and customer-enrolled private-agent policies.
- Transparent, reproducible releases and community security review.

## Current status

The initial scaffold includes:

- A Rust workspace with protocol, policy, HTTP execution, core, and runtime crates.
- A versioned job envelope and structured result type.
- A conservative URL validation baseline.
- A CLI that validates a URL locally without making a network request.
- CI for formatting, linting, and tests.

It does **not** yet include server polling, agent enrollment, credential storage, lease management, production-grade DNS/IP pinning, or a signed update system.

## Quick start

Install a current stable Rust toolchain, then run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo run -p monitor-agent -- validate-url https://example.com/health
```

The `validate-url` command performs syntax and baseline policy validation only. It does not send a request.

## Project principles

1. Public agents must not access private, loopback, link-local, metadata, multicast, or otherwise prohibited destinations.
2. Private agents are opt-in and must be scoped to customer-authorized destinations.
3. The agent never executes shell commands or customer-provided scripts.
4. TLS certificate and hostname verification remain enabled.
5. Server-provided limits may tighten local limits, never relax them.
6. Jobs are versioned, expiring, authenticated, and bound to an enrolled agent.
7. Results are structured and bounded; response bodies are not uploaded by default.
8. Security restrictions are enforced locally, not only by the control plane.

See `docs/architecture.md`, `docs/security.md`, and `CONTRIBUTING.md`.

## License

Apache-2.0. See `LICENSE`.
