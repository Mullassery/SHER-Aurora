# Honest Roadmap and Known Gaps

This file is the single source of truth for what's broken, unbuilt, or
unverified in this repository. Blunt by design: no "planned"/"may be
added" softening for things that simply don't exist. Where something is
broken, it says broken. Where something is untested, it says untested —
not "should work."

Verified locally (macOS, this pass, `2026-09-19`):
`cargo build --workspace` — succeeds. `cargo test --workspace` — 518
tests, 0 failed. `cargo clippy --workspace --all-targets -- -D warnings`
— 0 warnings. `cargo fmt --check` — clean. `cargo run --example
gtk4_harness -p aurora-gtk` — runs, builds real GTK 4.22.4 widgets.
`cargo audit` — **could not run**; this sandbox has no network access to
fetch the RustSec advisory database. CI's `Security Audit` job
(`rustsec/audit-check`) runs this on every push/PR in a real GitHub
Actions runner — its live status is what to trust, not this sandbox.

**Quick-fix pass, `2026-09-21`:** re-verified locally (`cargo build
--workspace`, `cargo test --workspace` — 518 tests, 0 failed, `cargo
clippy --workspace --all-targets -- -D warnings` — 0 warnings, `cargo
fmt --check` — clean) after fixing the two items marked FIXED below (the
hex-literal `.unwrap()` pattern and the missing GNOME/CLI doc-comment
disclosure). No other items in this file were touched — everything else
below is carried forward unchanged from the `2026-09-19` pass.

---

## 1. Built but not independently re-verified live on GitHub

- **CI green status is not independently confirmed by this pass.** The
  last commit (`ed34d39`, "Confirm CI fully green after Cargo.lock fix")
  claims all 9 CI jobs pass. This sandbox has no network access to GitHub
  Actions, so that claim was **not** re-checked here — it's carried
  forward from git history, not re-verified. Check the live badge at the
  top of `README.md` yourself before trusting it further out from that
  commit.
- **`.github/dependabot.yml` — added in this pass, unverified in
  production.** Configured for `cargo` and `github-actions` ecosystems,
  weekly. It has not yet run against the real repo (that only happens
  once merged and picked up by GitHub), so whether it opens usable PRs
  (e.g., doesn't conflict with the committed `Cargo.lock`) is unverified.
- The 5 widgets with `.build()` (Button, Input, Checkbox, Card, Switch)
  are proven to construct real `gtk4` objects (see `gtk4_harness.rs`,
  re-run in this pass, output confirms `GtkButton, GtkEntry,
  GtkCheckButton, GtkSwitch, GtkBox`). Their *visual* correctness (does it
  actually look right, is it usable) has not been evaluated by a human in
  this pass — only structural/API assertions were checked.

## 2. Not built (state plainly, no hedging)

- **No Qt or web renderer exists.** `aurora-qt` and `aurora-web` are
  empty stub crates (no logic, no dependencies). `aurora-core` is also an
  empty stub — there is no unified facade over the other crates.
- **No real GNOME system integration exists anywhere in this workspace.**
  `crates/aurora-gtk/src/gnome/dconf.rs` (153 lines) defines
  `DConfSchema::schema_xml()`, which returns a hardcoded XML string
  literal — it does not write a schema file to disk, does not call
  `glib-compile-schemas`, and does not read or write any real dconf key.
  `notifications.rs` defines an `as_dbus_int()` method that returns a
  plain `i32` — no D-Bus connection is ever opened anywhere in this
  workspace (there is no `zbus`/`dbus`/`gio` dependency in any
  `Cargo.toml`). `observer.rs` and `settings_panel.rs` are in-memory data
  structures with no OS hooks. None of this performs real GNOME
  integration; it models what such integration's data shapes might look
  like.
- **No CLI tool exists.** `crates/aurora-gtk/src/cli/mod.rs` defines
  `Command`/`CommandType` (line 130 / line 9) as plain data structures.
  There is no `[[bin]]` target in any `Cargo.toml` in this workspace, no
  `clap` (or any argument-parsing) dependency, and no `fn main` anywhere
  that reads `std::env::args()`. `aurora new`/`add`/`generate`/`theme`/
  `export`/`init`, referenced in `CHANGELOG.md`'s `[1.1.0]` entry, do not
  exist as runnable commands and never have, as far as this pass can
  verify from the current source tree.
- **12 of 17 originally-scoped widgets have no real rendering.**
  DataTable, Tabs, Menu, Dialog, List, Select, Sidebar, Badge, Breadcrumb,
  Radio, Tooltip, IconDock have Rust state/styling logic and unit tests,
  but no `.build()` method and construct no real `gtk4` object. (Already
  disclosed in `README.md`'s status table; repeated here for completeness
  since this is the bucket for it.)
- **No accessibility testing beyond color contrast.** `aurora-a11y`
  mechanically audits WCAG contrast ratios only. There is no automated
  keyboard-navigation test, no automated screen-reader test, and no
  automated `prefers-reduced-motion` test anywhere in this repo, despite
  design docs (now archived) describing "Full WCAG AAA compliance"
  including colorblind simulation, dyslexia fonts, and motion-reduction
  support. None of that exists in code — only the contrast audit is real.
- **Not published to crates.io.** Only usable as a git dependency.
- **The tagged releases lag `main`.** The latest git tag is `v1.2.0`; the
  workspace version in `Cargo.toml` and `CHANGELOG.md`'s `[1.3.0]` entry
  are ahead of it with no matching tag/GitHub Release. Anyone following
  `README.md`'s install instructions (`tag = "v1.2.0"`) does not get the
  HDR-fallback fix or the CI fixes described in `[1.3.0]` — those exist
  only on `main`, unreleased.
- **The APT/Debian "Aurora Linux Design System" packaging ecosystem was
  never built.** Moved to `docs/archive/unbuilt-apt-packaging/` in this
  pass — see that directory's README for specifics (wrong GitHub org in
  every `debian/control` file, placeholder maintainer email, a
  literal-no-op `make validate` target, no GPG key ever generated, no APT
  repository ever hosted).

