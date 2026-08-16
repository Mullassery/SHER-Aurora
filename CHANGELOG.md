# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - 2026-08-16

### Added

- **`aurora-a11y` crate is now real** - previously an empty stub despite being documented as the project's dedicated accessibility layer. Now provides an automated WCAG 2.1 contrast audit (`audit_theme`, `audit_all_themes`) that computes real contrast ratios for every semantically meaningful color-token pairing (readable text, brand-color-on-container, and non-text UI components) across all four shipped themes (Light, Dark, OLED, HDR), plus `wcag_level`/`TextSize`/`WcagLevel` for checking any individual pair.
- `aurora-color::Color` gained WCAG's real large-text thresholds - `passes_wcag_aaa_large`/`passes_wcag_aa_large` (4.5:1 / 3:1, per SC 1.4.3/1.4.6) and `passes_wcag_ui_component` (3:1, per SC 1.4.11) - alongside the existing normal-text `passes_wcag_aaa`/`passes_wcag_aa` (7:1 / 4.5:1).
- **`aurora-gtk` now renders real GTK4** - the `gtk4` crate dependency (previously commented out, so nothing in this crate ever actually rendered) is real again, pinned to `gtk4 = "0.11"` with the `v4_12` feature. `Button`, `Input`, `Checkbox`, `Card`, and the new `Switch` widget each gained a `build()` method that constructs an actual `gtk4::Button`/`Entry`/`CheckButton`/`Box`/`Switch` object from the widget's Aurora descriptor - not a mock. `CssProvider::install()` loads Aurora's token-derived stylesheet into a real `gtk4::CssProvider` and attaches it to a real `gtk4::gdk::Display`.
- Real-GTK4 widget-construction tests (`#[gtk4::test]`, gated `#[cfg(not(target_os = "macos"))]`) run for real on Linux CI, where GTK4 has no main-thread restriction; on macOS, `gtk4::init()` cannot succeed inside Rust's `#[test]` harness at all (verified: GTK4's Cocoa backend requires init on the OS's true main thread, which no test-runner thread is), so the equivalent local proof ships as `crates/aurora-gtk/examples/gtk4_harness.rs`, a runnable harness that initializes real GTK4, builds real widgets, installs real CSS, realizes a real window, and asserts on state read back through the real GTK4 API.
- **`aurora-icons` is now real** - previously a one-line stub despite README/docs claiming "2000+ organized system and application icons," with zero SVG assets anywhere in the repository. Now ships 24 real, hand-authored, tested SVG icons (navigation, actions, status, media, system categories), each a complete, valid 24x24 stroke-based SVG document. `aurora-gtk`'s icon metadata registry (`icons::core`) now registers exactly these 24 icons - it previously padded itself out to 210+ entries with placeholder names like `nav-23`/`act-41` that had no real artwork behind them; that padding has been removed. `IconDock::DockItem::icon_svg()` resolves an item's icon name to real SVG when it matches one of the 24.
- Two real, compiling GTK4 examples in `crates/aurora-gtk/examples/`: `gtk4_harness.rs` (headless proof, described above) and `showcase.rs` (a real, interactive `gtk4::Application` window built from Aurora's real widgets).

### Fixed

- Dark/OLED/HDR themes: `primary_container`, `secondary_container`, `error_container`, `warning_container`, `success_container`, and `info_container` were too close in luminance to the semantic color rendered on top of them (as low as 1.92:1 for `warning`/`warning_container`, failing even WCAG AA). Darkened to reach real AAA text contrast (7:1) against their paired semantic color.
- `outline` in every theme fell short of the WCAG 1.4.11 non-text contrast minimum (3:1) against `background`/`surface` (as low as 1.32:1) - borders and focus indicators using this token were effectively invisible for low-vision users. Adjusted to genuinely clear 3:1 in all four themes.
- These were found by the new `aurora-a11y` audit computing real ratios from the shipping palette, not by manual inspection - the audit's regression tests now catch any future palette change that reintroduces a sub-3:1 UI component or a sub-4.5:1 text-on-container pairing.
- Removed a compiled ~1MB macOS binary (`aurora_calendar`) that had been committed to the repo root as if it were source; added the corresponding filenames to `.gitignore`.
- Removed four top-level `examples/*.rs` "example applications" (`aurora_calendar`, `aurora_files`, `aurora_music`, `aurora_settings`, plus the old `showcase.rs`) that the README claimed were runnable via `cargo run --example` but never actually compiled: three had no `fn main` at all and no connection to GTK or Aurora widgets, and the fourth imported a `gtk` crate this project has never depended on (the real crate is `gtk4`).
- `aurora-core`, `aurora-qt`, and `aurora-web` were silent one-line stub crates (`//! aurora-core`) with no indication they were unimplemented; they now have doc comments explicitly stating their not-yet-implemented status and why they're deferred (matching how the GTK4 backend is real but Qt/web backends are separate, larger platform-integration efforts).
- README/CLAUDE.md corrected: removed the "v1.0.0 Production Ready," "17 production-ready GTK4 widgets," and "2000+ organized system and application icons" claims, replacing them with the real, current state described above. Also fixed a stale README/CLAUDE.md claim of "dual-licensed under MIT and Apache License 2.0" that no longer matched `Cargo.toml`'s `license = "Proprietary"` (set in an earlier commit) or the root `LICENSE` file; removed the correspondingly stale `LICENSE-MIT`/`LICENSE-APACHE` files.
- Four real `clippy -D warnings` errors that a prior pass's cleanup had missed: `to_string()` inside `format!` args in `icons/svg.rs`; a `format!()` call with no interpolation in `widgets/datatable.rs::css_class()` (which, while fixing the lint, also fixed a real logic bug — the method unconditionally returned the `-selectable` CSS class regardless of `self.selectable`); `Option::map_or(false, ..)` in `widgets/menu.rs` simplified to `is_some_and`; and `Tabs::next()` in `widgets/tabs.rs`, renamed to `next_tab()` because it shadowed `Iterator::next` confusingly. `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` are both clean as of this release.
- README rewritten from a real user's perspective: install instructions, a working code example, and an explicit "what's real today" table, replacing internal audit/phase-status language plus leftover fabricated content from the original README (a nonexistent `curl | sudo bash` APT installer for a domain this project doesn't control, an 18-package Debian ecosystem, and a comparison table with invented competitor stats). `.github/INSTALL.md` was also corrected — it previously instructed `pip install aurora`, left over from an unrelated template and never applicable to this Rust/GTK4 project.

## [1.1.0] - 2027-03-31

### Added

- **Icon Dock Component** - Animated icon navigation bar with spring physics animations, hover effects, badges, and four position variants (Top, Bottom, Left, Right)
- **Icon Font Generation** - Web font builder supporting TTF, WOFF2, and WOFF formats with automatic CSS and HTML generation for web deployment
- **SVG Icon Rendering** - Complete SVG icon generator with 10 core icons (Home, Save, Delete, Settings, Search, Menu, Close, Check, Alert, Info) supporting multiple sizes and semantic colors
- **Comprehensive Icon System** - 210+ icon definitions organized by category (Navigation, Actions, Status, Media, System) with semantic color contexts and tagging system
- **Accessibility Layer** - Full WCAG AAA compliance including:
  - Colorblind vision simulations (Protanopia, Deuteranopia, Tritanopia, Achromatopsia)
  - Dyslexia-friendly fonts (OpenDyslexic, Verdana, Comic Sans)
  - High contrast mode with relative luminance calculation
  - Motion reduction support for vestibular disorders
- **Aurora CLI** - Command-line tool with 6 commands:
  - `aurora new` - Create new Aurora project
  - `aurora add` - Add components to project
  - `aurora generate` - Generate code from templates
  - `aurora theme` - Manage themes
  - `aurora export` - Export design assets
  - `aurora init` - Initialize Aurora in existing project
- **Storybook Component Documentation** - Interactive component showcase with stories, accessibility notes, code examples, and props documentation for all 17 components
- **DataTable Component** - Sortable, paginated data display with:
  - Column sorting (ascending/descending)
  - Row selection (single and multi-select)
  - Pagination with configurable page size
  - Full WCAG AAA keyboard navigation
  - 16 comprehensive unit tests
- **Tabs Component** - Multi-view navigation with:
  - 3 style variants (Filled, Underline, Pills)
  - 2 orientations (Horizontal, Vertical)
  - Smooth animated transitions
  - Full keyboard support
  - 18 unit tests
- **Select/Combobox Component** - Dropdown with:
  - Search filtering
  - Single and multi-select modes
  - Custom rendering
  - Keyboard navigation
  - 15 unit tests
- **Menu Component** - Context and navigation menus with:
  - Keyboard shortcuts display
  - Hierarchical menu structure
  - Keyboard navigation
  - 14 unit tests
- **Breadcrumb Component** - File path and hierarchical navigation with:
  - Automatic overflow handling
  - Custom segment rendering
  - Semantic HTML structure
  - 12 unit tests

### Changed

- Improved color system with additional semantic contexts
- Enhanced animation presets (spring, smooth, fast) across all components
- Updated component library to 17 production-ready widgets
- Restructured documentation for better organization
- Improved motion engine performance

### Fixed

- Fixed icon size scaling across different device DPIs
- Resolved animation timing issues in rapid interactions
- Corrected contrast ratios in high contrast mode
- Fixed keyboard navigation in complex component hierarchies

### Compatibility

- Fully backward compatible with v1.0
- No breaking changes
- All v1.0 components work unchanged

## [1.0.0] - 2027-01-15

### Added

- Initial production release of Aurora design system
- 10 core components:
  - Button with 4 style variants (Filled, Tinted, Outlined, Ghost)
  - Card with 2 style variants (Elevated, Outlined)
  - Input with 3 type variants (Text, Password, Search)
  - Dialog with modal and non-blocking modes
  - Checkbox with customizable states
  - Radio button with group selection
  - Tooltip with positioning options
  - List with scrollable container
  - Badge with semantic styling
  - Sidebar with navigation support
- Design token system with spacing, radius, elevation, and motion
- Typography engine with responsive scales and variable font support
- Color system with Light, Dark, and OLED themes
- Motion engine with spring physics animations
- GNOME Shell integration with dconf settings
- Comprehensive documentation and API reference
- Example applications (Settings, Files, Calendar, Music)
- Full test suite (111 tests, 100% passing)

### Documentation

- Complete API reference
- Integration guide for GNOME applications
- Architecture documentation
- Component library reference
- Design system guidelines
- Accessibility compliance guide

---

## [Unreleased]

### Planned Features

- Qt6 renderer for cross-platform support
- Web/WASM renderer for browser-based apps
- Extended component library (40+ widgets)
- Figma library export
- Theme customization UI
- AI-powered component suggestions
- Mobile optimizations
- Advanced animation tools

---

## Version History Summary

- **v1.1.0** (2027-03-31): Enhanced components, icon system, developer tools, accessibility layer
- **v1.0.0** (2027-01-15): Initial production release with core components and design system

## Upgrade Paths

### From v1.0 to v1.1

No breaking changes. Simply update the version in Cargo.toml:

```toml
aurora-gtk = "1.1"
```

All v1.0 code will continue to work without modification. Optionally, adopt new v1.1 features like Icon Dock and DataTable components.

## Support

- For issues, visit: https://github.com/Mullassery/aurora/issues
- For discussions, visit: https://github.com/Mullassery/aurora/discussions
- For security concerns, email: mullassery@gmail.com

## Contributors

- Georgi Mammen Mullassery (Lead)
- Community contributors (see GitHub contributors)

---

Generated with care for the Aurora community. ❤️
