# Unbuilt: APT/Debian packaging for "Aurora Linux Design System"

Everything under this directory (`packages/`, `debian/`, `Makefile`,
`scripts/`) describes a distro-wide Debian/Ubuntu package ecosystem for a
product called "Aurora Linux Design System" — GTK/Qt/Plasma themes, icon
themes, cursor themes, fonts, wallpapers, GDM/SDDM/Plymouth login-screen
theming, KDE/GNOME shell integration, VS Code/JetBrains editor themes,
terminal themes, an APT repository with GPG-signed releases, and 18
separate `.deb` sub-packages.

**None of this was ever built, tested, or published.** It does not
describe the project that actually ships from this repository (a Rust
workspace of `aurora-*` crates providing design tokens, typography, color,
motion, sound, accessibility, and a GTK4 widget library — see the root
[README.md](../../../README.md)). Concretely:

- Every `debian/control` file here lists `Maintainer: Aurora Team
  <aurora@example.com>` and `Vcs-Git:
  https://github.com/aurora-linux/aurora.git` — a placeholder email and a
  different GitHub org than this repo (`Mullassery/aurora`).
- `Makefile`'s `validate` target is a literal no-op
  (`echo "✓ Asset validation complete (placeholder)"`).
- None of these scripts or targets are invoked by `.github/workflows/`, by
  the root `README.md`, or by `CONTRIBUTING.md`.
- No GPG key was ever generated, no APT repository was ever hosted, no
  `.deb` package has ever been built from this content.

This is not a roadmap item "in progress" — it is dead, unreferenced
content from an earlier, different product direction, kept here (not
deleted) per this org's convention of archiving rather than discarding
history. If this direction is picked up again, treat it as a from-scratch
design, not a continuation of what's here (the org name, maintainer
email, and version numbers below would all need to be fixed first, and
none of it has been validated against a real `dpkg-buildpackage`/`lintian`
run).

See [`../README.md`](../README.md) for the rest of the archive, and the
root [`ROADMAP_HONEST.md`](../../../ROADMAP_HONEST.md) for this project's
actual, current status.
