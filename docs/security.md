# Security invariants and release gates

The current scaffold is not production ready. The following are requirements, not claims that the implementation is complete.

## Required before untrusted jobs

- [ ] Agent enrollment uses one-time, short-lived tokens.
- [ ] Agent credentials are stored securely, can be rotated, and can be revoked.
- [ ] Jobs are authenticated, versioned, expiring, and bound to the intended agent.
- [ ] Public destination policy validates every DNS answer and the actual peer IP.
- [ ] DNS validation and connection use the same approved address to prevent rebinding.
- [ ] Redirects are disabled; if introduced, each redirect target is fully revalidated.
- [ ] TLS certificate and hostname verification are enabled.
- [ ] Request duration, response bytes, headers, redirects, concurrency, and queue size are bounded.
- [ ] No shell, plugin, or user-script execution exists.
- [ ] Results do not include response bodies or sensitive headers by default.
- [ ] Private-agent scope is explicit, narrow, locally enforced, and revocable.
- [ ] Network-level egress controls are documented for supported deployments.
- [ ] Fuzzing and regression tests cover URL parsing, IPv4/IPv6 edge cases, DNS rebinding, and protocol deserialization.

## Public destination policy

At minimum, public mode must reject:
- non-HTTP(S) schemes;
- URL credentials;
- disallowed ports;
- loopback, private, link-local, multicast, unspecified, reserved, and metadata IPs;
- hostnames that resolve to any prohibited address;
- redirects to destinations that have not been revalidated.

A hostname check alone is not enough. The executor must ensure the connected peer is an address approved by the policy. The current scaffold only provides baseline syntax checks and a limited IP-literal guard; it is not an SSRF defense.
