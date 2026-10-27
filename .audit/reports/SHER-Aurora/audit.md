# Audit Report: SHER-Aurora

**Audit Date:** 2026-10-26  
**Repository Path:** `/Users/georgimullassery/Desktop/Projects/SHER-Aurora`  
**Audit Type:** Read-only systems design audit  
**Version:** 1.3.0 (workspace)  
**Branch:** `main`  
**Source Size:** ~15,950 Rust LOC; 11 workspace crates (8 default members); 525 tests

## Executive Summary

SHER-Aurora is a GNOME-oriented Rust design system: token, typography, color, motion, sound, icon, and accessibility crates plus a GTK4 widget layer. The non-widget layers are real and unusually well tested for a design system: WCAG contrast is computed from actual token pairings rather than asserted, themes are generated from tokens, and the GTK harness constructs real `gtk4` objects. The project is honest that most widgets are styling/state logic only, and that the Qt/Web/core backends are placeholders.

The main issues are in the input bridge and release truth. `attach_to_glib` clones the dispatcher inside every timeout tick, so pointer/window-state mutations do not persist across ticks. The background input worker has no shutdown path. `poll_and_dispatch` drains until empty, so a continuous event source can starve the GTK main loop. Documentation also contradicts itself about widget count (5, 8, and 7 are all implied) and about “zero Cargo-level dependency on any sibling,” which is false when the optional `sher-input` feature is enabled.

## Purpose and Positioning

- **What it is:** a Rust GNOME design system with real design tokens, accessibility checks, and a growing GTK4 widget layer.
- **Who it is for:** GNOME/GTK Rust application developers who want a coherent token-driven look and an automated accessibility baseline.
- **What it is not:** a complete widget set, a desktop shell, an application framework, a Qt/Web implementation, or a published crates.io release.
- **Differentiator:** measured WCAG contrast across every theme/token pairing plus real GTK4 object construction, not CSS-only mockups.

## Architecture

```text
aurora-tokens ──> aurora-color / typography / motion / sound / icons
       │
       ├──> aurora-a11y (WCAG contrast audit)
       │
       └──> aurora-gtk
              CSS/theme provider
              ThemeListener (GTK settings signals)
              widgets (real .build() implementations + styling/state-only widgets)
              input bridge (optional SHER-Input feature)
              storybook / harness / showcase

aurora-core, aurora-qt, aurora-web: placeholder crates, not default members
```

### GTK layer

```text
Theme / tokens
  -> CssProvider
  -> installed on GdkDisplay
  -> widgets read semantic classes/tokens
  -> ThemeListener reacts to dark-theme/theme-name notify signals
```

### Input bridge

```text
InputSource (generic or SHER-Input ring)
  -> AuroraInputDispatcher
  -> EV_KEY: send_key
  -> EV_ABS: update pointer x/y and send_pointer_move
  -> EV_REL: currently ignored
```

## Repository Layout

```text
crates/aurora-tokens/       spacing, radius, elevation, motion, color
crates/aurora-color/        semantic palettes and themes
crates/aurora-typography/   responsive type scales
crates/aurora-motion/       easing/spring math
crates/aurora-sound/        semantic sound definitions
crates/aurora-icons/        24 hand-authored SVGs
crates/aurora-a11y/         WCAG contrast computation/regression tests
crates/aurora-gtk/          CSS, theme listener, widgets, input bridge
crates/aurora-core/         placeholder facade
crates/aurora-qt/           placeholder Qt/QML backend
crates/aurora-web/          placeholder WASM/web backend
examples/                   harness/showcase proofs
.github/workflows/          tests, fmt, clippy, audit, release
```

## Technology Stack and Dependencies

- **Rust:** 2021 edition; `gtk4` 0.11 (`v4_12`), `glib` 0.22, `serde`, `tracing`, `anyhow`/`thiserror`.
- **Optional sibling:** `sher_input_core` behind the `sher-input` feature.
- **Testing:** Rust unit tests, `#[gtk4::test]` real-display tests, xvfb on Linux.
- **CI:** Ubuntu + macOS, stable + beta, xvfb-run tests, doc tests, fmt, clippy, security audit, release build.
- **Packaging:** not published to crates.io; intended for pinned git dependencies.

## Public API

