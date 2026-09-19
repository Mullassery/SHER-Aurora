# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in Aurora, please do not open a public GitHub issue. Instead, please email the maintainer directly at:

**mullassery@gmail.com**

Please include the following information in your report:

- A description of the vulnerability
- Steps to reproduce the issue
- The affected versions
- Any potential impact or workarounds

We will investigate all security reports and work with you to determine the severity and provide a patch if necessary.

## Supported Versions

This is a single-maintainer project with no formal LTS policy. Only the
latest release (currently `1.3.x` — see `Cargo.toml`'s
`[workspace.package] version`) receives fixes. Older releases are not
patched; upgrade to the latest tag if you need a fix.

## Security Best Practices

When using Aurora in your applications:

1. Keep Aurora and its dependencies updated to the latest versions
2. Review security advisories regularly
3. Follow secure coding practices in your applications
4. Report any security issues you discover responsibly

## Dependencies

`Cargo.lock` is committed (see `ROADMAP_HONEST.md` for why that wasn't
always true) so builds are reproducible. Every push and pull request runs
`rustsec/audit-check` against `Cargo.lock` in the `Security Audit` job of
[`.github/workflows/ci.yml`](.github/workflows/ci.yml) — this is a real,
running CI job, not an aspirational claim; check its status via the CI
badge in `README.md`.

To check for known vulnerabilities in the dependency tree yourself:

```bash
cargo install cargo-audit
cargo audit
```

## Acknowledgments

We appreciate security researchers and community members who responsibly disclose vulnerabilities. Depending on the severity and impact, we may publicly acknowledge your contribution in the release notes.

## More Information

For more information about secure coding practices, please see:

- OWASP Top 10: https://owasp.org/www-project-top-ten/
- Rust Security Advisory Database: https://rustsec.org/
- GNOME Security Guidelines: https://wiki.gnome.org/Projects/GNOME/Security_Advisories
