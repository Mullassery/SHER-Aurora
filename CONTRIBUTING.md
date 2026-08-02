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
- GTK4 development libraries
- Libadwaita
- GLib
- Build tools and pkg-config

### Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt install -y \
    libgtk-4-dev libadwaita-1-dev libglib2.0-dev \
    build-essential pkg-config rustc cargo
```

**Fedora/RHEL:**
```bash
sudo dnf install -y \
    gtk4-devel libadwaita-devel glib2-devel \
    gcc make pkg-config rust cargo
```

**Arch:**
```bash
sudo pacman -S gtk4 libadwaita glib2 base-devel rust
```

### Build Aurora

```bash
cd aurora

# Register Aurora with GNOME
sudo cp crates/aurora-gtk/schemas/org.gnome.desktop.interface.aurora.gschema.xml \
    /usr/share/glib-2.0/schemas/
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/

# Build
cargo build --release

# Run tests
cargo test --lib

# Generate documentation
cargo doc --no-deps --open
```

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
4. Ensure WCAG AAA accessibility compliance
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
- User guides: `docs/`
- API docs: Inline Rust doc comments
- Architecture: `docs/APT_DISTRIBUTION_ARCHITECTURE.md`
- Design philosophy: `CLAUDE.md`

### Design Contributions

If you're a designer:

- Propose new icons for the icon system
- Suggest UI improvements to examples
- Review visual consistency across components
- Propose accessibility improvements

Create an issue with screenshots or design mockups to discuss your ideas.

### Accessibility Contributions

Aurora must maintain WCAG AAA compliance:

- Review components for accessibility issues
- Test with screen readers (Orca on GNOME)
- Test keyboard navigation
- Suggest high-contrast improvements
- Test with colorblind simulations

---

## Code Standards

### Rust Code

Follow Rust conventions:

```bash
# Format code
cargo fmt

# Lint
cargo clippy --all-targets --all-features

# Both together
cargo fmt && cargo clippy --all-targets --all-features
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

Test coverage targets: 95%+

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

All UI components must be accessible:

- Semantic HTML/GTK (proper widget types)
- ARIA labels for screen readers
- High contrast support (4.5:1 minimum ratio)
- Keyboard navigation (no mouse required)
- Reduced motion support (respect prefers-reduced-motion)
- Color not sole differentiator

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
2. Run all tests: `cargo test --lib`
3. Format code: `cargo fmt`
4. Lint: `cargo clippy --all-targets --all-features`
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

1. Automated checks (tests, linting, formatting)
2. Code review (functionality, quality, standards)
3. Accessibility review (WCAG AAA compliance)
4. Documentation review

---

## Design Principles

All contributions should follow Aurora's design philosophy:

**GNOME-native integration** — Deep integration with GNOME, not platform-agnostic design

**Consistency over customization** — All GNOME apps follow the same design language

**Design systems over themes** — Tokens and semantic abstractions, not cosmetic themes

**Motion over decoration** — Every animation clarifies interaction and feedback

**Typography over visual effects** — Text is the primary interface; make it exceptional

**Accessibility over aesthetics** — WCAG AAA compliance by default, not an afterthought

**Polish over complexity** — Visual excellence over feature-richness

**libadwaita integration** — Build on GNOME's modern toolkit, not around it

See CLAUDE.md for full design philosophy.

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

By contributing to Aurora, you agree that your contributions will be licensed under the same license as the project (MIT/Apache 2.0).

---

Thank you for contributing to Aurora! Your work makes GNOME more beautiful for everyone.
