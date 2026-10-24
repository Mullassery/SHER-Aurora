# Competitive intelligence in roadmap and upgrade work

**Policy date:** 2026-10-19

Competitive findings are an input to product decisions, not a one-off report.
Every pending roadmap item, new feature, and routine dependency upgrade should
check the repository's [`AUDIT_FINDINGS.md`](AUDIT_FINDINGS.md) before work
starts, and record the decision in the PR or changelog.

## Required check before building

1. **Read `AUDIT_FINDINGS.md`.** Find the relevant competitor row and the
   ranked next actions. If the finding is stale, update it with a dated source.
2. **Name the competitor and the user problem.** A feature is not justified by
   "competitor has it". State the workflow it unlocks, who needs it, and how
   often. Mark unverified demand as a hypothesis.
3. **Prefer reuse over new code.** Check whether an existing Mullassery repo
   already provides the capability (see `BUNDLES.md` in the profile repo and
   the bundle role file in this repo, when present). Typical reuse:
   - crawling/fetching/robots/SSRF → **MudFish**
   - architecture maps / PR diffs → **CodeAtlas**
   - data-quality contracts / validation → **StatGuardian**
   - token counting / cost tables → **PyTokenCalc**
   - dependency graph / advisories / SBOM → **PyDeepMesh**
   - segmentation / CLV / churn → **ClusterAudienceKit**
   - activation / lineage / PII → **PyReverseETL**
   - fleet state sync → **SyncFleet** (only after conflict resolution)
4. **Choose one disposition:** build, integrate, defer, or reject. Write it
   down. A rejection with a reason is a successful outcome.
5. **For routine upgrades**, check whether the upgrade closes a known
   competitor gap or breaks a differentiated capability. Run the tests that
   cover the affected path; do not merge a green build that silently changes a
   user-visible contract.

## PR / changelog checklist

- [ ] Competitor and source URL/date recorded (or "no direct competitor").
- [ ] User impact and frequency stated; unverified demand labelled.
- [ ] Reuse check done against sibling repos/bundles.
- [ ] Failure mode and rollback considered (especially auth, PII, cost).
- [ ] Docs/roadmap status updated honestly.
- [ ] Tests or a manual verification command included.

## Review cadence

- Refresh the competitor row when a release changes the capability.
- Re-check the full `AUDIT_FINDINGS.md` at least quarterly and before a major
  release.
- If a finding is wrong or stale, correct it in the same PR that relies on it.

## Non-goals

- Copying features for parity.
- Adding a dependency just because a competitor uses it.
- Treating star counts or download counts as proof of user need.

## OSS gap analysis — 2026-10-24

**What matters most here:** widget coverage and a GNOME conformance path. Only 7 of 17 widgets construct real GTK4 objects, and no libadwaita/HIG path exists; finish the widget layer before positioning it as a design system.

Scope: workspace v1.3.0 — real token/typography/color/motion/sound/icon crates,
`aurora-a11y` contrast auditing, and `aurora-gtk`. Seven of 17 widget modules have
a `.build()` returning a real `gtk4` object (Button, Input, Checkbox, Card, Switch,
Badge, Dialog); ten are logic-only. Qt/web/core are stubs; tag v1.2.0 lags 1.3.0.

### Comparable OSS projects

| Project | GitHub | License | Last activity | Why it is the comparison |
|---|---|---|---|---|
| libadwaita | https://github.com/GNOME/libadwaita | LGPL-2.1+ | Tag 1.10.0 on 2026-09-14; commits 2026-10-20 | The Adwaita widget/style library GNOME apps use; direct alternative to a parallel GNOME design system |
| gtk4-rs | https://github.com/gtk-rs/gtk4-rs | MIT | Tag 0.11.5 on 2026-09-20; commits 2026-10-23 | Rust GTK4 bindings; what users build on when they do not adopt Aurora's wrappers or tokens |
| iced | https://github.com/iced-rs/iced | MIT | Tag 0.14.0 on 2025-12-07; commits 2026-10-21 | Cross-platform Rust GUI toolkit with its own widgets and theming; the non-GTK alternative |

