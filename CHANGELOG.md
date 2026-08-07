# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - Unreleased

### Added

- **`aurora-a11y` crate is now real** - previously an empty stub despite being documented as the project's dedicated accessibility layer. Now provides an automated WCAG 2.1 contrast audit (`audit_theme`, `audit_all_themes`) that computes real contrast ratios for every semantically meaningful color-token pairing (readable text, brand-color-on-container, and non-text UI components) across all four shipped themes (Light, Dark, OLED, HDR), plus `wcag_level`/`TextSize`/`WcagLevel` for checking any individual pair.
- `aurora-color::Color` gained WCAG's real large-text thresholds - `passes_wcag_aaa_large`/`passes_wcag_aa_large` (4.5:1 / 3:1, per SC 1.4.3/1.4.6) and `passes_wcag_ui_component` (3:1, per SC 1.4.11) - alongside the existing normal-text `passes_wcag_aaa`/`passes_wcag_aa` (7:1 / 4.5:1).

### Fixed

- Dark/OLED/HDR themes: `primary_container`, `secondary_container`, `error_container`, `warning_container`, `success_container`, and `info_container` were too close in luminance to the semantic color rendered on top of them (as low as 1.92:1 for `warning`/`warning_container`, failing even WCAG AA). Darkened to reach real AAA text contrast (7:1) against their paired semantic color.
- `outline` in every theme fell short of the WCAG 1.4.11 non-text contrast minimum (3:1) against `background`/`surface` (as low as 1.32:1) - borders and focus indicators using this token were effectively invisible for low-vision users. Adjusted to genuinely clear 3:1 in all four themes.
- These were found by the new `aurora-a11y` audit computing real ratios from the shipping palette, not by manual inspection - the audit's regression tests now catch any future palette change that reintroduces a sub-3:1 UI component or a sub-4.5:1 text-on-container pairing.

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
