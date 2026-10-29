# Security Policy

## Status

This project is an early scaffold and is not yet suitable for production use.

## Reporting a vulnerability

Please do not disclose suspected exploitable vulnerabilities in public issues. Until a private security contact is configured, open a GitHub Security Advisory draft if the repository has GitHub Security Advisories enabled. Otherwise, contact the repository maintainers privately through the contact method listed in the repository profile.

Include affected versions/commits, impact, reproduction steps, and any suggested mitigation. Avoid including real credentials or sensitive customer data.

## Security design priorities

- SSRF and DNS rebinding prevention.
- Strict public/private agent separation.
- Safe redirect handling (disabled by default).
- Hard request, response, concurrency, and time limits.
- Credential enrollment, rotation, and revocation.
- Job integrity, expiry, agent binding, and replay resistance.
- Tenant isolation in the control plane.
- Signed release artifacts and a secure update process.

Do not assume the current starter validation is sufficient for production.
