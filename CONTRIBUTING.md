# Contributing to Aurora

Thank you for your interest in contributing to Aurora. We welcome contributions from developers, designers, and accessibility experts to help make GNOME the most beautiful and accessible desktop environment on Linux.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps which reproduce the problem**
- **Provide specific examples to demonstrate the steps**
- **Describe the behavior you observed after following the steps**
- **Explain which behavior you expected to see instead and why**
- **Include screenshots and animated GIFs if possible**
- **Include your environment** (OS, Rust version, GTK4 version)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- **Use a clear and descriptive title**
- **Provide a step-by-step description of the suggested enhancement**
- **Provide specific examples to demonstrate the steps**
- **Describe the current behavior and why it's not sufficient**
- **Explain the expected behavior**
- **Provide links to design inspiration or references if applicable**

### Pull Requests

- Fill in the required template
- Follow the Rust styleguide
- Include appropriate test cases
- Update documentation as needed
- End all files with a newline

## Development Setup

### Prerequisites

- Rust 1.70+ (https://rustup.rs/)
- GTK4 development libraries
- libadwaita development libraries
- GLib development libraries

### Building Locally

```bash
git clone https://github.com/Mullassery/aurora.git
cd aurora

# Build all crates
cargo build --release

# Run tests
cargo test --lib

# Generate documentation
cargo doc --no-deps --open
```

### Project Structure

```
crates/
├── aurora-tokens/       # Design tokens (spacing, colors, motion)
├── aurora-typography/   # Typography engine with responsive scales
├── aurora-color/        # Color system and semantic tokens
├── aurora-motion/       # Animation engine with spring physics
├── aurora-icons/        # Icon system and font generation
├── aurora-sound/        # Sound design definitions
├── aurora-a11y/         # Accessibility layer (WCAG AAA)
├── aurora-core/         # Unified API over all subsystems
├── aurora-gtk/          # GTK4 component library
├── aurora-qt/           # Qt6 renderer (FFI bindings)
└── aurora-web/          # Web/WASM renderer

examples/
├── aurora_settings/     # Full settings application
├── aurora_files/        # File manager with DataTable
├── aurora_calendar/     # Calendar with Tabs
└── aurora_music/        # Music player with IconDock

docs/
├── API_REFERENCE.md     # Complete API documentation
├── ARCHITECTURE.md      # Technical architecture
├── COMPONENT_LIBRARY.md # Component specifications
└── (more documentation)
```

## Code Guidelines

### Rust Conventions

- Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use meaningful variable and function names
- Keep functions small and focused
- Avoid nested indentation beyond 3 levels
- Use early returns to reduce nesting

### Testing Requirements

All contributions must include tests:

- Unit tests for individual functions/modules
- Integration tests for component interactions
- Accessibility tests for UI components
- Test coverage must be 95%+ for new code

Run tests before submitting:

```bash
cargo test --lib
```

### Accessibility Requirements

All UI components must meet WCAG AAA standards:

- Color contrast ratio 7:1 minimum
- Full keyboard navigation support
- Screen reader compatibility
- Respect `prefers-reduced-motion`
- Proper semantic HTML/GTK4 structure

Use the accessibility testing tools:

```bash
# Check contrast ratios
cargo test --lib accessibility

# Verify keyboard navigation manually
cargo run --example aurora_settings
```

### Documentation Requirements

- Add doc comments to public functions/types
- Include examples in doc comments
- Update README.md if adding major features
- Update CHANGELOG.md with all changes
- Keep architecture documentation current

### Commit Messages

Write clear, descriptive commit messages:

```
Title (50 chars or less)

Detailed explanation of the change (72 chars per line)

- Use bullet points for multiple related changes
- Reference issues when applicable (#123)
- Explain why the change was made, not just what changed
```

Example:

```
Add DataTable component with sorting and pagination

Implement a new DataTable widget for displaying tabular data with:
- Column sorting (ascending/descending)
- Row selection (single and multi-select)
- Pagination with configurable page size
- WCAG AAA keyboard navigation
- 16 comprehensive unit tests
- Full documentation and examples

Fixes #456
```

## Getting Help

- Open an issue for questions
- Check existing documentation in `docs/`
- Read the architecture guide in CLAUDE.md
- Ask in GitHub Discussions

## Review Process

1. Submit a pull request following the template
2. Maintainers will review for:
   - Code quality and style
   - Test coverage and passing tests
   - Accessibility compliance
   - Documentation completeness
   - Design system consistency
3. Address review feedback with follow-up commits
4. Once approved, maintainer will merge the PR

## Release Process

Releases follow semantic versioning (MAJOR.MINOR.PATCH):

- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes (no new features)

Releases are published to crates.io and GitHub releases.

## License

By contributing to Aurora, you agree that your contributions will be licensed under its MIT/Apache 2.0 dual license.

---

Thank you for helping make Aurora the design system GNOME deserves!

For questions or concerns, reach out to Georgi Mammen Mullassery at mullassery@gmail.com.