## 3. CI / infrastructure gaps

- **No live re-verification possible from this sandbox** (no network) —
  see section 1. This is a gap in *this pass's* ability to confirm
  status, not a known-broken CI job.
- **Security-audit coverage is dependency-only.** `rustsec/audit-check`
  covers known-vulnerable crates in `Cargo.lock`. There is no SAST/static
  analysis, no fuzzing, and no `cargo geiger` (unsafe-code) check in CI.
  `grep -rn "unsafe" crates/*/src` found zero occurrences in this pass, so
  this is a low-priority gap today, but worth naming if that changes.
- **`cargo clippy`/`cargo fmt` run in a separate job (`fmt`, `clippy`)
  from `Tests`** in `ci.yml` — fine, but means a PR could pass `Tests`
  while `clippy`/`fmt` are still running; no blocking dependency between
  jobs is configured (not a bug, just worth knowing when reading a
  partially-green run).
- **Fixed in this pass (small, safe, verified with `actionlint`):**
  removed unused `libadwaita`/`libadwaita-1-dev` install steps from
  `ci.yml` (three places) — no crate in this workspace has ever depended
  on `libadwaita`; bumped `actions/cache@v3` → `v4` and
  `softprops/action-gh-release@v1` → `v2` in `release.yml` (v1 flagged by
  `actionlint` as running on a deprecated Actions runtime); fixed a stale
  code comment in `release.yml` referencing the old "Proprietary" license
  (project relicensed to Apache-2.0 on 2026-09-06, comment wasn't
  updated).

## 4. Features that look functional but are not (undisclosed until this pass)

- See section 2's GNOME-integration and CLI bullets above — these are the
  primary items in this bucket. They compile, have some unit tests
  (e.g., `notifications.rs`'s `test_urgency_dbus_int` at line ~207
  verifies the returned `i32` matches expected values — a test of a
  constant mapping, not of any real D-Bus behavior), and are exported as
  `pub mod` from `aurora-gtk`'s `lib.rs`, so they look like functioning
  subsystems from the public API surface alone. They are not.
