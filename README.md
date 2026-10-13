# SHER Aurora

A GNOME design system for Rust: design tokens, typography, color, motion, sound, icons, and a growing set of real GTK4 widgets — backed by an automated WCAG accessibility audit.

[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/Mullassery/SHER-Aurora/actions/workflows/ci.yml/badge.svg)](https://github.com/Mullassery/SHER-Aurora/actions/workflows/ci.yml)

## What it is

Aurora gives GNOME app developers a single, consistent source of truth for spacing, color, typography, motion, sound, and iconography — instead of every app inventing its own. It is a Rust workspace of design-token crates plus a GTK4 widget layer, where a widget listed as "renders on real GTK4" constructs a genuine `gtk4` crate object, not a mock or a CSS-only description.

Aurora is one member of the SHER family and is the planned primary shell on top of SHER-Display, but today it has zero Cargo-level dependency on any sibling and builds and functions standalone as a GTK/Qt/Web design-system toolkit.

## Features / Highlights

- **Design tokens** — spacing, radius, elevation, and motion as a consistent, unit-tested source of truth (`aurora-tokens`).
- **Typography** — responsive type scales with internationalization and script-aware adjustments (`aurora-typography`).
- **Color system** — semantic tokens (`surface`, `primary`, `success`, `error`, …) across four themes: Light, Dark, OLED, and HDR (`aurora-color`).
- **Motion** — spring-physics and easing math rather than linear easing (`aurora-motion`).
- **Sound** — semantic sound-event definitions (`aurora-sound`; no audio playback).
- **Accessibility audit** — `aurora-a11y` computes real WCAG contrast ratios across every token pairing in every theme (7:1 / 4.5:1 / 3:1 thresholds), and its regression tests fail if a future palette change reintroduces a sub-threshold pairing.
- **Icons** — 24 real, hand-authored 24×24 stroke-based SVGs across navigation, actions, status, media, and system (`aurora-icons`).
- **GTK4 widgets** — 5 of 17 planned widgets render as real `gtk4` objects via `.build()` today: Button, Input, Checkbox, Card, Switch.
- **Runnable proofs** — `gtk4_harness` (assertion-driven, constructs real GTK4 objects and reads state back) and `showcase` (a real interactive window).

## Who it's for / Use cases

- **Keeping a consistent design system across multiple GNOME apps** — one source of truth for spacing/color/typography/motion instead of each app reinventing it.
- **Using real GTK4 widgets with an automated WCAG accessibility audit built in**, rather than a design-only spec you re-implement per app.
- **Learning from a measured, not asserted, accessibility approach** — `aurora-a11y` computes real contrast ratios against the shipped palettes.

## What it is not

- **Not a full widget set yet.** Only 5 of 17 planned widgets construct real GTK4 objects. The other 12 (DataTable, Tabs, Menu, Dialog, List, Select, Sidebar, Badge, Breadcrumb, Radio, Tooltip, IconDock) have working Rust logic/tests but no `.build()` yet.
- **Not an application or desktop distribution.** There is no installer, no theme daemon, and no GNOME Shell extension; it is a library workspace.
- **Not libadwaita-based.** `libadwaita` is not a dependency of any crate — only `gtk4` and `glib`.
- **No system integration.** Nothing calls into `dconf`, `gsettings`, D-Bus, or any GNOME daemon; `aurora-gtk`'s `gnome/` modules model what such integration would look like but perform no OS I/O.
- **No runnable CLI.** `aurora-gtk/src/cli/` defines `Command`/`CommandType` data structures, but there is no `[[bin]]` target, no argument parser, and no `fn main` that reads `std::env::args()`.
- **Not multi-backend yet.** `aurora-qt` (Qt/QML), `aurora-web` (WASM/web), and `aurora-core` (unified facade) are placeholder crates with no implementation.
- **Not published to crates.io** and no tagged releases; use it as a pinned Cargo git dependency.
- **Not the claimed icon count in older docs.** The set is 24 real SVGs, not "2000+" (that older claim was never backed by assets).

## Project status

| Layer | Status |
|---|---|
| Design tokens, typography, color, motion, sound | Real, unit-tested |
| Accessibility (`aurora-a11y`) | Real automated WCAG contrast audit over every theme's tokens |
| Icons (`aurora-icons`) | 24 real hand-authored SVGs |
| GTK4 widgets — Button, Input, Checkbox, Card, Switch | Render as real `gtk4` objects via `.build()` |
| GTK4 widgets — DataTable, Tabs, Menu, Dialog, List, Select, Sidebar, Badge, Breadcrumb, Radio, Tooltip, IconDock | Styling/state logic only — no `.build()` yet |
| `aurora-qt`, `aurora-web`, `aurora-core` | Not implemented — placeholder crates |

- **Tests:** about 518 tests pass locally with zero clippy warnings and `cargo fmt --check` clean.
- **CI:** green across jobs after fixing a Linux compile error and a headless-display issue (tests run under `xvfb-run` on Linux).
- **Not started:** growing the GTK4 widget list; the `aurora-qt`/`aurora-web` backends; bridging widgets into SHER-Display's scene graph (a real, planned integration, blocked on both sides' roadmaps).

## Installation