The GNOME Human Interface Guidelines (https://developer.gnome.org/hig/) are the
design reference for Adwaita conformance; they are documentation, not a repo.

### Capability comparison

| Capability | Ours | libadwaita | gtk4-rs | iced | Gap |
|---|---|---|---|---|---|
| Design tokens (spacing/type/color/motion) | Yes | Partial | No | Partial | Ahead in breadth as a Rust API; Adwaita exposes styles rather than tokens |
| Prebuilt styled widget set | Partial | Yes | Partial | Yes | Weaker: 7 of 17 widget modules render real GTK4 objects |
| Four themes incl. OLED/HDR palettes | Yes | Partial | No | Partial | Differentiated: Light/Dark/OLED/HDR token palettes; Adwaita adds high contrast |
| Automated WCAG contrast audit | Yes | Partial | No | No | Differentiated: computed ratios per pairing/theme, not design intent |
| Accessibility beyond color contrast | No | Yes | Partial | Partial | Absent: no keyboard-nav, screen-reader, or reduced-motion tests |
| GNOME HIG conformance | Partial | Yes | Partial | No | Weaker: GNOME-style tokens, no HIG conformance tests |
| libadwaita interoperability | No | Yes | Partial | No | Absent by design: no crate depends on libadwaita |
| GNOME system integration (dconf/D-Bus) | No | Yes | Partial | No | Absent: `gnome/` modules are data models with no OS I/O |
| Reactive OS theme preference | Yes | Yes | Partial | Partial | Parity: real GTK settings notify signals and in-place CSS reload |
| Icon set | Partial | Yes | No | Partial | Weaker: 24 hand-authored SVGs versus a full symbolic icon theme |

### Gaps that matter (ranked)

1. **Only 7 of 17 widgets render (absent, L).** DataTable, Tabs, Menu, List,
   Select, Sidebar, Breadcrumb, Radio, IconDock, and Tooltip have logic but no
   `.build()`; a GNOME app still writes its own GTK4 for the harder widgets.
2. **No libadwaita or HIG conformance path (absent, M/L).** Aurora defines a
   parallel GNOME-style language, while libadwaita gives the actual Adwaita widgets,
   accessibility, and adaptive layouts; adopting Aurora means opting out.
3. **Accessibility is contrast-only (absent, M).** `aurora-a11y` computes WCAG
   ratios, but there is no keyboard-navigation, screen-reader, or reduced-motion
   test. libadwaita inherits GTK's accessible roles and focus handling.
4. **No real GNOME system integration (absent, M).** `dconf.rs`,
   `notifications.rs`, `observer.rs`, and `settings_panel.rs` return hardcoded values
   or in-memory structs; no gio/zbus/D-Bus dependency exists anywhere in the workspace.
5. **Status docs disagree with the code (weaker, S).** README says 5 widgets,
   then 6 with Badge, while `dialog.rs` also has `.build()` (7 total); ROADMAP
   still lists 12 without rendering. The 1.3.0 source is untagged (latest tag
   v1.2.0), so a pinned consumer cannot get the documented fixes.
6. **No consumer proves the system (weaker, S/M).** No downstream application,
   no crates.io package, and no tagged 1.3.0 release mean the token API and
   widget ergonomics are unvalidated outside the repo's own 525 tests.

### Where we are differentiated

- The token layer is a real Rust API with four audited palettes (including OLED
  and HDR), typography scales, spring-physics motion, and semantic sound events;
  Adwaita ships styles, not an equivalent cross-crate token model.
- `aurora-a11y` measures WCAG ratios for every semantic foreground/background
  pairing in every theme and fails on regression; that is more explicit than the
  design-review process around Adwaita, though narrower than full accessibility.
- `ThemeListener` reacts to real GTK theme/prefer-dark settings without taking a
  libadwaita dependency, which is useful for GTK-only deployments.
- 24 hand-authored SVGs and a CSS provider generated from tokens are real assets,
  not placeholders.

### Disposition

- **Build:** complete only the widgets a named consumer needs (DataTable, Tabs,
  Menu, List, Select, Sidebar first); add keyboard-focus and reduced-motion tests;
  do a documentation truth pass so counts and tags match the code.
- **Integrate/reuse:** add an optional libadwaita adapter instead of reimplementing
  Adwaita; use GTK's accessibility APIs rather than a parallel model; keep gtk4-rs
  as the binding; consider the system symbolic icon theme over growing the set.
- **Defer/reject:** Qt/web/core stub backends, full 17-widget parity, a theme
  daemon or Shell extension, and crates.io publication until a real app adopts it.

### Sources

- https://github.com/GNOME/libadwaita and /tags (accessed 2026-10-24)
- https://github.com/gtk-rs/gtk4-rs and /tags (accessed 2026-10-24)
- https://github.com/iced-rs/iced and /tags (accessed 2026-10-24)
- https://developer.gnome.org/hig/ (accessed 2026-10-24)
- Repo files inspected: README.md, AUDIT_FINDINGS.md, ROADMAP_HONEST.md,
  Cargo.toml, all manifests, `aurora-gtk/src/widgets/*.rs`,
  `aurora-gtk/src/theme_listener.rs`, `aurora-a11y/src/*.rs`
