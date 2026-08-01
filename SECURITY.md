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

Aurora uses semantic versioning (MAJOR.MINOR.PATCH):

| Version | Status | Support |
|---------|--------|---------|
| 1.1.x | Current | Full support |
| 1.0.x | Previous | Limited support (critical fixes only) |
| < 1.0 | Deprecated | No support |

## Security Best Practices

When using Aurora in your applications:

1. Keep Aurora and its dependencies updated to the latest versions
2. Review security advisories regularly
3. Use Aurora's WCAG AAA accessibility features to protect user data
4. Follow secure coding practices in your applications
5. Report any security issues you discover responsibly

## Dependencies

Aurora's dependencies are audited regularly using `cargo audit`. All dependencies are tracked in `Cargo.lock` for reproducible builds.

To check for known vulnerabilities in the dependency tree:

```bash
cargo audit
```

## Acknowledgments

We appreciate security researchers and community members who responsibly disclose vulnerabilities. Depending on the severity and impact, we may publicly acknowledge your contribution in the release notes.

## More Information

For more information about secure coding practices, please see:

- OWASP Top 10: https://owasp.org/www-project-top-ten/
- Rust Security Advisory Database: https://rustsec.org/
- GNOME Security Guidelines: https://wiki.gnome.org/Projects/GNOME/Security_Advisories