- **`CHANGELOG.md`'s `[1.0.0]` and `[1.1.0]` entries describe features
  that don't match the code and contain an impossible date ordering**
  (`[1.1.0]` dated `2027-03-31`, later than `[1.2.0]`'s `2026-08-16` and
  `[1.3.0]`'s `2026-08-26`). Flagged in-place in `CHANGELOG.md` with an
  editorial note in this pass rather than silently rewritten, since the
  true original dates/state can't be reconstructed at this distance.

## Technical debt (concrete, file:line where possible)

- **FIXED (2026-09-21, quick-fix pass):** `.unwrap()` on parsed hex-color
  literals in production code, not just tests. `crates/aurora-color/src/theme.rs`
  (88 occurrences, lines 72–189) and `crates/aurora-tokens/src/color.rs`
  (68 occurrences, lines 108 onward) now use
  `.expect("valid built-in hex constant")` instead of bare `.unwrap()`,
  so a future typo in a built-in hex literal fails loudly with context
  instead of a bare panic. This is the "at minimum" mitigation named
  below, not the `const fn` compile-time-checked parser — that larger
  alternative is still not built and would need a dedicated follow-up if
  wanted.
- **FIXED (2026-09-21, quick-fix pass):** `crates/aurora-gtk/src/gnome/`
  and `crates/aurora-gtk/src/cli/` are exported public API (`pub mod
  gnome; pub mod cli;` in `lib.rs`) and previously had no doc-comment
  disclosure that they perform no real system I/O / aren't a runnable
  CLI. Added explicit `//! NOTE: no real system I/O` (or equivalent)
  doc comments to the top of `dconf.rs`, `notifications.rs`,
  `observer.rs`, `settings_panel.rs`, and `cli/mod.rs`, pointing at this
  file. This was the "cheap" disclosure option named below; actually
  implementing real dconf/D-Bus/CLI behavior is still not done and would
  need a dedicated follow-up if wanted.
- **No `[[bin]]` targets or installable binary anywhere in the
  workspace**, despite a `CHANGELOG.md` `[1.3.0]` entry justifying
  committing `Cargo.lock` partly by reference to "real binaries
  (`aurora_calendar`, `aurora_files`, `aurora_music`, `aurora_settings`)"
  — those were fake top-level example files deleted in `[1.2.0]` (per
  that same changelog), not real, currently-existing `[[bin]]` targets.
  Committing `Cargo.lock` is still correct practice for this workspace
  regardless (reproducible builds matter for libraries too, and it's
  needed for `cargo audit`/`rustsec/audit-check` to run at all), but the
  historical justification text is imprecise. Not fixed in this pass
  (historical changelog text, left as-is per this org's convention of not
  rewriting history) — flagged here instead.
- **No `unsafe` blocks anywhere in `crates/*/src`** (verified via grep in
  this pass) — noted as a positive, not a gap.
- **No `#[allow(...)]` lint suppressions anywhere in `crates/*/src`**
  (verified via grep) — also a positive; clippy is not being silenced
  anywhere to fake a clean run.
- **No `TODO`/`FIXME`/`HACK`/`XXX` markers anywhere in `crates/*/src`**
  (verified via grep) — also a positive; if debt exists it isn't
  currently being flagged in-source at all, which is itself worth naming
  (this file is now the place that debt should be flagged instead).
- **This pass's `docs/` overhaul was large and mostly mechanical, not
  line-by-line verified against source for every archived file.** 26
  files were moved to `docs/archive/` because they described a
  larger/different, partially fictional product (see
  `docs/archive/README.md` for the specific evidence per file: icon
  counts, `libadwaita`, GDM/dconf claims, future-dated 2027 roadmaps,
  "PRODUCTION READY" status). This was a documentation-disclosure action
  (moving+labeling, not editing each file's claims individually) — if any
  of those 26 files contains something narrowly true worth salvaging
  (e.g., `TYPOGRAPHY_IMPLEMENTATION.md`'s specific type-scale numbers),
  that wasn't individually fact-checked against `aurora-typography`'s
  source in this pass. **Recommend a dedicated follow-up session** if
  anyone wants to mine the archive for salvageable accurate content
  rather than treating it as fully superseded.

## Recommended priority for a dedicated follow-up session

1. **Highest, partially done:** the in-source doc-comment disclosure for
   `aurora-gtk::gnome`/`aurora-gtk::cli` was added in the 2026-09-21
   quick-fix pass (see Technical debt section above). Actually
   implementing real dconf/D-Bus/CLI behavior is still real work and
   still needs a dedicated follow-up if wanted.
2. **Medium:** cut a `v1.3.0` tag/GitHub Release so `README.md`'s install
   instructions actually deliver the HDR-fallback and CI fixes already on
   `main`.
3. **Done (2026-09-21 quick-fix pass):** the hex-literal `.unwrap()`
   pattern in `aurora-color` and `aurora-tokens` now uses
   `.expect("valid built-in hex constant")`. A `const fn`
   compile-time-checked parser would still be a stronger alternative but
   is a larger change, left undone.
4. **Low:** mine `docs/archive/` for any narrowly-accurate technical
   content worth restoring in a corrected form (typography scale specs,
   component spec structure) rather than leaving the archive as the only
   record.
