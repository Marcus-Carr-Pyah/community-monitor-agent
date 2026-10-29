# Contributing

Thanks for helping build a trustworthy community monitoring agent.

## Before opening a pull request

- Open an issue for significant changes so design and security implications can be discussed.
- Keep changes focused and include tests.
- Run `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace`.
- Never commit credentials, private endpoint details, customer data, or production tokens.
- Do not weaken destination policy, TLS verification, resource limits, or credential checks without an explicit security review.

## Security-sensitive changes

Changes involving URL parsing, DNS resolution, redirects, sockets, TLS, job authentication, credential storage, result reporting, or private-agent scope require extra scrutiny and targeted tests.

Please do not report exploitable vulnerabilities in public issues. Follow `SECURITY.md`.

## License and contributions

By submitting a contribution, you agree that it is provided under the repository's Apache-2.0 license. This project currently uses the Developer Certificate of Origin (DCO) model: sign off commits with `git commit -s` to certify that you have the right to submit the work under the project's license.

See https://developercertificate.org/ for the DCO text.