- `aurora_tokens::{DesignTokens, Color, ColorSystem, Theme, Spacing, Radius, Elevation, MotionScale, ...}`.
- `aurora_color` / `aurora_typography` / `aurora_motion` / `aurora_sound` / `aurora_icons` design primitives.
- `aurora_a11y` contrast computation and audit types.
- `aurora_gtk::{CssProvider, Theme, ThemePreference, ThemeListener, AuroraGtk}`.
- `aurora_gtk::widgets` with real GTK builders and styling/state-only types.
- `aurora_gtk::input::{AuroraInputDispatcher, InputSource, InputSink, NoopSink, RawInputEvent}`.
- Optional `sher_input_core` adapter behind `sher-input`.
- `spawn_glib`, `attach_to_glib`, and `spawn_aurora_input_worker` for GLib/thread integration.
- Examples: `gtk4_harness` and `showcase`.

## Design Patterns

- **Token-first theming:** widgets consume semantic tokens/classes rather than hardcoded colors.
- **Computed accessibility:** contrast checks are generated from real palettes.
- **Real-object proof:** a widget counted as implemented must construct a genuine `gtk4` object.
- **Optional integration:** sibling input types are feature-gated instead of default-linked.
- **Theme listener:** responds to GTK settings signals and reloads CSS in place.
- **Scaffold honesty:** placeholder crates are excluded from defaults.

## Data and Storage

- No database, network service, or durable application state.
- Tokens/themes are compile-time Rust data plus generated/shipped CSS.
- GTK widget state lives in GTK objects.
- Icons are SVG assets compiled/shipped with the crate.
- Theme preference is read from GTK settings; nothing writes dconf/gsettings.
- No secrets or credentials.

## Concurrency and Performance

- GTK must be used on the main thread; the GLib timeout source is appropriate for widget dispatch.
- **Confirmed bug:** `attach_to_glib` captures `dispatcher.clone()` and then clones again inside the callback (`let mut d = dispatcher.clone()`), so each tick dispatches from a fresh copy. Pointer coordinates and active-window changes made inside `route_event` do not persist to the next tick, and later mutations to the caller's dispatcher are not observed.
- The interval is converted to `u32` milliseconds; a sub-millisecond interval becomes 0, and `glib::timeout_add_local(0)` can spin.
- `poll_and_dispatch` loops until the source is empty. Under a continuously fed ring/queue, one tick can drain indefinitely and starve the GTK main loop.
- `spawn_aurora_input_worker` has an infinite loop with no stop flag/channel; the `JoinHandle` cannot complete cleanly, so shutdown/restart leaks a thread.
- Pointer state is updated only for absolute axes; relative motion (`EV_REL`) is ignored by default.
- Input source access uses a `Mutex`; a poisoned lock panics on `unwrap`.

## Security and Privacy

### Strengths

- No network, credentials, or personal-data handling.
- No unsafe code was highlighted in the audited GTK layer.
- Feature-gating avoids a mandatory sibling dependency.
- Accessibility checks reduce unsafe color contrast.
- Theme listener is read-only with respect to OS settings.
- Placeholder backends cannot be mistaken for complete implementations in default builds.

### Gaps

- The input bridge is not a security boundary; it assumes the input source is trusted.
- No capture/exclusivity model in Aurora itself; capture semantics live in SHER-Input/Display.
- No validation of synthetic event origin/ownership at this layer.
- The `sher-input` feature creates a path to a sibling crate, contradicting the “zero Cargo-level dependency” security/independence claim.
- No memory/resource bounds on event queues; if an unbounded source is used, the drain loop can be monopolized.

## Reliability and Observability

- `ThemeListener` reacts to real GTK signals and is testable with real GTK.
- Widget construction is proven by the harness.
- No structured logging/metrics/health surface; this is a UI library.
- Input bridge has no error channel: sink failures are the sink's responsibility.
- No shutdown protocol for the worker thread.
- No backpressure or per-tick event budget.
- No panic boundary around widget callbacks.
- Tests cover tokens, contrast, typography, motion, and GTK object state.

## Testing, CI, and Quality

- 525 static test definitions; README says about 518.
- Tests include real GTK4 construction under xvfb, WCAG regression tests, token/color/typography/motion tests, and theme-listener behavior.
- CI matrix: Ubuntu/macOS, stable/beta; `cargo test --lib`, doc tests, fmt, clippy, security audit, release build.
- No test for the `attach_to_glib` state-persistence bug, worker shutdown, or continuous-input starvation.
- No test for `EV_REL` routing because it is not implemented.
- No coverage gate or benchmark.
- The test suite is the strongest evidence that the design-token and GTK layers are real.

