# Aurora

**A GNOME design system for Rust: design tokens, typography, color, motion, and a growing set of real GTK4 widgets — all backed by an automated WCAG accessibility audit.**

[![CI](https://github.com/Mullassery/aurora/actions/workflows/ci.yml/badge.svg)](https://github.com/Mullassery/aurora/actions/workflows/ci.yml)

Aurora gives GNOME app developers a single, consistent source of truth for spacing, color, typography, motion, sound, and iconography — instead of every app inventing its own. Where a widget is listed below as "renders on real GTK4," it means exactly that: it constructs a genuine `gtk4` crate object, not a mock or a CSS-only description.

---

## What's real today

| Layer | Status |
|---|---|
| Design tokens (spacing, radius, elevation) | Real, unit-tested (`aurora-tokens`) |
| Typography (responsive scales, i18n, script detection) | Real, unit-tested (`aurora-typography`) |
| Color (Light/Dark/OLED/HDR themes, semantic tokens) | Real, unit-tested (`aurora-color`) |
| Motion (spring physics, easing) | Real, unit-tested (`aurora-motion`) |
| Sound (semantic feedback definitions) | Real, unit-tested (`aurora-sound`) |
| Accessibility (`aurora-a11y`) | Real automated WCAG contrast audit over every theme's color tokens |
| Icons (`aurora-icons`) | 24 real, hand-authored SVG icons (navigation, actions, status, media, system) — not 2000+, see [Icons](#icons) below |
| GTK4 widgets — Button, Input, Checkbox, Card, Switch | Render as real `gtk4` objects via `.build()` |
| GTK4 widgets — DataTable, Tabs, Menu, Dialog, List, Select, Sidebar, Badge, Breadcrumb, Radio, Tooltip, IconDock | Styling/state logic only — no `.build()` yet, do not construct real GTK4 objects |
| `aurora-qt` (Qt/QML backend), `aurora-web` (WASM/web backend), `aurora-core` (unified facade) | Not implemented — placeholder crates for future work |

That's 5 of 17 planned widgets rendering on real GTK4 right now. The rest have working Rust logic (state, styling rules, tests) but nothing that touches the screen yet — growing that list is the main open work in this project.

---

## Install

Aurora is not yet published to crates.io (see [License](#license)). Use it as a Cargo git dependency:

```toml
[dependencies]
aurora-gtk = { git = "https://github.com/Mullassery/aurora", tag = "v1.2.0" }
aurora-color = { git = "https://github.com/Mullassery/aurora", tag = "v1.2.0" }
aurora-tokens = { git = "https://github.com/Mullassery/aurora", tag = "v1.2.0" }
aurora-motion = { git = "https://github.com/Mullassery/aurora", tag = "v1.2.0" }
gtk4 = { version = "0.11", features = ["v4_12"] }
glib = "0.22"
```

You'll also need the system GTK4 library (>= 4.12) installed, since `aurora-gtk` links against it:

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

Or, to build and hack on Aurora itself:

```bash
git clone https://github.com/Mullassery/aurora.git
cd aurora
cargo build --workspace
cargo test --workspace
```

---

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

Two complete, runnable versions of this ship in the repo itself:

```bash
# Non-interactive, assertion-driven proof that these widgets construct real
# gtk4 objects (initializes real GTK4, builds real widgets, installs real
# CSS, realizes a real window, and asserts on state read back through the
# real GTK4 API):
cargo run --example gtk4_harness -p aurora-gtk

# A real, interactive GTK4 window using the widgets above:
cargo run --example showcase -p aurora-gtk
```

(On macOS, `gtk4::init()` must run on the process's actual OS main thread, which is why widget-construction tests are `#[cfg(not(target_os = "macos"))]` in this repo — `gtk4_harness.rs` is the macOS-compatible proof, and it runs for real via `cargo run`, not inside the `#[test]` harness.)

---

## Icons

`aurora-icons` ships **24 real, hand-authored SVG icons** across five categories (navigation, actions, status, media, system) — each a complete, valid 24×24 stroke-based SVG document, not a placeholder. Earlier project documentation claimed "2000+ icons"; that was never backed by actual SVG assets and has been corrected. Growing this set is real future work, tracked honestly rather than pre-claimed.

```rust
use aurora_icons::{icon_svg, IconId};

let svg = icon_svg(IconId::Home).unwrap();
assert!(svg.starts_with("<svg"));
```

---

## Themes & accessibility

Four themes ship today — Light, Dark, OLED, HDR — each defined as semantic color tokens (`surface`, `primary`, `success`, `error`, etc.), never raw hex values in application code. `aurora-a11y` runs an automated WCAG contrast audit over every token pairing in every theme (not a manual claim): normal reading text, large text, and non-text UI components are checked against their real WCAG thresholds (7:1 / 4.5:1 / 3:1), and the audit's own regression tests fail if a future palette change reintroduces a sub-threshold pairing.

---

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

All of the above pass clean as of this release (515 tests, zero clippy warnings, `cargo fmt --check` clean).

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full contribution workflow, and [CHANGELOG.md](CHANGELOG.md) for a detailed, corrected history of what's changed release over release (including where earlier versions of this README overstated what existed).

---

## Known issues

- `aurora-tokens`'s standalone `ColorSystem::current()` has an unfinished HDR branch that silently falls back to the Light palette (`crates/aurora-tokens/src/color.rs`, marked `TODO: Implement HDR theme`). This code path is not what the shipped `aurora-color` crate uses — `aurora-color`'s own `ColorSystem`/`Theme` (the one referenced above and by `aurora-gtk`) has a real, distinct HDR palette — but the dead branch in `aurora-tokens` is still worth fixing or removing to avoid future confusion.
- No open GitHub issues at the time of this writing.

## Issues & contributing

Found a bug, or a claim in this README that doesn't match reality? Please [open an issue](https://github.com/Mullassery/aurora/issues) — this project has a specific history of documentation overstating what was actually built, and keeping that honest going forward is a priority.

---

## License

Aurora is source-available under a **Proprietary license — free to use with explicit attribution to the original author**. See the [LICENSE](LICENSE) file for full terms. Because of this license, Aurora is not published to crates.io; consume it as a git dependency as shown above.

---

**Georgi Mammen Mullassery** — [github.com/Mullassery](https://github.com/Mullassery)
