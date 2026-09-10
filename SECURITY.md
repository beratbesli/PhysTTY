# Security Policy

## Scope

PhysTTY is a local Rust terminal application. The supported security surface is
the code on `main` and the latest tagged release. It does not provide a hosted
service or accept network requests.

## Reporting a vulnerability

Please report security issues privately through GitHub's **Report a
vulnerability** flow on this repository. Do not include exploit details,
credentials, or other sensitive data in a public issue. If private reporting is
not available for your account, open a public issue with only a non-sensitive
summary and ask the maintainer for a private channel.

Useful reports include the affected commit or release, terminal/platform
details, reproduction steps, and the expected versus observed behavior.

## Response

Reports are acknowledged as soon as practical, investigated against the
current `main` branch, and coordinated with the reporter before public
disclosure. Because PhysTTY is a local prototype, terminal capability and
platform-specific behavior may affect severity and reproducibility.