Aurora is not yet published to crates.io and has no tagged releases — use it as a Cargo git dependency pinned to a commit (pick a `<commit-sha>` from the [commit history](https://github.com/Mullassery/SHER-Aurora/commits/main); pinning to a commit rather than `branch = "main"` keeps your build from changing on an unrelated push):

```toml
[dependencies]
aurora-gtk = { git = "https://github.com/Mullassery/SHER-Aurora", rev = "<commit-sha>" }
aurora-color = { git = "https://github.com/Mullassery/SHER-Aurora", rev = "<commit-sha>" }
aurora-tokens = { git = "https://github.com/Mullassery/SHER-Aurora", rev = "<commit-sha>" }
aurora-motion = { git = "https://github.com/Mullassery/SHER-Aurora", rev = "<commit-sha>" }
gtk4 = { version = "0.11", features = ["v4_12"] }
glib = "0.22"
```

You also need the system GTK4 library (>= 4.12), since `aurora-gtk` links against it:

```bash
# macOS
brew install gtk4

# Ubuntu / Debian
sudo apt update && sudo apt install -y libgtk-4-dev libglib2.0-dev build-essential pkg-config

# Fedora
sudo dnf install -y gtk4-devel glib2-devel gcc make pkg-config

# Arch
sudo pacman -S gtk4 glib2 base-devel
```

To build and hack on Aurora itself:

```bash
git clone https://github.com/Mullassery/SHER-Aurora.git
cd SHER-Aurora
cargo build --workspace
cargo test --workspace
```

## Quick start

This builds a real GTK4 window using Aurora's tokens, theme, and widgets:

```rust
use aurora_gtk::widgets::{Button, ButtonStyle, Card, CardStyle, Checkbox, Input, InputType};
use aurora_gtk::{CssProvider, Theme};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Orientation};

fn main() {
    let app = Application::builder()
        .application_id("com.example.myapp")
        .build();

    app.connect_activate(|app| {
        // Install Aurora's token-derived CSS onto the real display.
        if let Some(display) = gtk4::gdk::Display::default() {
            let css = CssProvider::new(Theme::Light).expect("failed to build Aurora CSS");
            css.install(&display);
        }

        let window = ApplicationWindow::builder()
            .application(app)
            .title("My Aurora App")
            .default_width(420)
            .default_height(400)
            .build();

        let root = gtk4::Box::new(Orientation::Vertical, 16);
        root.set_margin_top(24);
        root.set_margin_bottom(24);
        root.set_margin_start(24);
        root.set_margin_end(24);

        let card = Card::new().with_style(CardStyle::Elevated).with_spacing(12).build();
        card.append(&Button::new("Primary action").with_style(ButtonStyle::Filled).build());
        card.append(&Input::new(InputType::Email).with_placeholder("you@example.com").build());
        card.append(&Checkbox::new("Remember me").checked(true).build());

        root.append(&card);
        window.set_child(Some(&root));
        window.present();
    });

    app.run();
}
```

## Usage

Two complete, runnable examples ship in the repo:

```bash
# Non-interactive, assertion-driven proof that widgets construct real gtk4 objects
cargo run --example gtk4_harness -p aurora-gtk

# A real, interactive GTK4 window using the widgets above
cargo run --example showcase -p aurora-gtk
```

On macOS, `gtk4::init()` must run on the process's actual OS main thread, so widget-construction tests are `#[cfg(not(target_os = "macos"))]`; `gtk4_harness` is the macOS-compatible proof and runs via `cargo run` rather than inside the `#[test]` harness.

Icons:

```rust
use aurora_icons::{icon_svg, IconId};

let svg = icon_svg(IconId::Home).unwrap();
assert!(svg.starts_with("<svg"));
```

Development commands:

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

## Documentation

- [`CHANGELOG.md`](CHANGELOG.md) — corrected history of what has changed release over release
- [`ROADMAP_HONEST.md`](ROADMAP_HONEST.md) — the single source of truth for open issues and technical debt
- [`CONTRIBUTING.md`](CONTRIBUTING.md) — the full contribution workflow
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)
- [`SECURITY.md`](SECURITY.md) — how to report a vulnerability
- [`CLAUDE.md`](CLAUDE.md) — design intent and conventions for contributors
- [`docs/architecture/README.md`](docs/architecture/README.md) — architecture diagram
- [`docs/archive/`](docs/archive/) — older docs kept for history

## Contributing

Found a bug, or a claim in these docs that does not match reality? Please [open an issue](https://github.com/Mullassery/SHER-Aurora/issues) — this project has a specific history of documentation overstating what was actually built, and keeping that honest going forward is a priority. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the workflow and [`ROADMAP_HONEST.md`](ROADMAP_HONEST.md) for the current open-issue list.

## Related projects

- [SHER-KERNEL](https://github.com/Mullassery/SHER-KERNEL) — AI-native operating-system kernel with zero-trust security
- [SHER-Display](https://github.com/Mullassery/SHER-Display) — display server, compositor, and window manager for the SHER OS family
- [SHER-Graphics](https://github.com/Mullassery/SHER-Graphics) — native graphics architecture (software GPU + Vulkan backend)
- [SHER-INPUT](https://github.com/Mullassery/SHER-INPUT) — input subsystem that normalizes device activity into a canonical event stream
- [SHER-Aurora](https://github.com/Mullassery/aurora) — GNOME design system for Rust
- [SHER-Process-Explorer](https://github.com/Mullassery/SHER-Process-Explorer) — evidence-based Linux process explorer
- [Browse all projects](https://github.com/Mullassery?tab=repositories)

## License

This project is licensed under the [Apache License 2.0](LICENSE).

---

**Georgi Mammen Mullassery** — [github.com/Mullassery](https://github.com/Mullassery)
