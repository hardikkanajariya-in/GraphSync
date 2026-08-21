# Security Policy

## Supported versions

| Version | Supported |
| --- | --- |
| latest release | yes |
| older releases | best effort |

## Reporting a vulnerability

Please do not report security vulnerabilities in public GitHub issues.

Instead, report them privately using **GitHub Security Advisories** for this repository, or contact the maintainers through a private channel if advisory access is unavailable.

Include:

- A clear description of the issue
- Steps to reproduce
- Potential impact
- Affected versions or platforms

## Response expectations

Maintainers will acknowledge valid reports as quickly as possible and work on a fix
or mitigation. We appreciate responsible disclosure.

## Security model notes

GraphSync is designed so that:

- File contents are not uploaded to the coordination API
- Auth tokens remain in the Rust backend
- Relative paths are validated to prevent directory traversal
- Sensitive values are not written to logs

If you discover a bypass of these protections, please report it promptly.
