# Aurora Linux Design System — APT Distribution Infrastructure

**Version:** 1.0.0 (Draft)  
**Status:** Design Phase  
**Author:** Aurora Engineering Team  
**Date:** 2026-08-02  
**Scope:** Production-grade Debian/Ubuntu package distribution ecosystem

---

## Executive Summary

This document defines a complete, production-ready APT repository infrastructure for Aurora Linux Design System. Aurora will be installable on any Ubuntu or Debian system via a single command:

```bash
sudo apt install aurora
```

This architecture covers:
- Package hierarchy and dependency management
- Debian packaging standards and compliance
- Secure repository generation and signing
- Automated CI/CD deployment pipeline
- Long-term maintenance and upgrade strategy
- Multi-platform distribution roadmap

**Key Design Principles:**
- Follow Debian Policy Manual strictly
- Security-first (reproducible builds, signed packages, key rotation)
- Scalability (support thousands of concurrent users)
- Developer-friendly (clear tooling, documentation, contribution workflows)
- User-friendly (seamless installation, automatic updates, backward compatibility)

---

## Part 1: Package Architecture

### 1.1 Package Hierarchy

The Aurora ecosystem will be distributed as interconnected Debian packages with clear separation of concerns:

```
aurora (meta-package)
├── aurora-themes (GTK/Qt/Plasma core)
│   ├── aurora-themes-gtk (GTK 3/4 themes)
│   ├── aurora-themes-qt (Qt/Plasma themes)
│   └── aurora-themes-kde (KDE Colors)
├── aurora-icons (icon themes)
│   ├── aurora-icons-light
│   ├── aurora-icons-dark
│   └── aurora-icons-extra (optional)
├── aurora-cursors (cursor themes)
├── aurora-fonts (typography)
│   ├── aurora-fonts-core
│   └── aurora-fonts-extended (optional)
├── aurora-wallpapers (backgrounds)
├── aurora-colors (color palettes & tokens)
├── aurora-branding (brand assets)
├── aurora-terminal-themes (terminal colors)
├── aurora-vscode (VS Code theme)
├── aurora-jetbrains (JetBrains IDEs)
├── aurora-plymouth (boot splash)
├── aurora-gdm (GNOME login screen)
├── aurora-sddm (KDE login screen)
├── aurora-gnome-integration
├── aurora-kde-integration
├── aurora-kde-themes (full KDE Plasma theme)
└── aurora-accessibility (a11y assets)
```

### 1.2 Package Specifications

#### **aurora** (Meta-package)
- **Purpose:** Convenience package that installs all core Aurora components
- **Type:** Architecture-independent (`all`)
- **Size:** ~5 KB (meta-package, no actual files)
- **Install Location:** None (meta-package only)
- **Dependencies:** Depends on all core packages
- **Recommends:** Extended/optional packages
- **Conflicts:** None
- **Replaces:** None
- **Semantic Versioning:** MAJOR.MINOR.PATCH (tied to visual design release cycle)
- **Version Strategy:** Bump MAJOR on visual redesign, MINOR on component additions, PATCH on bugfixes

#### **aurora-themes**
- **Purpose:** Core GTK, Qt, and Plasma theme files
- **Type:** Architecture-independent (`all`)
- **Install Location:** 
  - `/usr/share/themes/Aurora/` (GTK)
  - `/usr/share/themes/Aurora-Light/` (light variant)
  - `/usr/share/themes/Aurora-Dark/` (dark variant)
- **Dependencies:** None (optional: glib, gtk-update-icon-cache)
- **Recommends:** aurora-icons, aurora-cursors, aurora-fonts
- **Post-install:** Update icon cache with `update-icon-caches`
- **Conflicts:** None (themes are non-exclusive)

#### **aurora-icons**
- **Purpose:** Icon theme assets
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/icons/Aurora/` (with scalable/ and symbolic/ subdirs)
- **Dependencies:** None
- **Recommends:** aurora-themes, aurora-cursors
- **Post-install:** Run `update-icon-caches /usr/share/icons/Aurora`
- **Note:** Includes both light and dark variants with symlinked fallbacks

#### **aurora-cursors**
- **Purpose:** Cursor theme set
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/icons/Aurora-Cursors/`
- **Dependencies:** None
- **Recommends:** aurora-icons

#### **aurora-fonts**
- **Purpose:** Typography (core + extended)
- **Type:** Architecture-independent (`all`)
- **Core Package:** ~10 MB (essential fonts)
- **Extended Package:** ~40 MB (additional typefaces, optional)
- **Install Location:** `/usr/share/fonts/aurora/` (opentype/truetype)
- **Dependencies:** fontconfig
- **Post-install:** Run `fc-cache -fv` to rebuild font cache
- **Recommends:** aurora-themes (for visual harmony)

#### **aurora-wallpapers**
- **Purpose:** Background images for desktop
- **Type:** Architecture-independent (`all`)
- **Size:** ~150 MB (high-quality 4K backgrounds)
- **Install Location:** `/usr/share/backgrounds/aurora/`
- **Dependencies:** None
- **Recommends:** aurora-themes

#### **aurora-colors**
- **Purpose:** Color palettes, design tokens, CSS variables
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/aurora/colors/` (JSON/YAML format)
- **Dependencies:** None
- **Contents:**
  - CSS variables (`:root { --aurora-primary: ... }`)
  - Tailwind config presets
  - Material Design colors
  - Accessibility contrast presets

#### **aurora-branding**
- **Purpose:** Brand assets, logos, media kit
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/aurora/branding/`
- **Dependencies:** None
- **Recommends:** aurora-fonts (for brand-aligned typography)

#### **aurora-terminal-themes**
- **Purpose:** Terminal color schemes for bash, zsh, fish, etc.
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/aurora/terminal/` (colorscheme definitions)
- **Dependencies:** None
- **Includes:** iTerm2, Alacritty, Kitty, GNOME Terminal presets

#### **aurora-vscode**
- **Purpose:** VS Code color theme
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/aurora/vscode/theme.json`
- **Dependencies:** None
- **Recommends:** aurora-fonts (for editor font pairing)
- **Note:** Users can symlink or copy to `.vscode/extensions/`

#### **aurora-jetbrains**
- **Purpose:** JetBrains IDE color scheme
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/aurora/jetbrains/`
- **Dependencies:** None
- **Supports:** IntelliJ, PyCharm, WebStorm, Rider, etc.

#### **aurora-plymouth**
- **Purpose:** Plymouth boot splash
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/plymouth/themes/aurora/`
- **Dependencies:** plymouth
- **Post-install:** `update-alternatives --install /etc/alternatives/default.plymouth ...`
- **Conflicts:** May conflict with other boot splash themes during set

#### **aurora-gdm**
- **Purpose:** GNOME Display Manager (login screen) theme
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/themes/Aurora-GDM/` or `/usr/share/gnome-shell/themes/`
- **Dependencies:** gdm (soft dependency via Recommends)
- **Post-install:** Update GDM dconf settings

#### **aurora-sddm**
- **Purpose:** KDE SDDM (login screen) theme
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/sddm/themes/aurora/`
- **Dependencies:** sddm (soft dependency)
- **Post-install:** Update SDDM config

#### **aurora-gnome-integration**
- **Purpose:** GNOME-specific integration (settings, extensions)
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/glib-2.0/schemas/` (if dconf schemas), `/usr/share/gnome-shell/extensions/`
- **Dependencies:** gnome-shell (soft), gsettings-desktop-schemas
- **Recommends:** aurora-themes, aurora-icons, aurora-wallpapers
- **Post-install:** `glib-compile-schemas /usr/share/glib-2.0/schemas/`

#### **aurora-kde-integration**
- **Purpose:** KDE Plasma-specific integration
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/kservices5/`, `/usr/share/plasma/look-and-feel/`
- **Dependencies:** plasma-framework (soft)
- **Recommends:** aurora-kde-themes, aurora-icons, aurora-cursors

#### **aurora-kde-themes**
- **Purpose:** Full KDE Plasma visual theme (workspace, colors, decorations)
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/plasma/desktoptheme/Aurora/`
- **Dependencies:** plasma-framework (soft)
- **Includes:** color scheme, window decoration, cursor theme, icon theme

#### **aurora-accessibility**
- **Purpose:** High-contrast, dyslexia-friendly, visually-impaired variants
- **Type:** Architecture-independent (`all`)
- **Install Location:** `/usr/share/themes/Aurora-HighContrast/`, `/usr/share/themes/Aurora-Dyslexia/`
- **Dependencies:** None
- **Recommends:** aurora-themes, aurora-icons, aurora-fonts

### 1.3 Package Naming Conventions

**Format:** `aurora[-component][-variant]`

Examples:
- `aurora` — meta-package
- `aurora-themes` — all themes
- `aurora-themes-gtk` — GTK themes only
- `aurora-fonts` — core fonts
- `aurora-fonts-extended` — extended font pack
- `aurora-icons-light` — light icon variant
- `aurora-accessibility-highcontrast` — high-contrast theme

**Naming Rules:**
1. Always lowercase, use hyphens (never underscores)
2. Component names are single words (themes, icons, fonts, etc.)
3. Variants go after component (light, dark, extended, beta, etc.)
4. No version info in package name (version goes in control file)
5. Follow Debian package naming policy

### 1.4 Semantic Versioning Strategy

**Version Format:** `MAJOR.MINOR.PATCH[-prerelease][+build]`

**MAJOR:** Bumped when
- Visual language fundamentally redesigned
- Installation paths change
- Breaking changes to asset compatibility
- Major UI paradigm shift

**MINOR:** Bumped when
- New component added (e.g., aurora-jetbrains)
- New features/options added to existing theme
- New color palette variants
- Extended asset collections

**PATCH:** Bumped when
- Bug fixes (broken colors, misaligned icons, etc.)
- Asset improvements without functionality change
- Documentation updates
- Performance optimizations

**Pre-release:** `1.0.0-beta.1`, `1.0.0-rc.1` (used for testing channels)

**Build Metadata:** `1.0.0+ubuntu22.04` (Ubuntu version targeting)

---

## Part 2: Debian Package Structure

### 2.1 Standard Debian Package Layout

Each Aurora package follows Debian Policy Manual §5:

```
aurora-themes-1.0.0/
├── DEBIAN/
│   ├── control              # Package metadata
│   ├── copyright            # License/copyright info
│   ├── changelog            # Version history
│   ├── postinst             # Post-installation script
│   ├── postrm               # Post-removal script
│   ├── preinst              # Pre-installation script
│   ├── prerm                # Pre-removal script
│   ├── conffiles            # Editable config files list
│   ├── md5sums              # File integrity checksums
│   └── shlibs               # Shared library info (if applicable)
├── usr/
│   └── share/
│       ├── themes/
│       │   └── Aurora/
│       │       ├── gtk-3.0/
│       │       │   ├── gtk.css
│       │       │   └── assets/
│       │       ├── gtk-4.0/
│       │       │   ├── gtk.css
│       │       │   └── assets/
│       │       ├── index.theme
│       │       └── METADATA.json
│       └── doc/
│           └── aurora-themes/
│               ├── copyright
│               └── changelog.gz
├── etc/                     # Configuration files (if any)
└── var/                     # Variable data (if any)
```

### 2.2 DEBIAN/control File Template

```
Package: aurora-themes
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Vcs-Browser: https://github.com/aurora-linux/aurora
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}
Recommends: aurora-icons, aurora-cursors
Suggests: aurora-fonts
Conflicts: aurora-themes-old (<< 0.9.0)
Breaks: gnome-shell (<< 40)
Replaces: aurora-themes-old

Description: Aurora Linux design system — GTK/Qt/Plasma themes
 Aurora is a comprehensive Linux design system providing cohesive visual
 experience across desktop environments.
 .
 This package contains core theme files for:
  - GTK 3 and GTK 4
  - Qt 5 and Qt 6
  - KDE Plasma
 .
 Install this package to use Aurora themes on your desktop.
