# Aurora

A Rust workspace providing a GNOME-facing design system — design tokens,
typography, color, motion, sound, accessibility auditing, and a growing
set of real GTK4 widgets. This file is guidance for anyone (human or AI)
working in this codebase. For current, verified project status, always
defer to the root [`README.md`](README.md), [`CHANGELOG.md`](CHANGELOG.md),
and [`ROADMAP_HONEST.md`](ROADMAP_HONEST.md) over anything below — this
file describes design intent and conventions, not a status report.

**Do not add marketing language to this repo's docs** ("production
ready", "most polished", "enterprise-grade", etc.) unless it is backed by
a runnable command whose output you have actually checked. This project's
git history contains multiple corrections of exactly this kind of
overclaiming (see `CHANGELOG.md` and `docs/archive/README.md`) — don't
reintroduce it.

## What this actually is (and isn't)

- A library, not an application or a desktop distribution. There is no
  installer, no theme daemon, no GNOME Shell extension.
- GTK4-only today. `libadwaita` is **not** a dependency of any crate in
  this workspace (only `gtk4` and `glib`). Do not write or accept docs
  that claim libadwaita integration.
- No system integration. Nothing in this workspace calls into `dconf`,
  `gsettings`, D-Bus, or any GNOME daemon. `crates/aurora-gtk/src/gnome/`
  (`dconf.rs`, `notifications.rs`, `observer.rs`, `settings_panel.rs`)
  defines Rust data structures that *model* what such integration would
  look like (e.g. `DConfSchema::schema_xml()` returns a hardcoded XML
  string) — none of it performs real I/O against the OS. See
  `ROADMAP_HONEST.md` before describing these modules as "integration."
- `crates/aurora-gtk/src/cli/` defines `Command`/`CommandType` data
  structures matching an imagined `aurora new/add/generate/theme/export/
  init` CLI. There is no `[[bin]]` target anywhere in this workspace, no
  `clap` (or any arg-parsing) dependency, and no `fn main` that reads
  `std::env::args()`. It is not a runnable CLI.
- Icon set is real but small: 24 hand-authored SVGs in `aurora-icons`, not
  "1000+" or "2000+" (that number appears in archived docs and was never
  backed by real assets).
- Not published to crates.io. Consumed as a git dependency only (see
  README `Install` section).

## Workspace structure

```
crates/
├── aurora-tokens/       # Spacing/radius/elevation/motion/color tokens — real, tested
├── aurora-typography/   # Type scales, i18n/script-aware adjustments — real, tested
├── aurora-color/        # Color system, semantic tokens, 4 themes — real, tested
├── aurora-motion/       # Spring-physics/easing math — real, tested
├── aurora-icons/        # 24 real SVG icons — real, tested
├── aurora-sound/        # Semantic sound-event *definitions* (no audio playback) — real, tested
├── aurora-a11y/         # Automated WCAG contrast audit over color tokens — real, tested
├── aurora-gtk/          # GTK4 widgets, CSS provider, gnome::*/cli data models (see above)
├── aurora-core/         # Unimplemented — stub crate, no facade logic written
├── aurora-qt/           # Unimplemented — stub crate, no Qt/QML code written
└── aurora-web/          # Unimplemented — stub crate, no WASM/web code written
```

Dependency direction: `aurora-gtk` depends on `aurora-tokens`,
`aurora-typography`, `aurora-motion`, `aurora-color`, `aurora-sound`,
`aurora-icons`. `aurora-a11y` depends on `aurora-color`. The other token
crates (`aurora-tokens`, `aurora-typography`, `aurora-color`,
`aurora-motion`, `aurora-icons`, `aurora-sound`) have no dependencies on
each other. See [`docs/architecture/README.md`](docs/architecture/README.md)
for a diagram.

## Design principles actually reflected in the code

- **Token-driven color** — application code consumes semantic tokens
  (`surface`, `primary`, `success`, `error`, …), never raw hex. Enforced
  by convention in `aurora-color`/`aurora-tokens`, not by a compiler
  check.
- **Four real themes** — Light, Dark, OLED, HDR, each with its own
  contrast-audited palette (`aurora-a11y`).
- **Spring-physics motion**, not linear easing, in `aurora-motion`.
- **Accessibility is measured, not asserted** — `aurora-a11y` computes
  real WCAG contrast ratios against the shipped palettes and fails its
  own tests if a future palette regresses below threshold. It does not
  check keyboard navigation, screen-reader behavior, or anything outside
  color contrast — don't describe it as broader than that.

## Before claiming something works

1. Run the actual command (`cargo test --workspace`, `cargo clippy
   --workspace --all-targets -- -D warnings`, `cargo fmt --check`) and
   quote its real output/exit code.
2. If a widget's status is being described, check whether it has a
   `.build()` method that constructs a real `gtk4` object — the README's
   "What's real today" table is the source of truth for this, keep it in
   sync with any widget you add or change.
3. If you find a doc making a claim you can't verify against the code,
   fix the doc (or move it to `docs/archive/` with a note) rather than
   propagating the claim further.

## License

Apache License 2.0 — see the root `LICENSE` file and `Cargo.toml`'s
`[workspace.package] license = "Apache-2.0"`. (This repo was previously
marked "Proprietary" for a period, then relicensed to Apache-2.0; if you
find a doc anywhere still saying "Proprietary" or "MIT/Apache dual
license," that doc is stale — fix it.)

---

**Georgi Mammen Mullassery** | GitHub: Mullassery | Email: mullassery@gmail.com
