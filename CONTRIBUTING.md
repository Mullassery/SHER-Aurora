# Contributing to Aurora

Aurora is built by the community, for the community. We welcome contributions from developers, designers, documentation writers, and anyone passionate about improving GNOME.

---

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork: `git clone https://github.com/your-username/aurora.git`
3. Add upstream: `git remote add upstream https://github.com/Mullassery/aurora.git`
4. Create a feature branch: `git checkout -b feature/your-feature`

---

## Development Setup

### Prerequisites

- Rust 1.70+ ([install here](https://rustup.rs))
- System GTK4 (>= 4.12) development libraries — this workspace links
  against the real `gtk4` crate. **`libadwaita` is not a dependency** of
  any crate here; don't install it expecting it to be used.
- Build tools and pkg-config

### Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt install -y \
    libgtk-4-dev libglib2.0-dev \
    build-essential pkg-config rustc cargo
```

**Fedora/RHEL:**
```bash
sudo dnf install -y \
    gtk4-devel glib2-devel \
    gcc make pkg-config rust cargo
```

**Arch:**
```bash
sudo pacman -S gtk4 glib2 base-devel rust
```

**macOS:**
```bash
brew install gtk4
```

### Build Aurora

```bash
cd aurora

# Build
cargo build --workspace

# Run tests
cargo test --workspace

# Generate documentation
cargo doc --no-deps --open
```

There is no dconf/GSettings schema to install — `aurora-gtk` does not
integrate with the real GNOME Settings system today (see `CLAUDE.md` and
`ROADMAP_HONEST.md`). If you see a doc claiming otherwise, it's stale.

---

## Contribution Types

### Code Contributions

**Bug Fixes:**
1. Create a branch: `git checkout -b fix/issue-description`
2. Make minimal changes to fix the issue
3. Add tests that verify the fix
4. Run `cargo test` to ensure all tests pass
5. Commit with clear message: `fix: Brief description of fix`
6. Push and create a Pull Request

**New Features:**
1. Open an issue first to discuss the feature
2. Create a branch: `git checkout -b feature/feature-name`
3. Implement the feature with comprehensive tests
4. If the feature touches color tokens, run the `aurora-a11y` contrast audit (see Accessibility below)
5. Update documentation
6. Commit with clear message: `feat: Brief description of feature`
7. Push and create a Pull Request

**Refactoring:**
1. Only refactor code that has good test coverage
2. Ensure all tests pass before and after
3. Keep refactoring focused and minimal
4. Commit with message: `refactor: Brief description of changes`

### Documentation Contributions

Documentation improvements are highly valued:

- Clarify existing docs
- Add examples
- Fix typos and grammar
- Improve architecture documentation
- Create guides for specific use cases

**Documentation Files:**
- API docs: Inline Rust doc comments (`cargo doc --no-deps --open`)
- Architecture: `docs/architecture/README.md`
- Design philosophy and conventions: `CLAUDE.md`
- Historical/superseded docs: `docs/archive/` (do not treat as current)

### Design Contributions

If you're a designer:

- Propose new icons for the icon system
- Suggest UI improvements to examples
- Review visual consistency across components
- Propose accessibility improvements

Create an issue with screenshots or design mockups to discuss your ideas.

### Accessibility Contributions

- Review components for accessibility issues
- Test with screen readers (Orca on GNOME) — manually; there is no automated screen-reader test in CI today
- Test keyboard navigation — manually; there is no automated keyboard-navigation test in CI today
- Suggest high-contrast improvements
- Contribute an automated test for any of the above — this is a real, open gap (see `ROADMAP_HONEST.md`)

---

## Code Standards

### Rust Code

Follow Rust conventions:

```bash
# Format code
cargo fmt

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Both together
cargo fmt && cargo clippy --workspace --all-targets -- -D warnings
```

### Writing Tests

Every code change must include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_behavior() {
        // Arrange
        let input = ...;
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

There is no enforced coverage threshold; write tests that actually
exercise the behavior you changed rather than targeting a percentage.

### Documentation

Every public function must have a doc comment:

```rust
/// Brief description of what this function does.
///
/// More detailed explanation if needed. Include examples of usage.
///
/// # Examples
///
/// ```
/// let result = my_function(input);
/// assert_eq!(result, expected);
/// ```
///
/// # Panics
///
/// Panics if... (if applicable)
///
/// # Errors
///
/// Returns an error if... (if applicable)
pub fn my_function(input: T) -> Result<U, Error> {
    // implementation
}
```

### Accessibility

`aurora-a11y` mechanically checks one thing today: WCAG contrast ratios
for every semantic color-token pairing in every theme (7:1 normal text,
4.5:1 large text, 3:1 non-text UI components — the real WCAG thresholds,
not a single "AAA everywhere" bar). If you add or change a color token,
run `cargo test -p aurora-a11y` and make sure the audit still passes.
Keyboard navigation, screen-reader behavior, and reduced-motion support
are design goals for widgets but are **not** currently covered by any
automated test — treat claims about them as unverified until a real test
exists.

---

## Git Workflow

### Commit Messages

Write clear commit messages:

```
type: Brief description (50 chars max)

Longer explanation if needed. Wrap at 72 characters.
Explain what was changed and why, not how.

Related issues: Fixes #123, Related to #456
```

Types:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `style:` Code formatting (no logic changes)
- `refactor:` Code reorganization (no logic changes)
- `perf:` Performance improvement
- `test:` Test additions or fixes
- `chore:` Tooling, dependencies, etc.

### Pull Requests

Before submitting a PR:

1. Ensure your branch is up to date: `git fetch upstream && git rebase upstream/main`
2. Run all tests: `cargo test --workspace`
3. Format code: `cargo fmt`
4. Lint: `cargo clippy --workspace --all-targets -- -D warnings`
5. Verify accessibility

PR guidelines:
- One logical change per PR
- Keep PRs focused and manageable
- Include tests
- Update documentation
- Reference related issues
- Write clear PR description

---

## Review Process

All PRs go through review:

1. Automated checks (tests, linting, formatting — see `.github/workflows/ci.yml`)
2. Code review (functionality, quality, standards)
3. Accessibility review (contrast audit if color tokens changed; manual review otherwise)
4. Documentation review

---

## Design Principles

All contributions should follow Aurora's design philosophy:

**GTK4-native today** — widgets construct real `gtk4` objects rather than a platform-agnostic abstraction; deeper GNOME Shell/dconf/D-Bus integration is a design goal, not something built yet (see `CLAUDE.md`)

**Consistency over customization** — All GNOME apps follow the same design language

**Design systems over themes** — Tokens and semantic abstractions, not cosmetic themes

**Motion over decoration** — Every animation clarifies interaction and feedback

**Typography over visual effects** — Text is the primary interface; make it exceptional

**Accessibility over aesthetics** — measured contrast compliance by default, not an afterthought (see the Accessibility section above for exactly what's checked today)

**Polish over complexity** — Visual excellence over feature-richness

See CLAUDE.md for the full, current design guidance — it takes precedence over any older doc under `docs/archive/`.

---

## Reporting Bugs

Found a bug? Help us fix it:

1. Check if the issue already exists
2. Create a detailed issue report including:
   - What you did (steps to reproduce)
   - What you expected to happen
   - What actually happened
   - Your system (Ubuntu 24.04, GNOME 46, etc.)
   - Error messages or logs
   - Screenshots if applicable

---

## Feature Requests

Have an idea for Aurora? We'd love to hear it:

1. Check if the feature is already requested
2. Create an issue with:
   - Clear description of the feature
   - Why it's needed
   - How it would be used
   - Mockups or examples if possible

---

## Questions & Support

- GitHub Discussions: https://github.com/Mullassery/aurora/discussions
- Issues: https://github.com/Mullassery/aurora/issues

---

## Code of Conduct

Aurora is committed to providing a welcoming and inclusive environment for all contributors. Please be respectful and professional in all interactions.

---

## License

By contributing to Aurora, you agree that your contributions will be licensed under the project's license, [Apache License 2.0](LICENSE).

---

Thank you for contributing to Aurora! Your work makes GNOME more beautiful for everyone.