```

### 2.3 DEBIAN/copyright File

Debian copyright file format (Debian DEP 5):

```
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: Aurora
Upstream-Contact: Aurora Team <aurora@example.com>
Source: https://github.com/aurora-linux/aurora
Comment: Aurora Linux Design System
 A comprehensive, open-source design system for Linux desktops.

Files: *
Copyright: 2024-2026 Aurora Team
License: MIT
 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:
 .
 [Full license text]

Files: docs/assets/wallpapers/*
Copyright: 2024-2026 Aurora Contributors
License: CC-BY-4.0
 Creative Commons Attribution 4.0 International Public License
 .
 [Full license text]

Files: debian/*
Copyright: 2024 Aurora Team
License: MIT
```

### 2.4 DEBIAN/postinst Script

```bash
#!/bin/bash
set -e

case "$1" in
  configure)
    # Update GTK icon cache
    if command -v update-icon-caches &> /dev/null; then
      update-icon-caches /usr/share/icons/Aurora 2>/dev/null || true
    fi

    # Update font cache
    if command -v fc-cache &> /dev/null; then
      fc-cache -fv /usr/share/fonts/aurora 2>/dev/null || true
    fi

    # Log installation
    echo "Aurora themes installed successfully"
    ;;

  abort-upgrade|abort-remove|abort-deconfigure)
    ;;

  *)
    echo "postinst called with unknown argument \`$1'" >&2
    exit 1
    ;;
esac

exit 0
```

**Why each section:**
- `configure`: Runs after package extracted; performs setup
- `abort-*`: Handles failed installation/upgrade recovery
- `set -e`: Exit immediately on error (prevents partial installs)

### 2.5 DEBIAN/postrm Script

```bash
#!/bin/bash
set -e

case "$1" in
  remove|purge)
    # Rebuild caches after removal
    if command -v update-icon-caches &> /dev/null; then
      update-icon-caches /usr/share/icons 2>/dev/null || true
    fi

    if command -v fc-cache &> /dev/null; then
      fc-cache -fv 2>/dev/null || true
    fi
    ;;

  upgrade|failed-upgrade|disappear)
    ;;

  *)
    echo "posters called with unknown argument \`$1'" >&2
    exit 1
    ;;
esac

exit 0
```

### 2.6 DEBIAN/preinst Script

```bash
#!/bin/bash
set -e

case "$1" in
  install|upgrade)
    # Pre-flight checks
    if [ ! -d /usr/share/themes ]; then
      mkdir -p /usr/share/themes
    fi
    ;;

  abort-upgrade)
    ;;

  *)
    echo "preinst called with unknown argument \`$1'" >&2
    exit 1
    ;;
esac

exit 0
```

### 2.7 DEBIAN/conffiles

For packages with user-editable configuration:

```
/etc/aurora/theme.conf
/etc/aurora/colors.yaml
```

(Only if Aurora theme is customizable; typically not needed for asset-only packages)

### 2.8 Directory Structure Rationale

| Directory | Purpose | Why |
|-----------|---------|-----|
| `/usr/share/themes/` | GTK/Qt theme files | Debian standard location for desktop themes; read by GTK/Qt at runtime |
| `/usr/share/icons/` | Icon assets | Standard location; cached by icon systems; enables fallback chains |
| `/usr/share/fonts/aurora/` | Font files | Isolated to prevent conflicts; `fc-cache` scans all subdirs of `/usr/share/fonts/` |
| `/usr/share/backgrounds/` | Wallpapers | Standard location; scanned by GNOME Settings, KDE System Settings |
| `/usr/share/plasma/` | KDE assets | KDE Plasma looks here for themes, look-and-feel packages |
| `/usr/share/gnome-shell/` | GNOME assets | GNOME Shell searches here for extensions, themes |
| `/usr/share/plymouth/themes/` | Boot splash | Plymouth's standard theme directory |
| `/etc/` | Configuration | Only for user-editable settings (rare for theme-only packages) |
| `/usr/share/doc/aurora-*/` | Documentation | Debian requirement; changelog, copyright, README |

---

## Part 3: Meta-Package Strategy

### 3.1 Aurora Meta-Package Design

The `aurora` meta-package is a convenience package (no files, ~5 KB) that depends on core components:

```
Package: aurora
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Depends: aurora-themes, aurora-icons, aurora-cursors, aurora-fonts, 
         aurora-colors, aurora-wallpapers, aurora-branding
Recommends: aurora-terminal-themes, aurora-vscode, aurora-jetbrains
Suggests: aurora-kde-themes, aurora-sddm, aurora-gdm, aurora-accessibility
Conflicts: aurora << 0.9.0
Description: Aurora Linux Design System — Complete package
 Aurora is a comprehensive Linux design system providing a cohesive visual
 experience across all major desktop environments and applications.
 .
 This meta-package installs all core Aurora components. For selective
 installation, see:
 .
 - aurora-themes: GTK/Qt/Plasma themes
 - aurora-icons: Icon themes
 - aurora-fonts: Typography
 - aurora-wallpapers: Backgrounds
 - aurora-kde-themes: Full KDE integration
 - aurora-vscode: VS Code theme
 .
 For more details, visit: https://github.com/aurora-linux/aurora
```

### 3.2 Installation Scenarios

**Full Desktop Experience:**
```bash
sudo apt install aurora
# Installs: themes + icons + cursors + fonts + colors + wallpapers + branding
```

**Minimal (Just Themes):**
```bash
sudo apt install aurora-themes
```

**Developer Setup:**
```bash
sudo apt install aurora-themes aurora-icons aurora-fonts aurora-vscode aurora-jetbrains
```

**KDE-Focused:**
```bash
sudo apt install aurora-kde-themes aurora-sddm aurora-kdm-integration
```

**GNOME-Focused:**
```bash
sudo apt install aurora-themes aurora-gdm aurora-gnome-integration
```

### 3.3 Upgrade Path Management

When upgrading `aurora` meta-package:
- Old version: `aurora (1.0.0)` → Depends on old component versions
- New version: `aurora (1.1.0)` → Depends on new component versions

APT automatically pulls in new component versions when upgrading.

**Strategy for Deprecations:**
If old package is deprecated (e.g., `aurora-themes-old` → `aurora-themes`):
```
Replaces: aurora-themes-old
Breaks: aurora-themes-old
```

This allows clean transition; APT will automatically remove old package.

---

## Part 4: Dependency Strategy

### 4.1 Dependency Matrix

```
                    Depends      Recommends          Suggests
aurora              ✓ core       ○ terminal, vscode  □ kde, gdm, a11y
aurora-themes       – (none)     ✓ icons, cursors    □ fonts
aurora-icons        – (none)     ✓ themes            □ cursors
aurora-cursors      – (none)     ✓ icons, themes     □ –
aurora-fonts        fontconfig   – (none)            □ themes
aurora-wallpapers   – (none)     ✓ themes            □ –
aurora-colors       – (none)     – (none)            □ themes
aurora-branding     – (none)     ✓ fonts             □ –
aurora-terminal     – (none)     – (none)            □ fonts
aurora-vscode       – (none)     ✓ fonts             □ –
aurora-jetbrains    – (none)     ✓ fonts             □ –
aurora-kde-themes   plasma-fw    ✓ icons, cursors    □ sddm
aurora-sddm         sddm         ✓ kde-themes        □ –
aurora-gdm          gdm          ✓ themes, icons     □ –

Legend: ✓ = hard dependency | ○ = recommended | □ = optional
```

### 4.2 Depends (Hard Dependencies)

**Use when:**
- Package cannot function without the dependency
- Runtime will fail or crash without it

**Examples:**
```
aurora-fonts:
  Depends: fontconfig

aurora-gdm:
  Depends: gdm (>= 3.36)
```

**Why minimal?**
- Most Aurora packages are pure assets (GTK CSS, PNG icons, fonts)
- No runtime dependencies needed; themes are static files
- Users might have custom desktop environments; don't force heavy deps

### 4.3 Recommends (Important But Not Required)

**Use when:**
- Package works better with this, but functions without it
- Should be installed on most systems
- Users rarely want to exclude it

**Examples:**
```
aurora-themes:
  Recommends: aurora-icons, aurora-cursors

aurora:
  Recommends: aurora-terminal-themes, aurora-vscode, aurora-jetbrains
```

**APT behavior:** `apt install aurora` will install Recommends by default (unless `APT::Install-Recommends "false"` in apt.conf)

### 4.4 Suggests (Optional Enhancements)

**Use when:**
- Package is completely standalone
- Enhancement is only for specific use cases
- Users might reasonably exclude it

**Examples:**
```
aurora:
  Suggests: aurora-kde-themes, aurora-sddm, aurora-accessibility

aurora-themes:
  Suggests: aurora-accessibility-highcontrast
```

**APT behavior:** `apt install aurora` will NOT install Suggests (user must explicitly request or configure)

### 4.5 Conflicts (Incompatible Packages)

**Use when:**
- Cannot have both packages installed simultaneously
- Packages modify the same file
- Runtime conflict will occur

**Example:**
```
aurora-themes:
  Conflicts: ubuntu-themes (<< 16.04)
```

**Behavior:** APT will refuse to install if conflicting package is already installed (user must remove old one first)

### 4.6 Breaks (Dependency Breaks)

**Use when:**
- This package is incompatible with specific versions of another package
- Newer code won't work with old API/behavior

**Example:**
```
aurora-gnome-integration:
  Breaks: gnome-shell (<< 40)
```

**Behavior:** APT will upgrade the broken package automatically if installing this

### 4.7 Replaces (Supersession)

**Use when:**
- This package replaces an older or differently-named package
- Wants to absorb files from the old package

**Example:**
```
aurora-themes:
  Replaces: aurora-themes-legacy (<< 0.8.0)
```

**Behavior:** APT allows overwriting files from replaced package during upgrade

### 4.8 Architecture-Specific Dependencies

Most Aurora packages are `Architecture: all` (not architecture-specific). However:

```
aurora-vscode:
  Depends: fontconfig | fonts-noto
  # Use OR (|) for optional alternatives
  
aurora-kde-themes:
  Depends: plasma-framework (>= 5.80) | plasma-framework (>= 6.0)
  # Use OR for version ranges
```

### 4.9 Dependency Version Specifications

**Syntax:**
```
Depends: package (>> version)     # Strictly greater
Depends: package (>= version)     # Greater or equal
Depends: package (<< version)     # Strictly less
Depends: package (<= version)     # Less or equal
Depends: package (= version)      # Exactly equal (rare)
Depends: package (| alternative)  # OR operator
```

**Example:**
```
aurora-fonts:
  Depends: fontconfig (>= 2.11), 
           fonts-liberation | fonts-noto

aurora-kde-themes:
  Depends: plasma-framework (>= 5.80)
```

---

## Part 5: Building Packages

### 5.1 Build Pipeline Overview

```
Source Code (Git)
    ↓
Extract assets & prepare debian/
    ↓
dpkg-buildpackage (with debhelper)
    ↓
Package validation (lintian, tests)
    ↓
Sign .deb files (dpkg-sig)
    ↓
Repository (aptly/reprepro)
    ↓
Serve via HTTP/S
```

### 5.2 Build Tools & Versions

| Tool | Version | Purpose |
|------|---------|---------|
| dpkg-buildpackage | 1.21+ | Master build orchestrator |
| debhelper | 13+ | Debian packaging helpers, dh_* commands |
| fakeroot | 1.25+ | Simulate root permissions for file ownership |
| lintian | 2.113+ | Package validation & policy checking |
| dh-make | (optional) | Generate debian/ directory from scratch |
| git-buildpackage | 0.31+ | (Optional) Automate building from git |

### 5.3 debian/ Directory Structure

```
debian/
├── changelog            # Version history (Debian format)
├── control              # Package metadata
├── copyright            # License info
├── rules                # Build rules (Makefile-like)
├── postinst             # Post-install script
├── postrm               # Post-remove script
├── preinst              # Pre-install script
├── prerm                # Pre-remove script
├── install              # File installation manifest
├── dirs                 # Directory creation list
├── conffiles            # Configuration file list
├── source/format        # Source package format (3.0 native/quilt)
├── source/options       # dpkg-source options
└── [package].desktop    # .desktop file (if applicable)
```

### 5.4 debian/rules File

The `debian/rules` file is the build orchestrator:

```makefile
#!/usr/bin/make -f
# Aurora themes build rules

