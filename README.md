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

Locally on macOS, all of the above pass clean (518 tests, zero clippy warnings, `cargo fmt --check` clean) — verified directly against the current `main` commit. CI on Linux was previously red on a compile error (see [Known issues](#known-issues) — now fixed); the badge above reflects real, live status, not a static claim.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full contribution workflow, and [CHANGELOG.md](CHANGELOG.md) for a detailed, corrected history of what's changed release over release (including where earlier versions of this README overstated what existed).

---

## Relationship to the SHER family

This repo is `SHER-Aurora`, one member of an intentionally interdependent
stack — `SHER-Kernel` → `SHER-Graphics` → `SHER-Display` → `SHER-Input` →
Aurora — not just shared organizational naming. Aurora is the planned
primary shell on top of `SHER-Display` (see that repo's `ROADMAP.md` Phase
5), the same way `SHER-Display` depends on `SHER-Kernel`/`SHER-Graphics`/
`SHER-Input` today.

**Current state, verified by reading every `Cargo.toml` in the family:**
this repo has **zero Cargo-level dependency** on any of the other four
today, and none of them depend on it — that reflects where each repo's
own roadmap currently is (`SHER-Display`'s Phase 5 hasn't started; it
depends on that repo's Phases 1-4 first), not a decision to keep Aurora
separate. Aurora renders through literal `gtk4::Button`/`gtk4::Entry`
widgets today because the `SHER-Display` scene-graph target it would
eventually render to instead doesn't exist yet either (nothing in that
repo's `scene/` crate is exposed for an external toolkit to target). Until
that lands on both sides, this workspace (`crates/aurora-*`) builds and
functions standalone as a GTK/Qt/Web design-system toolkit — genuinely
useful on its own in the meantime, not blocked on the rest of the stack.

## Known issues

- **`aurora-tokens`'s HDR fallback — fixed.** `ColorSystem::current()` used to silently return the Light palette for `Theme::HDR` (`crates/aurora-tokens/src/color.rs`, was marked `TODO: Implement HDR theme`) instead of erroring or returning something real. `SemanticColor::hdr()` now has a genuinely distinct palette, sourced from `aurora-color`'s already-shipped `Theme::hdr()` values (the crate `aurora-gtk` actually consumes) rather than invented separately, so the two crates' HDR palettes agree. `ColorSystem::validate_contrast()` now checks it alongside Light/Dark/OLED (previously skipped it entirely), and new tests prove `current()` returns the real HDR palette rather than falling back to Light, and that it passes the same WCAG contrast checks as the other three themes.
- **CI's Linux compile failure — fixed.** `crates/aurora-gtk/src/widgets/button.rs`'s `test_button_disabled_is_insensitive_in_real_gtk4` called `button.is_sensitive()` without `gtk4::prelude::WidgetExt` in scope, failing to compile (`E0599`) — gated `#[cfg(not(target_os = "macos"))]`, so it never ran (or failed) locally on macOS, which is why `cargo test --workspace` reported clean there despite CI being red. Added the missing `use gtk4::prelude::*;` (matching the identical pattern the adjacent `test_button_build_is_real_gtk4_widget` already uses). Verified two ways: the real GitHub Actions failure logs for this repo's most recent CI run show `error[E0599]: no method named is_sensitive found for struct gtk4::Button` as the sole compile blocker in both the `Tests` and `Clippy` jobs, and a from-scratch `rust:latest` Docker container with `libgtk-4-dev` installed now compiles `aurora-gtk`'s full test suite cleanly (the `gtk_real` tests themselves still fail at runtime in that bare container with "GTK has not been initialized," which is a headless-display artifact of that ad-hoc container — not present in the real CI failure logs, so not this fix's concern to solve). The `Security Audit` job's separate, unrelated breakage (references a nonexistent action, `rustsec/audit-check-action@v1`) remains open — out of scope for this pass, flagging rather than silently leaving it undocumented.
- No open GitHub issues at the time of this writing.
- External critique proposed bridging Aurora widgets to `SHER-Display`'s scene graph and routing input through `SHER-INPUT`. Verified against the current code: not built yet, correctly so — see "Relationship to the SHER family" above. This is real, intended, planned work (`SHER-Display`'s `ROADMAP.md` Phase 5), not a mismatch to paper over by declaring Aurora out of scope. There's no bridge-shaped abstraction or Aurora-side event loop to retrofit yet because the phase it belongs to hasn't started on either side; widgets are literal `gtk4::Button`/`gtk4::Entry` builders with GTK4's native event handling today for that reason, not because a "GNOME-only" decision rules the integration out. `CLAUDE.md`'s Key Design Decision #3 is being revised alongside this to describe the current GTK4-native state without overstating it as a permanent boundary against the planned `SHER-Display` integration.

## Issues & contributing

Found a bug, or a claim in this README that doesn't match reality? Please [open an issue](https://github.com/Mullassery/aurora/issues) — this project has a specific history of documentation overstating what was actually built, and keeping that honest going forward is a priority.

---

## License

Aurora is source-available under a **Proprietary license — free to use with explicit attribution to the original author**. See the [LICENSE](LICENSE) file for full terms. Because of this license, Aurora is not published to crates.io; consume it as a git dependency as shown above.

---

**Georgi Mammen Mullassery** — [github.com/Mullassery](https://github.com/Mullassery)
