# Audit Findings: SHER-Aurora

**Audit Date:** 2026-10-26  
**Version:** 1.3.0  
**Branch:** `main`  
**Full report:** [`.audit/reports/SHER-Aurora/audit.md`](../.audit/reports/SHER-Aurora/audit.md)

## Scope

Read-only review of the token/color/typography/motion/sound/icon/a11y crates, GTK4 widget layer, input bridge, examples, CI, and release documentation. No new interactive GTK session was run.

## Findings

| ID | Severity | Finding | Evidence | Recommendation |
|---|---|---|---|---|
| AUR-01 | High | `attach_to_glib` clones the dispatcher inside every tick, losing pointer/window state. | `crates/aurora-gtk/src/input.rs`: `let mut d = dispatcher.clone()` in the timeout callback. | Keep one dispatcher/state across ticks and add a two-tick regression test. |
| AUR-02 | Medium | Background input worker has no shutdown path. | `spawn_aurora_input_worker` infinite loop. | Add a stop flag/channel and clean join. |
| AUR-03 | Medium | Continuous input can starve the GTK main loop. | `poll_and_dispatch` drains until empty. | Add a per-tick event/time budget. |
| AUR-04 | Medium | Widget-count documentation is contradictory. | README says 5 and 8 of 17; 7 real `.build()` widgets exist. | Generate docs from one authoritative widget table. |
| AUR-05 | Low/Medium | “Zero Cargo-level dependency on any sibling” is false with `sher-input`. | Optional `sher_input_core` path dependency. | Say “no default sibling dependency; optional integration.” |
| AUR-06 | Low/Medium | Relative motion (`EV_REL`) is ignored. | Empty arm in `route_event`. | Implement it or document upstream normalization. |
| AUR-07 | Low | Zero/short GLib interval can busy-spin. | `as_millis() as u32` with no minimum. | Enforce a minimum interval and reject zero. |
| AUR-08 | Low | Input mutex poisoning panics. | `source.lock().unwrap()`. | Recover from poison or return an error. |
| AUR-09 | Low | Test count drift. | README ~518 vs 525 static tests. | Update the count. |

## Positive Controls

- Real design-token foundation across spacing/color/type/motion.
- Automated WCAG contrast checks over actual token pairings.
- Real GTK4 objects built and asserted by the harness.
- ThemeListener uses real GTK settings notify signals.
- Clear separation of implemented, styling-only, and placeholder widgets.
- Optional sibling integration is feature-gated.
- CI tests real GTK under xvfb on Linux and macOS.

## Audit Limitations

- No new interactive GTK session or performance profile.
- Input defects were found by static inspection, not a new runtime test.
- Optional SHER-Input integration was not built.
- Test counts are definitions, not a verified pass count.
