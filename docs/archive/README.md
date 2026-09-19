# Archive

Historical, superseded, and never-built planning/vision documents, kept
for reference only per this org's "archive, don't delete" convention.
**None of these are current.** For the actual, current, verified status
of this project, see the root [`README.md`](../../README.md),
[`CHANGELOG.md`](../../CHANGELOG.md), and
[`ROADMAP_HONEST.md`](../../ROADMAP_HONEST.md).

## Phase/session reports (12 files, archived first)

`GITHUB_ABOUT_SETUP.md`, `GITHUB_OPTIMIZATION.md`, `PHASE1_COMPLETION.md`,
`PHASE2_KICKOFF.md`, `PHASE3_ROADMAP.md`, `PHASE4_COMPLETION.md`,
`PHASE4_FINAL_REPORT.md`, `PHASE5_ROADMAP.md`, `RECOGNITION.md`,
`STARS_WORTHY_CHECKLIST.md`, `STARS_WORTHY_COMPLETE.md`,
`STARS_WORTHY_SUMMARY.md`, `UBUNTU_QUICK_START.md` — internal
session/phase-status reports, none referenced elsewhere in the repo.

## Pre-honesty-rewrite vision, spec, and release docs (26 files)

Everything else in this directory was written before the README/CHANGELOG
honesty rewrites visible in this repo's git history and was never updated
to match. All of it describes a materially different, more ambitious
product than what actually ships. Concretely, across these files:

- Icon counts of **"1000+"**/**"2000+"** (`ARCHITECTURE.md`,
  `ARCHITECTURE_V2.md`, `ICON_DESIGN_SYSTEM.md`,
  `PACKAGE_CONTROL_EXAMPLES.md`, and others) — the real number, shipped in
  `aurora-icons`, is **24** hand-authored SVGs.
- **`libadwaita` integration** claimed throughout (`ARCHITECTURE.md`,
  `INTEGRATION_GUIDE.md`, `PRODUCT_VISION.md`,
  `PRODUCT_VISION_CORRECTED.md`, `UBUNTU_INSTALLATION.md`) —
  `crates/aurora-gtk/Cargo.toml` has never depended on `libadwaita`; only
  `gtk4` and `glib`.
- **GDM/dconf/GNOME-Shell/notification-daemon system integration**
  described as built or in-progress (`ARCHITECTURE.md`,
  `IMPLEMENTATION_ROADMAP.md`) — no crate in this workspace depends on
  `gio`, `zbus`, or any D-Bus/dconf library; see
  `ROADMAP_HONEST.md` for what `aurora-gtk::gnome::*` actually contains
  (data structures only, no real system calls).
- **"PRODUCTION READY" / "Production Ready"** status claims
  (`PROJECT_COMPLETE.md`, `RELEASE_NOTES_V1.1.md`, `RELEASE_v1_0_0.md`)
  for a project that, at the time and still today, has never published to
  crates.io and has 12 of 17 widgets with no real rendering.
- **Future-dated roadmaps and release notes** — `V1.1_ROADMAP.md` plans
  "January – March 2027" with "5-6 engineers"; `RELEASE_NOTES_V1.1.md` and
  the `CHANGELOG.md` `[1.1.0]` entry claim a release date of
  **2027-03-31**, later than the real, git-committed `[1.2.0]` (2026-08-16)
  and `[1.3.0]` (2026-08-26) entries. This is a real inconsistency — see
  `ROADMAP_HONEST.md` — left uncorrected in `CHANGELOG.md` itself because
  the true original date can't be reconstructed, but disclosed rather than
  silently re-dated.
- A wholesale **different product**: `APT_DISTRIBUTION_ARCHITECTURE.md`,
  `APT_REPOSITORY_INDEX.md`, `GPG_SIGNING_SETUP.md`, `HOSTING_SETUP.md`,
  `PACKAGE_CONTROL_EXAMPLES.md`, `PHASE1_2_COMPLETION.md`,
  `PHASE3_6_GUIDE.md`, `PRODUCTION_READINESS_CHECKLIST.md`,
  `PROJECT_COMPLETE.md`, `REPOSITORY_SETUP_GUIDE.md`,
  `UBUNTU_INSTALLATION.md`, `RELEASE_v1_0_0.md` describe "Aurora Linux
  Design System," a distro-wide APT package ecosystem (GDM/SDDM/Plymouth
  theming, KDE/GNOME integration, editor themes, a signed APT repository)
  under a different GitHub org (`aurora-linux/aurora`). The matching
  `packages/`, `debian/`, `Makefile`, and `scripts/` this described have
  been moved to
  [`unbuilt-apt-packaging/`](unbuilt-apt-packaging/README.md) in this same
  archive — none of it was ever built or published.
- The remaining files (`API_REFERENCE.md`, `COMPONENT_SPECIFICATIONS.md`,
  `DESIGN_LANGUAGE.md`, `ICON_ENHANCEMENT_GUIDE.md`,
  `TYPOGRAPHY_IMPLEMENTATION.md`, `PRODUCT_VISION.md`,
  `PRODUCT_VISION_CORRECTED.md`, `IMPLEMENTATION_ROADMAP.md`) are design
  specs and API docs written against this same, larger, unbuilt vision;
  they were never reconciled with the real crates and should not be
  trusted as API documentation. Real API surface is documented via
  `cargo doc --no-deps --open` against the actual source, and the root
  `README.md`'s "What's real today" table.

If a future contributor wants an accurate architecture reference, use
[`../architecture/README.md`](../architecture/README.md) instead, which
describes only what's actually in `Cargo.toml` today.
