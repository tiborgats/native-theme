# Security Policy

## Supported versions

Because this crate is pre-1.0 (see [semver](https://semver.org)), only the
latest `0.x.y` release receives security fixes. When `0.(x+1).0` ships, the
previous minor is no longer supported.

| Version | Supported |
|---|---|
| Latest `0.5.x` | ✅ |
| Older `0.5.*` | ❌ |
| Any `0.4.x` or earlier | ❌ |

## Reporting a vulnerability

Please report security issues via
**[GitHub Private Vulnerability Reporting](https://github.com/tiborgats/native-theme/security/advisories/new)**.

Do not file a public issue and do not email directly. Private Vulnerability
Reporting keeps the disclosure confidential until a fix is ready and
published, and gives us a coordinated disclosure timeline that public issues
can't.

When you report, please include:

- A brief description of the issue
- A reproduction (minimal Rust snippet or step list)
- The affected crate name and version
- Whether you believe the issue is exploitable in practice

## Response expectations

This project is maintained by a single developer on a best-effort basis.
Expect an initial response within 7 days. Fix timing depends on severity and
complexity. Hard SLAs are not promised pre-1.0 — if you need a stronger
commitment, please say so in the initial Private Vulnerability Report.

## Scope

This policy covers the following crates published from this repository:

- [`native-theme`](https://crates.io/crates/native-theme)
- [`native-theme-build`](https://crates.io/crates/native-theme-build)
- [`native-theme-derive`](https://crates.io/crates/native-theme-derive)
- [`native-theme-gpui`](https://crates.io/crates/native-theme-gpui)
- [`native-theme-iced`](https://crates.io/crates/native-theme-iced)

Vulnerabilities in downstream dependencies (gpui, iced, serde, ashpd, etc.)
should be reported to those projects directly.

## Dependency auditing

[`cargo audit`](https://rustsec.org/) runs in CI via
[`rustsec/audit-check`](https://github.com/rustsec/audit-check) on every
pull request, flagging known advisories against locked dependencies before
they merge.