export DH_VERBOSE = 1
export DH_OPTIONS = -v

%:
	dh $@

override_dh_auto_build:
	# Custom build steps (asset validation, CSS linting, etc.)
	@echo "Validating Aurora assets..."
	./scripts/validate-assets.sh
	@echo "Building theme files..."
	./scripts/build-themes.sh

override_dh_auto_install:
	# Install files to debian/aurora-themes/
	mkdir -p debian/aurora-themes/usr/share/themes
	cp -r Aurora/* debian/aurora-themes/usr/share/themes/Aurora/

override_dh_strip:
	# Disable binary stripping (no binaries, only assets)

override_dh_compress:
	dh_compress --exclude=.css --exclude=.json
	# Don't compress CSS/JSON (browsers need them readable)
```

**Key sections:**
- `%: dh $@` — Use debhelper defaults for most targets
- `override_dh_auto_build` — Custom build (asset validation)
- `override_dh_auto_install` — Copy files to staging directory
- `override_dh_strip` — Skip binary stripping (no binaries)
- `override_dh_compress` — Skip compressing web assets

### 5.5 Building a Package

```bash
# 1. Ensure dependencies installed
sudo apt install debhelper dh-make lintian

# 2. Build the package
dpkg-buildpackage -us -uc
# -us = unsigned source
# -uc = unsigned changes file

# 3. Or use git-buildpackage (if using git)
gbp buildpackage --git-pbuilder

# 4. Result: .deb file created
ls -lh ../aurora-themes_1.0.0_all.deb
```

### 5.6 Validation with lintian

```bash
# Check for policy violations
lintian -i aurora-themes_1.0.0_all.deb

# Example output:
# E: aurora-themes: wrong-file-format ./usr/share/themes/Aurora/gtk.css

# Common issues:
# - Executable bit on CSS files (should be 644)
# - Missing copyright header
# - Wrong permissions on symlinks
# - Uncompressed man pages
```

**Critical lintian checks:**
```bash
lintian -EviI aurora-themes_1.0.0_all.deb
# -E = show errors (fail if present)
# -v = verbose
# -i = info + warnings
# -I = ignore pedantic checks (sometimes too strict)
```

### 5.7 Signing Packages

```bash
# Sign .deb file with GPG key
dpkg-sig --sign builder -k aurora@example.com aurora-themes_1.0.0_all.deb

# Verify signature
dpkg-sig --verify aurora-themes_1.0.0_all.deb

# Output:
# Processing aurora-themes_1.0.0_all.deb...
# GOODSIG 0x1234567890ABCDEF aurora@example.com (timestamp)
```

### 5.8 Build Validation Checklist

Before releasing, verify:

- [ ] All lintian errors resolved
- [ ] Package installs without errors: `sudo dpkg -i .deb`
- [ ] Post-install scripts run successfully
- [ ] Files appear in correct locations: `dpkg -L aurora-themes`
- [ ] Icon/font caches updated properly
- [ ] No hardcoded paths (should use /usr/share/, not /opt/)
- [ ] Permissions correct (644 for files, 755 for dirs)
- [ ] No architecture-dependent binaries (for `all` packages)
- [ ] GPG signature valid
- [ ] Changelog entry present
- [ ] Copyright file complete
- [ ] Conflicts/Breaks/Replaces accurate

---

## Part 6: Repository Generation

### 6.1 Tool Comparison

| Tool | Complexity | Speed | Scalability | Maintenance | Learning Curve |
|------|-----------|-------|-------------|-------------|-----------------|
| **aptly** | Medium | Fast | Good (100k+ pkgs) | Excellent | Medium |
| **reprepro** | Low | Very Fast | Good | Good | Low |
| **apt-ftparchive** | Very Low | Very Fast | Excellent | Minimal | Very Low |
| **mini-dinstall** | Low | Slow | Fair | Fair | Low |

### 6.2 Recommended: aptly

**Why aptly for Aurora:**
- Designed for complex workflows (multiple suites, snapshots, mirrors)
- Excellent rollback capability (snapshots)
- Supports multiple GPG keys
- Built-in publishing pipeline
- JSON API for CI/CD integration
- Strong community support

**When to use alternatives:**
- `apt-ftparchive`: Very high volume (100k+ packages), minimal overhead
- `reprepro`: Preference for traditional approach, smaller scale
- `mini-dinstall`: Legacy systems, very lightweight needs

### 6.3 aptly Setup

```bash
# Install aptly
sudo apt install aptly

# Configure ~/.aptly.conf
{
  "architectures": ["all", "amd64", "i386", "arm64"],
  "dependencyFollowSuggests": false,
  "dependencyFollowRecommends": false,
  "dependencyFollowAllVariants": false,
  "dependencyFollowSource": false,
  "gpgDisableSign": false,
  "gpgDisableVerify": false,
  "gpgPersonalKey": "aurora@example.com",
  "downloadSourcePackages": true,
  "skipContentsPublishing": false,
  "ppaDistributorID": "ubuntu",
  "ppaCodename": "aurora",
  "downloadWithSource": false,
  "skipLegacyPool": true,
  "FileSystemPublishEndpoints": {
    "filesystem": {
      "rootDir": "/var/www/aurora-repo",
      "linkMethod": "hardlink"
    }
  }
}
```

### 6.4 Repository Workflow with aptly

```bash
# Create repositories for different suites
aptly repo create -architectures="all,amd64,i386,arm64" aurora-stable
aptly repo create -architectures="all,amd64,i386,arm64" aurora-testing
aptly repo create -architectures="all,amd64,i386,arm64" aurora-unstable

# Add package to testing
aptly repo add aurora-testing ./aurora-themes_1.0.0_all.deb

# Create snapshot (for rollback capability)
aptly snapshot create aurora-testing-snap1 from repo aurora-testing

# Promote snapshot to stable after testing
aptly snapshot merge aurora-testing-snap1 aurora-stable-snap1 aurora-stable-snap1

# Publish (generate Packages.gz, Release, etc.)
aptly publish snapshot aurora-testing-snap1 filesystem:filesystem
aptly publish snapshot aurora-stable-snap1 filesystem:filesystem
```

### 6.5 Repository Structure Output

After publishing with aptly:

```
/var/www/aurora-repo/
├── pool/
│   └── main/
│       ├── a/
│       │   └── aurora-themes/
│       │       └── aurora-themes_1.0.0_all.deb
│       ├── a/
│       │   └── aurora-icons/
│       │       └── aurora-icons_1.0.0_all.deb
│       └── ...
├── dists/
│   ├── stable/
│   │   ├── Release
│   │   ├── Release.gpg
│   │   ├── InRelease
│   │   ├── main/
│   │   │   ├── binary-all/
│   │   │   │   ├── Packages
│   │   │   │   ├── Packages.gz
│   │   │   │   └── Packages.xz
│   │   │   ├── binary-amd64/
│   │   │   │   ├── Packages
│   │   │   │   ├── Packages.gz
│   │   │   │   └── Packages.xz
│   │   │   └── source/
│   │   │       ├── Sources
│   │   │       ├── Sources.gz
│   │   │       └── Sources.xz
│   │   └── Contents-amd64
│   ├── testing/
│   │   └── [similar structure]
│   └── unstable/
│       └── [similar structure]
└── indices/
    └── override.ubuntu
```

### 6.6 Advantages & Disadvantages

#### aptly Advantages:
✓ Snapshots for rollback
✓ Multiple source/destination support
✓ API-first design
✓ Complex promotion workflows
✓ Excellent for CI/CD
✓ Key rotation without re-signing

#### aptly Disadvantages:
✗ Heavier resource usage than apt-ftparchive
✗ Steeper learning curve
✗ Requires disk space for snapshots

#### apt-ftparchive Advantages:
✓ Minimal overhead
✓ Very fast
✓ Part of official APT tools
✓ Perfect for pure generation

#### apt-ftparchive Disadvantages:
✗ No rollback capability (snapshots)
✗ Less suited for complex workflows
✗ Limited promotion features

---

## Part 7: Repository Layout

### 7.1 Complete Repository Structure

```
/var/www/aurora-repo/              # Repository root (serves as http://archive.aurora.linux/)
├── pool/                           # Pool directory (contains actual .deb files)
│   ├── main/                       # Main component (stable packages)
│   │   ├── a/
│   │   │   ├── aurora/
│   │   │   │   ├── aurora_1.0.0_all.deb
│   │   │   │   ├── aurora_1.0.1_all.deb
│   │   │   │   └── aurora_1.1.0_all.deb
│   │   │   └── aurora-accessibility/
│   │   │       └── aurora-accessibility_1.0.0_all.deb
│   │   ├── a/ (rest of alphabet)
│   │   └── z/
│   ├── contrib/                    # Contributed packages (optional)
│   └── non-free/                   # Non-free packages (if any)
├── dists/                           # Distribution metadata
│   ├── stable/                      # Latest stable release
│   │   ├── Release                  # Unsigned release file
│   │   ├── Release.gpg              # Detached GPG signature
│   │   ├── InRelease                # Inline signed (Release + signature)
│   │   ├── main/
│   │   │   ├── binary-all/
│   │   │   │   ├── Packages         # Package metadata (uncompressed)
│   │   │   │   ├── Packages.gz      # Package metadata (gzip)
│   │   │   │   └── Packages.xz      # Package metadata (xz)
│   │   │   ├── binary-amd64/
│   │   │   │   ├── Packages
│   │   │   │   ├── Packages.gz
│   │   │   │   └── Packages.xz
│   │   │   ├── binary-i386/
│   │   │   ├── binary-arm64/
│   │   │   ├── source/
│   │   │   │   ├── Sources
│   │   │   │   ├── Sources.gz
│   │   │   │   └── Sources.xz
│   │   │   ├── i18n/
│   │   │   │   └── Translation-en    # Localized descriptions
│   │   │   └── Contents-amd64        # File-to-package mapping
│   │   └── contrib/
│   ├── testing/                     # Testing release
│   │   └── [similar structure as stable]
│   ├── unstable/                    # Development/nightly
│   │   └── [similar structure as stable]
│   └── experimental/                # Cutting-edge (optional)
├── indices/                         # Debian override files
│   ├── override.ubuntu              # Priority/section overrides
│   └── override.debian              # For Debian target (if applicable)
├── .htaccess                        # Web server config (if using Apache)
├── README.md                        # How to use this repository
├── SIGNING_KEY.txt                  # Public key export (documentation)
└── MIRRORS.txt                      # Mirror list (if applicable)
```

### 7.2 File Descriptions

#### pool/main/a/aurora/
**Contains:** All versions of the `aurora` meta-package
**Naming:** `aurora_VERSION_ARCH.deb`
**Examples:** `aurora_1.0.0_all.deb`, `aurora_1.0.1_all.deb`
**Why:** First-letter alphabetical organization; standard in Debian

#### dists/stable/Release
**Contains:** Metadata about the entire stable release
**Format:** RFC 822-style key-value pairs
**Example:**
```
Origin: Aurora Linux
Label: Aurora Stable Repository
Suite: stable
Codename: stable
Date: Sat, 02 Aug 2026 12:00:00 +0000
Valid-Until: Sat, 09 Aug 2026 12:00:00 +0000
Architectures: all amd64 i386 arm64
Components: main contrib non-free
Description: Aurora Linux Design System repository
MD5Sum:
 1234567890abcdef1234567890abcdef     1234 main/binary-all/Packages
 fedcba0987654321fedcba0987654321     5678 main/binary-all/Packages.gz
SHA256:
 abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890  1234 main/binary-all/Packages
 ...
```

#### dists/stable/Release.gpg
**Contains:** Detached GPG signature of Release file
**Created by:** `gpg --detach-sign -o Release.gpg Release`
**Verified by:** `gpg --verify Release.gpg Release`
**Why:** Allows verification without inline signature (older systems)

#### dists/stable/InRelease
**Contains:** Release file with inline GPG signature
**Created by:** `gpg --clearsign -o InRelease Release`
**Verified by:** `gpg --verify InRelease`
**Why:** Modern systems prefer this (single file, clearer chain of trust)

#### dists/stable/main/binary-all/Packages
**Contains:** Metadata for all architecture-independent packages
**Format:** Debian control file format
**Example:**
```
Package: aurora-themes
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Installed-Size: 5120
Homepage: https://github.com/aurora-linux/aurora
Priority: optional
Section: misc
Filename: pool/main/a/aurora-themes/aurora-themes_1.0.0_all.deb
Size: 1234567
MD5sum: 1234567890abcdef1234567890abcdef
SHA1: fedcba0987654321fedcba0987654321fedcba09
SHA256: abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
Description: Aurora Linux design system — GTK/Qt/Plasma themes
 Aurora is a comprehensive Linux design system providing cohesive visual
 experience across desktop environments.
```

#### dists/stable/main/binary-all/Packages.gz
**Contains:** gzip-compressed Packages file
**Why:** Reduce bandwidth (150KB → 40KB typical)
**APT behavior:** `apt-get update` downloads .gz version, decompresses locally

#### dists/stable/main/source/Sources
**Contains:** Metadata for source packages (.dsc, .orig.tar.gz, etc.)
**Note:** Aurora provides source packages for transparency & reproducibility

#### dists/stable/main/Contents-amd64
**Contains:** Mapping of files to packages (for `apt-file search`)
**Example:**
```
FILE                                        SECTION PACKAGE
usr/share/themes/Aurora/gtk-3.0/gtk.css    misc    aurora-themes
usr/share/icons/Aurora/16x16/apps/...      misc    aurora-icons
```
**Why:** Allows `apt-file search gtk.css` to find package without downloading full metadata

### 7.3 Architecture Support

Aurora repository provides packages for:

| Architecture | Format | Use Case |
|--------------|--------|----------|
| `all` | `aurora-themes_1.0.0_all.deb` | Architecture-independent (themes, fonts, icons) |
| `amd64` | `somepackage_1.0.0_amd64.deb` | 64-bit Intel/AMD (if binary packages added later) |
| `i386` | `somepackage_1.0.0_i386.deb` | 32-bit Intel/AMD (legacy support) |
| `arm64` | `somepackage_1.0.0_arm64.deb` | 64-bit ARM (Raspberry Pi 4+, Apple Silicon) |
| `armhf` | `somepackage_1.0.0_armhf.deb` | 32-bit ARM (Raspberry Pi 3) |

For Aurora's pure asset packages, only `all` is needed initially.

### 7.4 Suite/Codename Strategy

**Codenames** (like Ubuntu's "Noble Numbat"):
- `stable` → Current production release (e.g., `1.0`)
- `testing` → Upcoming release being tested (e.g., `1.1-beta`)
- `unstable` → Development/nightly (e.g., `1.2-dev`)
- `experimental` → Cutting-edge features (optional)

**Why separate suites:**
- Users can choose stability level
- Testing/unstable can be aggressive with new features
- Rollback to stable always available
- CI/CD can test across all three

---

## Part 8: Repository Signing

### 8.1 GPG Key Infrastructure

**Three-key strategy:**
1. **Repository Key** — Signs Release files (public-facing)
2. **Signing Key** — Signs individual .deb packages (optional, for transparency)
3. **Backup Key** — Rotated periodically, offline storage

### 8.2 Create Repository GPG Key

```bash
# Generate key (one-time, 4096-bit RSA, 4-year validity)
gpg --full-generate-key

# Name: Aurora Linux Repository
# Email: aurora@example.com
# Passphrase: Store in secrets manager, NEVER commit to git

# Export public key for distribution
gpg --export -a aurora@example.com > aurora-archive-keyring.gpg

# Export key ID (short form)
gpg --list-keys aurora@example.com
# pub   rsa4096 2026-08-02 [SC] [expires: 2030-08-01]
#       1234567890ABCDEF1234567890ABCDEF12345678
# uid           [ultimate] Aurora Linux Repository <aurora@example.com>

# Store key ID
KEY_ID="1234567890ABCDEF1234567890ABCDEF12345678"
```

### 8.3 Sign Release File

```bash
# Detached signature (Release.gpg)
gpg --default-key aurora@example.com \
    --detach-sign \
    --armor \
    -o dists/stable/Release.gpg \
    dists/stable/Release

# Inline signature (InRelease)
gpg --default-key aurora@example.com \
    --clearsign \
    --armor \
    -o dists/stable/InRelease \
    dists/stable/Release

# Verify signature
gpg --verify dists/stable/Release.gpg dists/stable/Release
# Output: Good signature from "Aurora Linux Repository <aurora@example.com>"
```

### 8.4 Sign Individual Packages (Optional)

```bash
# Sign .deb package
dpkg-sig --sign builder \
         -k aurora@example.com \
         aurora-themes_1.0.0_all.deb

# Verify
dpkg-sig --verify aurora-themes_1.0.0_all.deb
# Output: GOODSIG

# Note: Most users don't verify individual package signatures;
# Release file signature is sufficient
```

### 8.5 User Key Import & Repository Addition

**User-facing workflow:**

```bash
# Step 1: Download & import Aurora repository key
wget https://archive.aurora.linux/aurora-archive-keyring.gpg
sudo apt-key add aurora-archive-keyring.gpg

# Alternative (modern method using keyrings):
sudo mkdir -p /usr/share/keyrings
sudo cp aurora-archive-keyring.gpg /usr/share/keyrings/aurora-archive-keyring.gpg

# Step 2: Add repository to sources
echo "deb [signed-by=/usr/share/keyrings/aurora-archive-keyring.gpg] https://archive.aurora.linux/dists/stable main" | \
  sudo tee /etc/apt/sources.list.d/aurora.sources

# Step 3: Update package lists
sudo apt update

# Step 4: Install
sudo apt install aurora
```

**Or, create an `aurora-archive-keyring` package:**

```
Package: aurora-archive-keyring
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Depends: debian-archive-keyring | ubuntu-keyring
Description: Aurora repository signing key

# debian/postinst:
install -D -m 0644 keyrings/aurora.gpg \
  /usr/share/keyrings/aurora-archive-keyring.gpg

# Automatically installs keyring on `apt install aurora-archive-keyring`
```

### 8.6 Key Rotation Strategy

**Scheduled rotation (every 2 years):**

```
Current Key (1.0–2.0): 1234567890ABCDEF... (expires 2030-08-01)
   ↓ (2028, one year before expiration)
Rotation: Generate new key + sign with old key
   ↓ Overlap period (old + new keys both valid)
Users update keyring package
   ↓ (2028-08-02, after expiration of old key)
Old key retired; new key primary

dists/stable/Release signed by: NEW_KEY
(can still verify old packages with old key in keyring)
```

**Emergency rotation (key compromise):**
1. Immediately revoke compromised key: `gpg --gen-revoke aurora@example.com > revoked.asc`
2. Generate new key
3. Re-sign all Release files with new key
4. Publish revocation certificate
5. Notify users of key change

### 8.7 Key Security Best Practices

✓ **Do:**
- Store private key in secrets manager (HashiCorp Vault, AWS Secrets Manager, GitHub Secrets)
- Require signing key passphrase
- Use 4096-bit RSA minimum
- Rotate keys every 2 years
- Keep backup key offline in secure location
- Use subkeys for signing (master key for certification only, rarely needed)
- Publish key fingerprint prominently on website

✗ **Don't:**
- Commit private key to Git repository
- Store key in environment variables in plain text
- Use same key across multiple organizations
- Reuse key for personal & repository signing
- Ignore key expiration
- Sign packages without verification
- Store key on shared systems without encryption

### 8.8 GPG Subkey Strategy (Advanced)

For large-scale repositories, use GnuPG subkey architecture:

```
Primary Key (Master)
  ├── Certification Subkey (C) — Create/revoke UIDs
  ├── Signing Subkey (S)       — Sign Release files
  ├── Encryption Subkey (E)    — Encrypt backups
  └── Authentication Subkey (A) — Authentication protocol
```

**Advantages:**
- Primary key stays offline
- Signing key can be rotated without regenerating identity
- Compromise of signing key doesn't compromise entire keyring
- Complex, but industry standard for critical infrastructure

---

## Part 9: Hosting Architecture

### 9.1 Hosting Options Comparison

#### Option 1: GitHub Pages (Recommended for small-medium)
**Cost:** Free
**Bandwidth:** 1GB/month soft limit (then slowdown)
**Setup:** Push to `gh-pages` branch
**HTTPS:** Automatic (GitHub's certificate)
**CDN:** GitHub's CDN (fast globally)
**Scalability:** ~10k concurrent users

**Pros:**
✓ Free forever
✓ Automatic HTTPS
✓ GitHub-integrated CI/CD
✓ No server maintenance
✓ Excellent global CDN

**Cons:**
✗ 1GB/month soft limit (not hard, but throttled)
✗ Limited to static content
✗ GitHub-hosted (not independent)

**Architecture:**
```
Push to main
  ↓ (GitHub Actions)
Build packages
  ↓
Generate repo (aptly)
  ↓
Push to gh-pages branch
  ↓
Served via https://aurora-linux.github.io/repo/
```

---

#### Option 2: Cloudflare R2 (Recommended for medium-large)
**Cost:** $0.015/GB stored, $0.15/GB egress (but can use free tier with free egress via Cloudflare Workers)
**Bandwidth:** Unlimited (via Workers, ~50 req/min limit)
**Setup:** S3-compatible API
**HTTPS:** Automatic (Cloudflare TLS)
**CDN:** Cloudflare's global network (300+ data centers)
**Scalability:** 100k+ concurrent users

**Pros:**
✓ Low cost (R2 cheaper than S3)
✓ Unlimited bandwidth on Cloudflare Workers
✓ Global CDN included
✓ S3-compatible API (easy migration)
✓ DDoS protection built-in

**Cons:**
✗ Slightly more complex setup (Cloudflare Workers)
✗ R2 is Cloudflare-specific (not standard S3)

**Architecture:**
```
GitHub Actions
  ↓
Upload .deb files to Cloudflare R2
  ↓
Generate repo metadata (aptly)
  ↓
Push to R2
  ↓
Cloudflare Workers (public gateway)
  ↓
Served via https://archive.aurora.linux/ (CNAME to Cloudflare)
```

**Cloudflare Worker script:**
```javascript
export default {
  async fetch(request) {
    return fetch(
      new Request(
        new URL(request.url).pathname.slice(1),
        request
      ),
      {
        cf: { cacheTtl: 3600 }
      }
    );
  }
};
```

---

#### Option 3: AWS S3 + CloudFront (Enterprise)
**Cost:** $0.023/GB stored (S3), $0.085/GB egress (CloudFront cheaper in bulk)
**Bandwidth:** Unlimited
**Setup:** AWS console or Terraform
**HTTPS:** CloudFront certificate
**CDN:** CloudFront (200+ edge locations)
**Scalability:** Unlimited

**Pros:**
✓ Industry standard, proven
✓ Unlimited bandwidth
✓ Fine-grained access control (IAM)
✓ CloudFront caching strategies
✓ Easy integration with AWS ecosystem

**Cons:**
✗ Higher cost (~$50–200/month for typical usage)
✗ More complex setup (AWS account, IAM, etc.)
✗ Needs AWS expertise

**Architecture:**
```
GitHub Actions → AWS S3 bucket
  ↓ (CI/CD role with S3 PutObject permission)
S3 stores .deb files + repo metadata
  ↓
CloudFront distribution (origin: S3 bucket)
  ↓
Served via https://archive.aurora.linux/
```

**Terraform example:**
```hcl
resource "aws_s3_bucket" "aurora_repo" {
  bucket = "aurora-repo"
}

resource "aws_cloudfront_distribution" "aurora" {
  origin {
    domain_name = aws_s3_bucket.aurora_repo.bucket_regional_domain_name
    origin_id   = "s3-aurora"

    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.aurora.cloudfront_access_identity_path
    }
  }

  enabled             = true
  default_root_object = "dists/stable/Release"

  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "s3-aurora"

    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }

    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 86400
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }
}
```

---

#### Option 4: DigitalOcean Spaces (Simple VPS)
**Cost:** $6.50/month (250GB, but blocks at that limit)
**Bandwidth:** Included (transferred to visitors, external transfers charged)
**Setup:** S3-compatible API
**HTTPS:** DigitalOcean CDN (optional, +$5/month)
**CDN:** Basic regional (3 data centers)
**Scalability:** 10k–50k concurrent users

**Pros:**
✓ Cheap ($6.50/month)
✓ S3-compatible (easy setup)
✓ Managed (no VPS to maintain)
✓ Bandwidth included

**Cons:**
✗ 250GB storage limit (blocks beyond)
✗ Limited CDN (DigitalOcean data centers only)
✗ Not as fast as Cloudflare/AWS

**Architecture:**
```
GitHub Actions → DigitalOcean Spaces
  ↓
S3-compatible API (aws s3 sync ...)
  ↓
Served via https://aurora-repo.nyc3.digitaloceanspaces.com/
```

---

#### Option 5: Self-Hosted VPS (Full Control)
**Cost:** $10–50/month (VPS) + $50–200/year (domain, TLS cert)
**Bandwidth:** Limited by VPS provider (typically 10–100TB/month)
**Setup:** Manual (Nginx, Apt setup)
**HTTPS:** Let's Encrypt (free, auto-renew)
**CDN:** None (unless integrated separately)
**Scalability:** 1k–10k concurrent users

**Pros:**
✓ Full control over infrastructure
✓ No vendor lock-in
✓ Can optimize heavily (caching, compression)
✓ Educational value

**Cons:**
✗ Operational overhead (server maintenance, security)
✗ No global CDN (slow for distant users)
✗ Bandwidth limits
✗ Self-responsible for security/backups

**Architecture:**
```
Linode/Vultr/Hetzner VPS (Ubuntu 24.04)
  ├── Nginx (reverse proxy, caching)
  ├── aptly (repository generation)
  └── /var/www/aurora-repo (served via HTTP/S)

CNAME archive.aurora.linux → VPS IP
Let's Encrypt for TLS certificate
```

**Nginx configuration:**
```nginx
server {
  listen 443 ssl http2;
  server_name archive.aurora.linux;

  ssl_certificate /etc/letsencrypt/live/archive.aurora.linux/fullchain.pem;
  ssl_certificate_key /etc/letsencrypt/live/archive.aurora.linux/privkey.pem;

  # Gzip compression
  gzip on;
  gzip_types text/plain text/css application/json;

  # Cache Packages files for 1 hour
  location ~ ^/dists/ {
    root /var/www/aurora-repo;
    expires 1h;
    add_header Cache-Control "public, max-age=3600";
  }

  # Pool files (immutable, cache forever)
  location ~ ^/pool/ {
    root /var/www/aurora-repo;
    expires 365d;
    add_header Cache-Control "public, max-age=31536000, immutable";
  }
}
```

---

### 9.2 Recommended Architecture: Tiered Approach

**Tier 1 (MVP):** GitHub Pages
- Free, zero maintenance
- Suitable for launch (~1000 users)
- Automatic CI/CD integration

**Tier 2 (Scaling):** GitHub Pages → Cloudflare R2
- Add Cloudflare R2 as primary storage (still free egress via Workers)
- GitHub Pages still secondary mirror
- ~10k users

**Tier 3 (Production):** AWS S3 + CloudFront
- S3 for versioning/backup
- CloudFront for global distribution
- Custom domain (archive.aurora.linux)
- ~100k+ users

**Tier 4 (Enterprise):** Multi-region active-active
- AWS S3 + CloudFront (primary)
- Cloudflare R2 (secondary mirror)
- Akamai CDN (optional, premium)
- Custom mirror network
- Unlimited scale

### 9.3 DNS & Domain Setup

```
aurora.linux (main domain)
├── archive.aurora.linux (APT repository)
│   └── CNAME cloudfront.d111111abcdef8.cloudfront.net
│       or: CNAME aurora-repo.nyc3.cdn.digitaloceanspaces.com
│       or: A 1.2.3.4 (self-hosted VPS)
├── www.aurora.linux (website)
└── docs.aurora.linux (documentation)
```

**DNS records (AWS Route53):**
```
archive.aurora.linux  CNAME  d111111abcdef8.cloudfront.net  (AWS CloudFront)
```

**TLS Certificate:**
- AWS CloudFront: Use AWS Certificate Manager (free)
- Cloudflare: Use Cloudflare TLS (free)
- Self-hosted: Use Let's Encrypt + Certbot (free)

---

## Part 10: User Installation Experience

### 10.1 Installation Workflow

The ideal installation flow:

```
1. User discovers Aurora
   ↓
2. Visit https://aurora.linux/install
   ↓
3. Copy-paste installation command
   ↓
4. Script imports key + adds repo + runs apt update + installs
   ↓
5. Aurora ready to use
   ↓
6. Future updates: sudo apt update && sudo apt upgrade (automatic)
```

### 10.2 One-Line Installation Script

Create `/install` endpoint with installer script:

```bash
#!/bin/bash
# Aurora Linux Design System — One-line Installer
# https://aurora.linux/install
# 
# Usage: curl https://get.aurora.linux | bash
#
# This script:
#   1. Downloads Aurora's signing key
#   2. Adds Aurora repository to APT
#   3. Runs apt update
#   4. Installs aurora meta-package

set -e

echo "🎨 Aurora Linux Design System Installer"
echo "========================================="
echo

# Check prerequisites
if ! command -v apt &> /dev/null; then
    echo "❌ Error: APT not found. Aurora requires Debian/Ubuntu."
    exit 1
fi

if [ "$EUID" -ne 0 ]; then
    echo "❌ Error: This script must be run with sudo"
    echo
    echo "Please run:"
    echo "  curl https://get.aurora.linux | sudo bash"
    exit 1
fi

echo "📥 Downloading Aurora repository key..."
mkdir -p /usr/share/keyrings
curl -s https://archive.aurora.linux/aurora-archive-keyring.gpg | \
    tee /usr/share/keyrings/aurora-archive-keyring.gpg > /dev/null

echo "✅ Key imported"
echo

echo "🔧 Adding Aurora repository..."
cat > /etc/apt/sources.list.d/aurora.sources <<EOF
Types: deb
URIs: https://archive.aurora.linux
Suites: stable
Components: main
Signed-By: /usr/share/keyrings/aurora-archive-keyring.gpg
EOF

echo "✅ Repository added"
echo

echo "🔄 Updating package lists..."
apt-get update -qq

echo "✅ Package lists updated"
echo

echo "📦 Installing Aurora..."
apt-get install -y aurora

echo "✅ Aurora installed successfully!"
echo
echo "🎉 Aurora is ready to use!"
echo
echo "Next steps:"
echo "  • Open your system settings to change themes/colors"
echo "  • Visit https://aurora.linux/docs for configuration guides"
echo "  • Report bugs at https://github.com/aurora-linux/aurora/issues"
```

**Serve via:**
```
https://get.aurora.linux/  (CNAME to github.io or R2)
https://aurora.linux/install  (direct link on website)
```

### 10.3 Installation Methods

#### Method 1: One-Line (Recommended)
```bash
curl https://get.aurora.linux | sudo bash
```

#### Method 2: Manual (Transparent)
```bash
# 1. Import key
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 1234567890ABCDEF
# or
wget https://archive.aurora.linux/aurora-archive-keyring.gpg
sudo apt-key add aurora-archive-keyring.gpg

# 2. Add repository
echo "deb https://archive.aurora.linux/dists/stable main" | \
  sudo tee /etc/apt/sources.list.d/aurora.sources

# 3. Update + Install
sudo apt update
sudo apt install aurora
```

#### Method 3: DEB Package (For distributions)
```bash
# Create aurora-repository package that installs keyring + sources
# Users download aurora-repository_1.0_all.deb
sudo dpkg -i aurora-repository_1.0_all.deb
sudo apt update
sudo apt install aurora
```

### 10.4 Post-Installation UX

After installation:

```bash
# Configuration wizard (optional)
sudo aurora-configure
# → Select color scheme (light/dark/auto)
# → Apply to GNOME/KDE/other DEs
# → Set wallpaper
# → Install fonts system-wide

# Version check
aurora --version
# Aurora Design System v1.0.0

# Help
aurora --help
# Aurora configuration tool
#   aurora-configure    Interactive setup
#   aurora list-themes  List available themes
#   aurora apply-theme THEME  Apply theme immediately
#   aurora --version    Show version
```

### 10.5 Upgrade Experience

**Automatic (APT):**
```bash
sudo apt update
sudo apt upgrade
# aurora and all components updated automatically
```

**Manual upgrade:**
```bash
sudo apt install --only-upgrade aurora
# Updates aurora + its Depends
```

**Selective upgrade:**
```bash
# Only upgrade themes, not wallpapers
sudo apt install --only-upgrade aurora-themes

# Update to testing channel
sudo apt list --upgradable
# → Shows testing versions if subscribed to testing repo
```

### 10.6 Uninstall Experience

**Full removal:**
```bash
sudo apt remove aurora
# Removes aurora meta-package and all dependencies

sudo apt remove --auto-remove aurora
# Also removes packages only needed by aurora
```

**Purge (remove + configs):**
```bash
sudo apt purge aurora
# Removes package files + configuration
```

**Remove repository:**
```bash
sudo rm /etc/apt/sources.list.d/aurora.sources
sudo rm /usr/share/keyrings/aurora-archive-keyring.gpg
sudo apt update
```

### 10.7 Troubleshooting Workflow

**"Unable to locate package aurora"**
```bash
# Check repository is added correctly
grep -r "archive.aurora.linux" /etc/apt/

# Check key is trusted
apt-key list | grep -i aurora

# Manually run update
sudo apt update

# Verify signature
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 1234567890ABCDEF
```

**"Break of aurora by other packages"**
```bash
# Check conflicts
apt-cache policy aurora

# Simulate upgrade
sudo apt install -s aurora
# Outputs what would be done without actually doing it
```

**"W: Target Packages (main/binary-all/Packages) is configured multiple times"**
```bash
# Remove duplicate entries
ls /etc/apt/sources.list.d/
# Remove .list files if also in .sources files
```

---

## Part 11: CI/CD Pipeline

### 11.1 GitHub Actions Workflow

**File:** `.github/workflows/release.yml`

```yaml
name: Release Aurora Packages

on:
  push:
    tags:
      - 'v*'

jobs:
  build-packages:
    name: Build & Sign Packages
    runs-on: ubuntu-latest
    permissions:
      contents: write
      packages: write
    
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Get version from tag
        id: version
        run: |
          VERSION=${GITHUB_REF#refs/tags/v}
          echo "version=$VERSION" >> $GITHUB_OUTPUT
          echo "Building Aurora $VERSION"

      - name: Setup build environment
        run: |
          sudo apt-get update
          sudo apt-get install -y debhelper dh-make lintian dpkg-dev fakeroot

      - name: Extract Aurora assets
        run: |
          ./scripts/prepare-packages.sh "${{ steps.version.outputs.version }}"
          # Generates debian/ directories for all packages

      - name: Validate assets
        run: |
          ./scripts/validate-assets.sh
          # Checks CSS syntax, icon sizes, font formats, etc.

      - name: Build packages
        run: |
          ./scripts/build-all-packages.sh "${{ steps.version.outputs.version }}"
          # Creates .deb files via dpkg-buildpackage

      - name: Lint packages
        run: |
          for deb in *.deb; do
            echo "Linting $deb..."
            lintian -EviI "$deb" || true
          done

      - name: Import GPG key
        run: |
          echo "${{ secrets.GPG_PRIVATE_KEY }}" | gpg --import
          gpg --trust-model always --batch --yes --import-ownertrust \
            <(echo "${{ secrets.GPG_KEY_FINGERPRINT }}:6:")

      - name: Sign packages
        run: |
          for deb in *.deb; do
            dpkg-sig --sign builder \
              -k "${{ secrets.GPG_KEY_ID }}" \
              "$deb"
          done

      - name: Generate repository metadata
        run: |
          mkdir -p dists/stable/main/binary-all
          mkdir -p pool/main
          
          # Use aptly to generate Packages file
          aptly repo create -architectures="all" aurora-stable
          aptly repo add aurora-stable *.deb
          aptly snapshot create aurora-stable-${{ steps.version.outputs.version }} \
            from repo aurora-stable
          aptly publish snapshot \
            aurora-stable-${{ steps.version.outputs.version }} \
            -skip-contents -force-overwrite

      - name: Sign Release file
        run: |
          cd dists/stable
          gpg --default-key "${{ secrets.GPG_KEY_ID }}" \
            --detach-sign --armor \
            -o Release.gpg Release
          gpg --default-key "${{ secrets.GPG_KEY_ID }}" \
            --clearsign --armor \
            --output InRelease Release

      - name: Upload to repository (GitHub Releases)
        uses: softprops/action-gh-release@v1
        with:
          files: |
            *.deb
            dists/stable/Release
            dists/stable/Release.gpg
            dists/stable/InRelease
          draft: false
          prerelease: false
          generate_release_notes: true

      - name: Upload to repository (R2/S3)
        env:
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          AWS_S3_BUCKET: ${{ secrets.AWS_S3_BUCKET }}
        run: |
          # Upload via AWS CLI to S3/R2
          aws s3 sync pool/ "s3://${AWS_S3_BUCKET}/pool/"
          aws s3 sync dists/ "s3://${AWS_S3_BUCKET}/dists/"
          
          # Invalidate CloudFront cache
          aws cloudfront create-invalidation \
            --distribution-id "${{ secrets.CF_DISTRO_ID }}" \
            --paths "/*"

      - name: Create release notes
        run: |
          cat > RELEASE_NOTES.md <<EOF
          # Aurora ${{ steps.version.outputs.version }}

          ## Installation

          \`\`\`bash
          sudo apt install aurora
          \`\`\`

          ## Packages

          EOF
          
          for deb in *.deb; do
            dpkg -I "$deb" | grep -A 50 "Description:" >> RELEASE_NOTES.md
          done

      - name: Publish release notes
        uses: softprops/action-gh-release@v1
        with:
          body_path: RELEASE_NOTES.md
```

### 11.2 Automatic Versioning

**Strategy:** Semantic versioning from git tags

```bash
# Tag commits to trigger release
git tag -a v1.0.0 -m "Aurora 1.0.0 release"
git push origin v1.0.0
# Automatically triggers GitHub Actions workflow

# Tag format:
# v MAJOR.MINOR.PATCH[-prerelease][+build]
# v1.0.0
# v1.0.0-beta.1
# v1.0.0-rc.1
# v1.1.0
```

**Version bump script:**

```bash
#!/bin/bash
# .github/scripts/bump-version.sh

CURRENT_VERSION=$(grep "^version" Cargo.toml | head -1 | sed 's/.*= "//' | sed 's/".*//')
BUMP_TYPE=${1:-patch}  # major, minor, or patch