## Documentation, Packaging, and Release

- Workspace version is 1.3.0; no tagged crates.io release.
- README is internally inconsistent about widget count:
  - “5 of 17” in the feature bullet and status table.
  - “Only 8 of 17” in “What it is not.”
  - Actual `.build()` implementations found: 7 (Button, Input, Checkbox, Card, Switch, Dialog, Badge).
- README says “zero Cargo-level dependency on any sibling,” but `aurora-gtk` has an optional `sher_input_core` path dependency behind `sher-input`.
- README says about 518 tests; static count is 525.
- The `glib = []` feature is empty; glib is a mandatory dependency, so the feature only gates modules.
- Install instructions correctly require GTK4 >= 4.12 and a pinned git revision.
- No release provenance or published package verification.

## Strengths

- Real design-token system with coherent semantic layers.
- Automated, measured WCAG contrast rather than aspirational claims.
- Real GTK4 object construction and a runnable harness/showcase.
- Theme listener uses actual GTK settings signals.
- Broad test coverage across design primitives and GTK behavior.
- Clear separation of implemented vs styling-only vs placeholder widgets.
- Optional sibling integration avoids forcing SHER-Input on standalone users.
- CI exercises real GTK under xvfb on Linux.

## Risks and Gaps

| Risk | Severity | Evidence | Impact |
|---|---|---|---|
| GLib timeout dispatcher loses state each tick | High | `aurora-gtk/src/input.rs`: `let mut d = dispatcher.clone()` inside callback | Pointer/window state does not persist; input behavior is wrong. |
| Input worker has no shutdown | Medium | `spawn_aurora_input_worker` infinite loop | Thread leak; no clean shutdown/restart. |
| Continuous input can starve GTK | Medium | `poll_and_dispatch` drains until empty | UI stalls under high event rates. |
| Widget-count documentation is contradictory | Medium | README says 5, 8, and lists 17; 7 real builders | Users cannot tell what is implemented. |
| “Zero sibling dependency” is false with `sher-input` | Low/Medium | Optional `sher_input_core` path dep | Standalone/integration claims are misleading. |
| Relative motion is ignored | Low/Medium | `route_event` EV_REL arm is empty | Pointer integration is incomplete for mouse-style sources. |
| Zero/short GLib interval can busy-spin | Low | `as_millis() as u32`; no minimum | CPU spin if callers pass 0/sub-ms. |
| Mutex poisoning panics | Low | `source.lock().unwrap()` | A prior panic can take down input dispatch. |
| Test count drift | Low | README ~518 vs 525 static | Minor release-truth issue. |

## Prioritized Recommendations

### P0 — input correctness

1. Fix `attach_to_glib` to keep one dispatcher instance across ticks. Capture a single mutable dispatcher in a `Rc<RefCell<...>>`/main-thread-local structure (GTK types are not `Send`), or move the state into the callback closure, and add a regression test that two ticks preserve pointer/window state.
2. Give the worker a stop flag/channel and a clean join path; add a shutdown test.
3. Add a per-tick event budget or time slice to `poll_and_dispatch` so continuous input cannot monopolize the GTK loop.

### P1 — API and documentation truth

4. Make widget status a single source of truth: actual `.build()` widgets, styling-only widgets, and placeholder backends. Update every README location from that list.
5. Reword the sibling-dependency claim: “no default sibling dependency; optional `sher-input` integration behind a feature.”
6. Decide whether `EV_REL` belongs in Aurora’s bridge; implement it or state clearly that relative motion must be normalized upstream.
7. Enforce a minimum GLib interval (for example, 1 ms) and reject zero.
8. Replace `unwrap()` on the input mutex with poison recovery or an explicit error.

### P2 — hardening

9. Add tests for high-rate input, shutdown, poisoned locks, and two-tick state persistence.
10. Add a coverage gate for the GTK/input layer.
11. Add a short integration guide for SHER-Display scene-graph bridging, clearly marked as not implemented.
12. Verify any future crate publication/provenance.

## Audit Limitations

- GTK behavior was reviewed statically and through existing tests; no new interactive GTK session was run.
- The `attach_to_glib` and worker-shutdown defects were identified by code inspection, not by a new runtime test.
- No performance profiling of input dispatch was performed.
- Test counts are static definitions, not a verified pass count.
- Optional SHER-Input integration was not built or exercised in this audit.