case "$BUMP_TYPE" in
  major)
    NEW_VERSION=$(echo "$CURRENT_VERSION" | awk -F. '{print ($1+1) ".0.0"}')
    ;;
  minor)
    NEW_VERSION=$(echo "$CURRENT_VERSION" | awk -F. '{print $1 "." ($2+1) ".0"}')
    ;;
  patch)
    NEW_VERSION=$(echo "$CURRENT_VERSION" | awk -F. '{print $1 "." $2 "." ($3+1)}')
    ;;
esac

# Update version in files
sed -i "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
# Update debian/changelog manually

# Commit + tag
git add Cargo.toml debian/changelog
git commit -m "Bump version to $NEW_VERSION"
git tag -a "v$NEW_VERSION" -m "Release $NEW_VERSION"
git push origin main "v$NEW_VERSION"

echo "Released Aurora $NEW_VERSION"
```

### 11.3 Continuous Testing

**Before release, GitHub Actions runs:**

```yaml
  test-packages:
    name: Test Packages
    runs-on: ubuntu-latest
    needs: build-packages
    
    steps:
      - name: Download artifacts
        uses: actions/download-artifact@v3

      - name: Test installation
        run: |
          sudo dpkg -i aurora-themes_*.deb
          sudo dpkg -i aurora-icons_*.deb
          # etc.
          
          # Verify files installed
          [ -d /usr/share/themes/Aurora ] || exit 1
          [ -d /usr/share/icons/Aurora ] || exit 1

      - name: Test post-install scripts
        run: |
          # Verify icon cache updated
          [ -f /var/cache/fontconfig/* ] || exit 1
          
          # Verify fonts registered
          fc-list | grep -i aurora || exit 1

      - name: Test removal
        run: |
          sudo apt remove aurora-themes -y
          [ ! -d /usr/share/themes/Aurora ]
```

---

## Part 12: Release Workflow

### 12.1 Release Channels

Aurora provides three release channels for different stability requirements:

| Channel | Version | Cadence | Testing | Use Case |
|---------|---------|---------|---------|----------|
| **Stable** | `1.0.0`, `1.0.1`, `1.1.0` | Every 6–8 weeks | Extensive (4 weeks beta) | Production desktops |
| **Testing** | `1.1.0-beta.1`, `1.1.0-rc.1` | Every 2–3 weeks | Moderate (community feedback) | Early adopters, developers |
| **Unstable** | `1.2.0-dev+git.abc123` | Nightly | Minimal (automated tests) | Bleeding edge, CI testing |

### 12.2 Branching Strategy

```
main (stable releases)
  ├── tag: v1.0.0  ─→ dists/stable
  └── tag: v1.0.1  ─→ dists/stable
      
develop (testing releases)
  ├── tag: v1.1.0-beta.1  ─→ dists/testing
  ├── tag: v1.1.0-rc.1    ─→ dists/testing
  └── [base for v1.1.0 final]

nightly (automatic from develop)
  └── Pushed daily to dists/unstable
      Version: 1.2.0-dev+git.$(date +%Y%m%d).$(git rev-parse --short HEAD)
```

### 12.3 Release Timeline

**6-week release cycle:**

```
Week 1: Feature freeze for next release
  └─ v1.1.0-dev branch created

Week 2–3: Testing, bug fixes
  └─ Community testing, feedback collection

Week 4: Beta release
  └─ v1.1.0-beta.1 released to testing channel
  └─ Wider testing with marketing push

Week 5: Release Candidate
  └─ v1.1.0-rc.1 released to testing channel
  └─ Final 1 week testing

Week 6: Final release
  └─ v1.1.0 released to stable channel
  └─ Release notes published
  └─ Blog post + social media announcement

Week 7: Develop starts next cycle
```

### 12.4 Repository Promotion

```
Build & Test
    ↓
Push to dists/unstable (nightly)
    ↓
Promote to dists/testing (beta)
    ↓
Promote to dists/stable (release)
```

**Promotion mechanism with aptly:**

```bash
# Nightly: Always publish latest develop
aptly publish snapshot -force-overwrite \
  aurora-unstable-$(date +%Y%m%d) \
  filesystem:filesystem

# Beta → Testing
aptly snapshot create aurora-testing-1.1.0-beta.1 \
  from repo aurora-testing
aptly publish snapshot aurora-testing-1.1.0-beta.1 \
  filesystem:filesystem

# RC → Testing (update)
aptly publish snapshot -replace \
  aurora-testing-1.1.0-rc.1 \
  filesystem:filesystem

# Final → Stable
aptly snapshot create aurora-stable-1.1.0 \
  from repo aurora-stable
aptly publish snapshot aurora-stable-1.1.0 \
  filesystem:filesystem
```

### 12.5 User Upgrade Path

**Stable → Stable (e.g., 1.0.0 → 1.0.1):**
```bash
sudo apt update && sudo apt upgrade
# Automatic patch update
```

**Stable → Testing (opt-in):**
```bash
# Add testing repository
echo "deb https://archive.aurora.linux/dists/testing main" | \
  sudo tee /etc/apt/sources.list.d/aurora-testing.sources

sudo apt update
sudo apt install -t testing aurora
# Installed from testing repository
```

**Switch back to Stable:**
```bash
# Remove testing repository
sudo rm /etc/apt/sources.list.d/aurora-testing.sources
sudo apt update
# Downgrades to latest stable version
```

---

## Part 13: Upgrade Strategy

### 13.1 Upgrade Scenarios

#### Patch Release (1.0.0 → 1.0.1)
- Bug fixes, minor asset improvements
- No breaking changes
- Automatic upgrade recommended
- Icon/font cache refresh (post-install script)

#### Minor Release (1.0.0 → 1.1.0)
- New components, new features
- Extended asset sets
- New package dependencies (Recommends)
- Backward compatible

#### Major Release (1.0.0 → 2.0.0)
- Visual redesign, new design language
- Potential breaking changes
- Theme migration needed
- Old version may be replaced

### 13.2 Asset Migration

**When Aurora redesigns themes (major bump):**

```debian/postrm (for old version)
#!/bin/bash

if [ "$1" = "upgrade" ]; then
  # Migrating to Aurora 2.0
  
  # Backup user customizations
  if grep -q "Aurora" ~/.config/gtk-3.0/settings.ini; then
    cp ~/.config/gtk-3.0/settings.ini \
       ~/.config/gtk-3.0/settings.ini.aurora-1.0.backup
    echo "⚠️  Backed up your Aurora 1.0 settings to settings.ini.aurora-1.0.backup"
  fi
fi
```

**In new package postinst:**
```bash
#!/bin/bash

if [ "$1" = "configure" ]; then
  # Detect previous Aurora version
  if dpkg -l | grep -q aurora-themes:all.*1\.0; then
    # Offer migration
    echo "🎨 Aurora 2.0 detected upgrade from 1.0"
    echo "   Your previous settings have been backed up."
    echo "   To restore old theme: GTK_THEME=Aurora-1.0 (if installed)"
  fi
fi
```

### 13.3 Configuration Preservation

Aurora assets are generally read-only, but user settings should be preserved:

```bash
# User customizations in ~/.config/
~/.config/gtk-3.0/settings.ini
~/.config/gtk-4/settings.ini
~/.config/kdedefaults
~/.local/share/fonts (user-installed fonts)

# Post-install should NOT overwrite these
```

**debian/conffiles (if user-editable config):**
```
/etc/aurora/theme.conf
```

**Post-install should:**
```bash
# Check for existing user config
if [ -f "$HOME/.config/gtk-3.0/settings.ini" ]; then
  # Preserve user's GTK settings
  # Only update if Aurora-specific keys missing
  fi
```

### 13.4 Deprecated Assets

**If theme is deprecated in 2.0:**

```debian/control (old package)
Package: aurora-themes
Version: 1.0.0
Replaces: aurora-themes (<< 2.0)
Breaks: aurora (< 2.0)
Status: not-installed not-installed

# Package can be marked "deinstall" but won't auto-remove
```

**New package replaces old:**
```debian/control (new package)
Package: aurora-themes-2
Replaces: aurora-themes (<< 2.0)
Breaks: aurora-themes (<< 2.0)
Conflicts: aurora-themes (< 2.0)
```

### 13.5 Rollback Strategy

**If new release has critical bugs:**

Users can downgrade to previous version:

```bash
# List available versions
apt-cache policy aurora-themes

# Install specific version
sudo apt install aurora-themes=1.0.0
# Pins to 1.0.0 version

# Later, allow auto-upgrade
sudo apt install aurora-themes
# Removes pin, updates to latest
```

**Or, repository keeps multiple suites:**
```bash
# Switch to stable-archive (frozen version)
echo "deb https://archive.aurora.linux/dists/stable-archive main" | \
  sudo tee /etc/apt/sources.list.d/aurora-archive.sources
  
sudo apt update
sudo apt install aurora-themes=1.0.0
```

**Automatic rollback (optional):**
```bash
# post-install error detection
if ! update-icon-caches /usr/share/icons/Aurora; then
  echo "Post-install failed, rolling back..."
  dpkg --remove --force-all aurora-themes
  apt-get install aurora-themes=1.0.0  # Previous version
  exit 1
fi
```

---

## Part 14: Security

### 14.1 Package Integrity

**SHA256 verification:**

Every package is signed with SHA256 hash:

```
Filename: pool/main/a/aurora-themes/aurora-themes_1.0.0_all.deb
Size: 1234567
MD5sum: 1234567890abcdef1234567890abcdef
SHA1: fedcba0987654321fedcba0987654321fedcba09
SHA256: abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
```

**APT verifies before installation:**
```bash
sudo apt update  # Downloads & verifies Packages.gz
sudo apt install aurora-themes  # APT checks SHA256 before extracting
```

### 14.2 Signed Repositories

**Release file signing:**

```bash
# Maintainer signs Release file with GPG
gpg --default-key aurora@example.com \
    --clearsign \
    -o dists/stable/InRelease \
    dists/stable/Release

# Users verify signature
gpg --verify dists/stable/InRelease
# Output: Good signature from "Aurora Linux Repository"
```

**APT trusts imported keys:**
```bash
# User imports Aurora's public key
sudo apt-key add aurora-archive-keyring.gpg

# APT automatically verifies Release file signature
# If signature is invalid, apt refuses to update
```

### 14.3 Reproducible Builds

**Goal:** Anyone building Aurora should get identical .deb files (byte-for-byte)

**Reproducibility techniques:**

```bash
# Set build date (use git commit date, not current time)
SOURCE_DATE_EPOCH=$(git log -1 --format=%ct)
export SOURCE_DATE_EPOCH

# dpkg-buildpackage respects SOURCE_DATE_EPOCH
dpkg-buildpackage -us -uc

# Verify reproducibility
mkdir /tmp/build1 /tmp/build2
# Build once in each directory
# Compare checksums
sha256sum *.deb

# Should be identical
```

**Debian packaging for reproducibility:**

```debian/rules
# Ensure deterministic file timestamps
export SOURCE_DATE_EPOCH := $(shell git log -1 --format=%ct)

override_dh_auto_build:
	find . -type f -exec touch -d @$(SOURCE_DATE_EPOCH) {} \;
	# Reset all file timestamps
```

**Benefits:**
- Users can independently verify binaries aren't backdoored
- Detect supply-chain compromises
- Transparency builds trust

### 14.4 Supply Chain Security

**Threats and mitigations:**

| Threat | Mitigation |
|--------|------------|
| Compromised GPG key | Use subkeys, offline master key, regular rotation |
| GitHub account compromise | Enable 2FA, restrict deploy keys, use OIDC |
| Dependency vulnerabilities | Audit assets (fonts, etc.), keep tools updated |
| Man-in-the-middle (MITM) | Use HTTPS, pin TLS certificates, GPG verify |
| Unsigned packages | Sign all .deb packages + Release file |

### 14.5 GitHub Actions Security

**Best practices:**

```yaml
# ✓ Use OIDC instead of personal access tokens
permissions:
  id-token: write

# ✓ Limit workflow permissions
permissions:
  contents: read
  packages: write

# ✓ Restrict who can approve deployments
environment: production
  deployment-branch-policy:
    protected-branches: true
    custom-deployment-branch-policies: false

# ✗ Never commit secrets
# ✗ Never use PAT from personal account
# ✗ Don't hardcode GPG passphrases
```

**Store secrets safely:**

```bash
# GPG private key (encrypted)
git-crypt add-gpg-user --trusted aurora@example.com
git-crypt encrypt .secrets/gpg-key.asc

# Or use GitHub Secrets + base64 encoding
echo "$GPG_PRIVATE_KEY" | base64 -w0 > /tmp/encoded
# Paste into GitHub Secrets settings
```

### 14.6 Dependency Auditing

**Aurora dependencies (minimal):**

```
aurora-themes:
  Depends: ${misc:Depends}  # Only debhelper variables

aurora-fonts:
  Depends: fontconfig
  # Verify fontconfig has no known CVEs
  # apt list --upgradable ubuntu-archive-keyring (check security)

aurora-gdm:
  Depends: gdm
  # GDM is standard Ubuntu package (vetted by Ubuntu security team)
```

**Audit dependencies:**

```bash
# Check for known CVEs
apt-cache depends aurora-fonts | grep Depends \
  | awk '{print $2}' | while read dep; do
    echo "Checking $dep..."
    # Query Ubuntu Security Advisories
    curl -s https://usn.ubuntu.com/ | grep "$dep" || echo "OK"
  done

# Use Dependabot (GitHub) for Git dependencies
# Or Trivy for container scanning
```

---

## Part 15: Developer Experience

### 15.1 Build Tools & Makefile

**Makefile for common tasks:**

```makefile
.PHONY: build validate lint test clean install

build:
	./scripts/prepare-packages.sh
	dpkg-buildpackage -us -uc

validate:
	./scripts/validate-assets.sh

lint:
	for deb in *.deb; do lintian -EviI $$deb; done

test:
	./scripts/test-installation.sh

clean:
	rm -f *.deb *.dsc *.changes
	rm -rf debian/*/

install: build
	sudo dpkg -i *.deb

release:
	./scripts/bump-version.sh patch
	git push origin main

.DEFAULT_GOAL := help

help:
	@echo "Aurora build targets:"
	@echo "  make build       - Build .deb packages"
	@echo "  make validate    - Validate assets (CSS, icons, fonts)"
	@echo "  make lint        - Run lintian checks"
	@echo "  make test        - Test package installation"
	@echo "  make install     - Build and install locally"
	@echo "  make clean       - Remove build artifacts"
	@echo "  make release     - Bump version and push"
```

**Usage:**
```bash
make build      # Quick build
make validate   # Check asset quality
make install    # Build + install locally
```

### 15.2 Docker Development Environment

**Dockerfile for consistent build environment:**

```dockerfile
FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    build-essential \
    debhelper \
    dh-make \
    dpkg-dev \
    fakeroot \
    lintian \
    git

WORKDIR /workspace
VOLUME ["/workspace"]

CMD ["/bin/bash"]
```

**Development container:**
```bash
# Build image
docker build -t aurora-builder .

# Run in container
docker run -it -v $(pwd):/workspace aurora-builder

# Inside container
cd /workspace
make build
make lint
```

### 15.3 Development Container (DevContainer)

**.devcontainer/devcontainer.json:**

```json
{
  "image": "mcr.microsoft.com/devcontainers/base:jammy",
  "features": {
    "ghcr.io/devcontainers/features/git:1": {},
    "ghcr.io/devcontainers/features/docker-in-docker:2": {}
  },
  "customizations": {
    "vscode": {
      "extensions": [
        "ms-vscode.makefile-tools",
        "charliermarsh.ruff",
        "golang.go"
      ]
    }
  },
  "postCreateCommand": "apt-get update && apt-get install -y debhelper dh-make dpkg-dev fakeroot lintian",
  "remoteUser": "vscode"
}
```

**VS Code opens project with full dev environment:**
```
Ctrl+Shift+P → "Dev Containers: Reopen in Container"
→ Full build tools available
```

### 15.4 Automated Testing

**Test script (./scripts/test-installation.sh):**

```bash
#!/bin/bash
set -e

echo "Testing Aurora package installation..."

# Create temporary test directory
TESTDIR=$(mktemp -d)
trap "rm -rf $TESTDIR" EXIT

# Extract packages
cd "$TESTDIR"
mkdir -p test-rootfs

for deb in /path/to/*.deb; do
    echo "Testing $(basename $deb)..."
    
    # Install to test rootfs
    dpkg -x "$deb" test-rootfs/
    
    # Verify key files exist
    case "$(basename $deb)" in
        aurora-themes*)
            [ -d test-rootfs/usr/share/themes/Aurora ] || exit 1
            ;;
        aurora-icons*)
            [ -d test-rootfs/usr/share/icons/Aurora ] || exit 1
            ;;
        aurora-fonts*)
            [ -d test-rootfs/usr/share/fonts/aurora ] || exit 1
            ;;
    esac
done

echo "✅ All tests passed!"
```

**Run tests before commit:**
```bash
./scripts/test-installation.sh && make lint
```

### 15.5 Contributing Guidelines

**CONTRIBUTING.md:**

```markdown
# Contributing to Aurora

## Development Setup

1. Clone repository
2. Install dependencies: `sudo apt-get install debhelper lintian`
3. Build locally: `make build`
4. Install for testing: `make install`

## Making Changes

1. Create feature branch: `git checkout -b feature/my-theme`
2. Make changes to themes/ or assets/
3. Validate: `make validate`
4. Build and test locally: `make build && make test`
5. Run linter: `make lint`
6. Commit with clear message: `git commit -m "Add XYZ feature"`
7. Open pull request

## PR Checklist

- [ ] Validated assets (CSS, icons, fonts)
- [ ] Lintian passes (`make lint`)
- [ ] Local installation test passed
- [ ] Updated CHANGELOG.md
- [ ] Updated documentation
- [ ] Tested on at least one desktop environment

## Package Naming

- Asset directories: `assets/themes/`, `assets/icons/`, etc.
- Debian files: Always in `debian/` directory
- Don't commit built .deb files

## Questions?

Join our Discord or open an issue on GitHub.
```

---

## Part 16: Documentation

### 16.1 Documentation Structure

```
docs/
├── INSTALLATION.md           # User installation guide
├── ARCHITECTURE.md           # Technical design (this file)
├── APT_DISTRIBUTION.md       # Repository management
├── RELEASE_PROCESS.md        # How releases work
├── CONTRIBUTING.md           # Developer guide
├── TROUBLESHOOTING.md        # Common issues
├── FAQ.md                    # Frequently asked questions
├── SECURITY.md               # Security policy
├── MAINTENANCE.md            # Long-term ops guide
├── examples/
│   ├── package-control.deb   # Example control file
│   ├── postinst.sh          # Example script
│   └── sources.list         # Example apt sources
└── API/
    ├── repository.md         # Repository API docs
    └── package-format.md     # Package format spec
```

### 16.2 Installation Documentation

**docs/INSTALLATION.md:**

```markdown
# Installing Aurora Linux Design System

## Quick Start

```bash
curl https://get.aurora.linux | sudo bash
sudo apt update
# Aurora is now installed!
```

## Manual Installation

### 1. Import Repository Key

```bash
wget https://archive.aurora.linux/aurora-archive-keyring.gpg
sudo apt-key add aurora-archive-keyring.gpg
```

### 2. Add Repository

```bash
echo "deb https://archive.aurora.linux/dists/stable main" | \
  sudo tee /etc/apt/sources.list.d/aurora.sources
```

### 3. Install

```bash
sudo apt update
sudo apt install aurora
```

## Upgrade Channels

### Stable (Default)
Production-ready, tested, recommended for most users.

```bash
# Already configured
sudo apt update
sudo apt upgrade
```

### Testing
Beta versions, new features, community feedback appreciated.

```bash
# Add testing repository
echo "deb https://archive.aurora.linux/dists/testing main" | \
  sudo tee /etc/apt/sources.list.d/aurora-testing.sources

sudo apt update
sudo apt install -t testing aurora=1.1.0-beta.1
```

### Unstable (Nightly)
Bleeding edge, updated daily, for developers and early adopters.

```bash
# Add unstable repository  
echo "deb https://archive.aurora.linux/dists/unstable main" | \
  sudo tee /etc/apt/sources.list.d/aurora-unstable.sources

sudo apt update
sudo apt install -t unstable aurora
```

## Selective Installation

Install only the components you need:

```bash
# Just themes
sudo apt install aurora-themes

# Themes + Icons
sudo apt install aurora-themes aurora-icons

# Full desktop integration
sudo apt install aurora aurora-kde-themes aurora-gdm aurora-sddm
```

## Configuration

### GNOME

1. Open Settings
2. Navigate to Appearance
3. Select Aurora theme (Light/Dark)

### KDE Plasma

1. Open System Settings
2. Go to Appearance → Global Theme
3. Select Aurora

### Other Desktops

Configuration varies. See examples for Xfce, Cinnamon, Mate, etc. at: https://aurora.linux/docs/configuration

## Troubleshooting

### "Package aurora not found"

Repository not added correctly:

```bash
grep archive.aurora.linux /etc/apt/sources.list.d/*
# Should show your repository line

# If not, add manually:
echo "deb https://archive.aurora.linux/dists/stable main" | \
  sudo tee /etc/apt/sources.list.d/aurora.sources

sudo apt update
```

### "GPG signature verification failed"

Key not imported:

```bash
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 1234567890ABCDEF
# or
wget https://archive.aurora.linux/aurora-archive-keyring.gpg && \
  sudo apt-key add aurora-archive-keyring.gpg

sudo apt update
```

### "Conflict with other theme packages"

Remove conflicting theme first:

```bash
sudo apt remove ubuntu-themes
sudo apt install aurora
```

## Uninstallation

```bash
# Remove packages
sudo apt remove aurora

# Remove repository
sudo rm /etc/apt/sources.list.d/aurora.sources
sudo apt-key del 1234567890ABCDEF
```

## Getting Help

- Docs: https://aurora.linux/docs
- Issues: https://github.com/aurora-linux/aurora/issues
- Discussions: https://github.com/aurora-linux/aurora/discussions
```

---

### 16.3 Release Process Documentation

**docs/RELEASE_PROCESS.md:**

```markdown
# Aurora Release Process

## Release Timeline

1. **Feature freeze** (Week 1)
   - Stabilize develop branch
   - Create release branch

2. **Beta testing** (Week 2–3)
   - Release beta.1 to testing repository
   - Gather feedback

3. **Release candidate** (Week 4–5)
   - Release rc.1 to testing repository
   - Final testing

4. **Final release** (Week 6)
   - v X.Y.Z released to stable repository
   - Release notes published
   - Social media announcement

## Performing a Release

### Preparation

```bash
# Update CHANGELOG.md with release notes
# Update version in Cargo.toml
git add CHANGELOG.md Cargo.toml
git commit -m "Prepare v1.1.0 release"

# Create annotated tag
git tag -a v1.1.0 -m "Aurora 1.1.0"

# Push tag (triggers GitHub Actions)
git push origin main
git push origin v1.1.0
```

### Verification

GitHub Actions automatically:
1. Builds all packages
2. Runs lintian
3. Signs packages
4. Uploads to repository
5. Publishes release notes

Check Actions tab to verify all steps pass.

### Post-Release

1. Announce on website
2. Post to social media
3. Update distribution mirrors
4. Monitor for issues
```

---

## Part 17: Long-Term Roadmap

### 17.1 Multi-Format Distribution

**Phase 1 (Current):** Debian/Ubuntu
**Phase 2 (Q3 2026):** Snap + Flatpak
**Phase 3 (Q4 2026):** Nix + Arch User Repository (AUR)
**Phase 4 (2027):** RPM + openSUSE
**Phase 5 (2027+):** Homebrew + OCI artifacts

#### Snap Package

```bash
# Install from snapcraft.io
sudo snap install aurora

# Automatic updates
# Automatic rollback on failure
# Confinement (sandboxing)
```

**advantages:**
- Universal Linux distribution
- Automatic updates
- Confinement for security
- Large reach (50+ Linux distributions)

**disadvantages:**
- Snapcraft infrastructure dependency
- Slower startup (confined environment)
- Larger disk footprint

#### Flatpak Package

```bash
# Install from Flathub
flatpak install flathub io.github.aurora.Linux

# Sandbox isolation
# Automatic updates
# Wide platform support
```

**Advantages:**
- Works across distributions
- Sandboxed (secure)
- Flatpak infrastructure mature

**Disadvantages:**
- Not all distros enable Flatpak by default
- Heavier than native packages

#### Nix Package

```bash
# declarative.nix
{
  environment.systemPackages = with pkgs; [
    aurora
    aurora-kde-themes
  ];
}

# nixos-rebuild switch
```

**Advantages:**
- Reproducible (content-addressable)
- Rollback support
- Multiple versions coexist

**Disadvantages:**
- Nix learning curve
- Smaller user base

#### AUR (Arch User Repository)

```bash
# Install from AUR
yay -S aurora

# Arch community maintains package
# Automatic updates via pacman
```

**Advantages:**
- Huge Arch user base
- Native compilation (optimized)
- Community-maintained

**Disadvantages:**
- Requires Arch Linux
- Community maintenance (not official)

#### RPM + Fedora/Red Hat

```bash
# Install on Red Hat/CentOS/Fedora
sudo dnf install aurora

# RPM format native to these distros
```

**Advantages:**
- Large market (enterprise Linux)
- Official binary distribution
- RedHat/CentOS/Fedora support

**Disadvantages:**
- RPM packaging different from .deb
- Additional CI/CD pipeline
- New signing infrastructure

### 17.2 Container Distribution

#### OCI Artifacts

```bash
# Push Aurora as OCI artifact
podman push aurora:1.0.0 registry.example.com/aurora:1.0.0

# Pull as container
podman pull registry.example.com/aurora:1.0.0

# Use in container builds
FROM ubuntu:24.04
RUN apt-get install aurora-themes
```

**Advantages:**
- Versioning via tags
- Registry infrastructure (Docker Hub, Quay.io)
- CI/CD integration

**Disadvantages:**
- Container overhead
- Not traditional package management

### 17.3 GUI Installation

**"Aurora Installer" application (future):**

```bash
# Download installer
wget https://aurora.linux/aurora-installer-1.0.0.deb
sudo dpkg -i aurora-installer-1.0.0.deb

# Launch GUI
aurora-installer

# Select options:
# [ ] GTK Themes
# [ ] Qt Themes
# [ ] KDE Plasma
# [ ] Fonts
# [ ] Icons
# [Install]
# → Handles repository setup automatically
```

**Advantages:**
- User-friendly (no command line)
- Configuration wizard
- Post-install setup

**Disadvantages:**
- Requires GTK/Qt application
- Additional maintenance

### 17.4 Automatic Update Notifications

**Telemetry-free update checker:**

```bash
# aurora-check-updates (runs weekly)
# Checks for new versions silently
# Notifies user via system notifications

# No analytics, no tracking
# Respects user privacy
# Optional (can disable in settings)
```

### 17.5 Theme Marketplace (Long-term)

**Aurora community-submitted themes:**

```
https://themes.aurora.linux/

- Upload custom themes
- Download community themes
- Version management
- Ratings/reviews

# Integration with aurora CLI
aurora theme install community://nature-inspired-theme
```

### 17.6 Cross-Distribution Package Generation

**Automated builds for all formats:**

```yaml
# GitHub Actions: One release, multiple outputs

build-matrix:
  package-type:
    - deb      # Debian/Ubuntu
    - rpm      # Fedora/Red Hat
    - snap     # All Linux
    - flatpak  # All Linux
    - nix      # NixOS
    - aur      # Arch Linux

# Each triggered on git tag
# All built simultaneously
# Published to respective repositories
```

---

## Deliverables Checklist

### System Architecture

- [x] Package hierarchy diagram (Part 1)
- [x] Dependency graph (Part 4)
- [x] Repository folder structure (Part 7)
- [x] Hosting architecture diagram (Part 9)
- [x] Upgrade lifecycle diagram (Part 13)
- [x] Release workflow diagram (Part 12)
- [x] Security architecture (Part 14)

### Package Templates

- [x] debian/control template (Part 2.2)
- [x] debian/copyright template (Part 2.3)
- [x] debian/postinst script (Part 2.4)
- [x] debian/postrm script (Part 2.5)
- [x] debian/preinst script (Part 2.6)
- [x] debian/rules file (Part 5.4)

### CI/CD & Automation

- [x] GitHub Actions workflow (Part 11)
- [x] Build pipeline script (Part 5.8)
- [x] Version bump script (Part 11.2)
- [x] Makefile (Part 15.1)
- [x] Test script (Part 15.4)

### Documentation

- [x] Installation guide (Part 16.2)
- [x] Release process (Part 16.3)
- [x] Contributing guidelines (Part 15.5)
- [x] Troubleshooting guide (Part 16.2)
- [x] Security policy (Part 14)
- [x] Architecture documentation (this document)

### Repository Management

- [x] aptly configuration (Part 6.3)
- [x] Repository structure (Part 7)
- [x] GPG key strategy (Part 8)
- [x] Signing workflow (Part 8.3–8.8)
- [x] Release channels (Part 12.1)

### Hosting

- [x] Hosting comparison (Part 9.1)
- [x] Recommended architecture (Part 9.2)
- [x] DNS setup (Part 9.3)
- [x] Installation scripts (Part 10.2)

---

## Production Readiness Checklist

Before production release:

### Infrastructure
- [ ] Domain acquired (aurora.linux)
- [ ] TLS certificate (Let's Encrypt or AWS ACM)
- [ ] Hosting platform selected (GitHub Pages → Cloudflare R2 → AWS S3)
- [ ] Repository server running (aptly)
- [ ] Backups configured
- [ ] Monitoring/alerting configured

### Security
- [ ] GPG key generated + backed up (offline)
- [ ] Repository key signing automated
- [ ] Package signing automated
- [ ] GitHub Actions secrets configured
- [ ] Deployment keys rotated
- [ ] Audit trail in place

### Documentation
- [ ] Installation guide published
- [ ] Release process documented
- [ ] Contributing guide published
- [ ] FAQ populated
- [ ] Troubleshooting guide written

### Testing
- [ ] Package installation tested on Ubuntu 20.04–24.04
- [ ] Package installation tested on Debian 11–12
- [ ] Different desktop environments tested (GNOME, KDE, Xfce, etc.)
- [ ] Lintian passes all checks
- [ ] Reproducible builds verified
- [ ] CI/CD pipeline tested end-to-end

### Release Automation
- [ ] GitHub Actions workflows configured
- [ ] Automatic versioning working
- [ ] Package signing automated
- [ ] Repository publication automated
- [ ] Release notes generation automated

### Support
- [ ] Issue template created
- [ ] Support contacts documented
- [ ] Response SLA defined
- [ ] Security reporting channel (security@aurora.linux)

---

## Common Pitfalls

### 1. Weak Dependency Management
**Problem:** Over-specifying dependencies; packages don't install
**Solution:** Test with minimal Depends, use Recommends for optional features

### 2. Hardcoded Paths
**Problem:** Paths like `/opt/aurora/` instead of `/usr/share/`
**Solution:** Always use `/usr/share/`, follow Debian filesystem hierarchy

### 3. Executable Files in /usr/share/
**Problem:** CSS, icons, fonts shouldn't have execute bit
**Solution:** Set permissions: `find . -type f -exec chmod 644 {} \;`

### 4. Key Rotation Failure
**Problem:** Old key expires; repository becomes untrusted
**Solution:** Plan rotation 6–12 months in advance; communicate to users

### 5. Repository Inconsistency
**Problem:** Packages in pool/ but not in Packages file
**Solution:** Use aptly or reprepro; don't manually add files

### 6. Missing Post-Install Scripts
**Problem:** Icon cache not updated; fonts don't appear
**Solution:** Always include postinst for: fonts, icons, themes

### 7. Broken Upgrade Path
**Problem:** Users upgrade; old settings conflict with new package
**Solution:** Test major-version upgrades; document migration steps

### 8. Insufficient Testing
**Problem:** Package works on maintainer's system; breaks for users
**Solution:** Test on multiple Ubuntu/Debian versions in CI

---

## Next Steps

1. **Immediate:** Set up GitHub repository structure + Makefile
2. **Week 1:** Build proof-of-concept .deb packages
3. **Week 2:** Set up GitHub Pages repository mirror
4. **Week 3:** Document installation workflow
5. **Week 4:** Beta release to testing channel
6. **Week 6:** Stable 1.0.0 release

---

**End of Document**

This architecture is production-grade and follows Debian Policy Manual strictly. It's designed to scale from hundreds to hundreds of thousands of users while maintaining security, reliability, and ease of use.
