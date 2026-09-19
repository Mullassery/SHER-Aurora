#!/bin/bash
# Aurora APT Package Preparation Script
# Generates debian/ directories for all Aurora packages

set -e

VERSION="${1:-1.0.0}"
MAINTAINER="Aurora Team <aurora@example.com>"
DATE=$(date -R)

echo "🔧 Aurora Package Preparation Script"
echo "===================================="
echo "Version: $VERSION"
echo "Maintainer: $MAINTAINER"
echo

# Define all Aurora packages
PACKAGES=(
  "aurora"
  "aurora-themes"
  "aurora-icons"
  "aurora-cursors"
  "aurora-fonts"
  "aurora-wallpapers"
  "aurora-colors"
  "aurora-branding"
  "aurora-terminal-themes"
  "aurora-vscode"
  "aurora-jetbrains"
  "aurora-kde-themes"
  "aurora-sddm"
  "aurora-gdm"
  "aurora-gnome-integration"
  "aurora-kde-integration"
  "aurora-accessibility"
  "aurora-plymouth"
)

# Create debian/ structure for each package
for pkg in "${PACKAGES[@]}"; do
  PKG_DIR="packages/$pkg"
  DEBIAN_DIR="$PKG_DIR/debian"

  # Skip if debian directory already exists
  if [ -d "$DEBIAN_DIR" ]; then
    echo "✓ $pkg (debian/ exists)"
    continue
  fi

  echo "📦 Setting up $pkg..."
  mkdir -p "$DEBIAN_DIR"

  # Use template if exists, otherwise create minimal
  if [ -f "$PKG_DIR/debian-control.template" ]; then
    cp "$PKG_DIR/debian-control.template" "$DEBIAN_DIR/control"
  else
    # Create minimal control file
    cat > "$DEBIAN_DIR/control" <<CONTROL
Package: $pkg
Version: $VERSION
Architecture: all
Maintainer: $MAINTAINER
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: \${misc:Depends}

Description: Aurora Linux Design System — $pkg
 Part of the Aurora Linux Design System.
CONTROL
  fi

  # Create changelog
  cat > "$DEBIAN_DIR/changelog" <<CHANGELOG
$pkg ($VERSION) UNRELEASED; urgency=medium

  * Initial release of $pkg for Aurora $VERSION

 -- $MAINTAINER  $DATE
CHANGELOG

  # Create copyright file
  cat > "$DEBIAN_DIR/copyright" <<COPYRIGHT
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: Aurora
Upstream-Contact: Aurora Team <aurora@example.com>
Source: https://github.com/aurora-linux/aurora
Comment: Aurora Linux Design System

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
 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 SOFTWARE.
COPYRIGHT

  # Create postinst script (handles common operations)
  cat > "$DEBIAN_DIR/postinst" <<'POSTINST'
#!/bin/bash
set -e

case "$1" in
  configure)
    # Update GTK icon cache if applicable
    if command -v update-icon-caches &> /dev/null; then
      update-icon-caches /usr/share/icons 2>/dev/null || true
    fi

    # Update font cache if applicable
    if command -v fc-cache &> /dev/null; then
      fc-cache -fv /usr/share/fonts 2>/dev/null || true
    fi

    # Update glib schemas if applicable
    if command -v glib-compile-schemas &> /dev/null; then
      glib-compile-schemas /usr/share/glib-2.0/schemas 2>/dev/null || true
    fi

    echo "✅ Aurora package configured successfully"
    ;;

  abort-upgrade|abort-remove|abort-deconfigure)
    ;;

  *)
    echo "postinst called with unknown argument \`$1'" >&2
    exit 1
    ;;
esac

exit 0
POSTINST
  chmod +x "$DEBIAN_DIR/postinst"

  # Create postrm script
  cat > "$DEBIAN_DIR/postrm" <<'POSTRM'
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
    echo "postrm called with unknown argument \`$1'" >&2
    exit 1
    ;;
esac

exit 0
POSTRM
  chmod +x "$DEBIAN_DIR/postrm"

  # Create rules file
  cat > "$DEBIAN_DIR/rules" <<'RULES'
#!/usr/bin/make -f
export DH_VERBOSE = 1

%:
	dh $@

override_dh_auto_build:
	@echo "No build required for asset packages"

override_dh_auto_install:
	# Install placeholder
	mkdir -p debian/tmp
	echo "Placeholder" > debian/tmp/INSTALL

override_dh_strip:
	# No binaries to strip

override_dh_compress:
	dh_compress
RULES
  chmod +x "$DEBIAN_DIR/rules"

  # Create source/format file
  mkdir -p "$DEBIAN_DIR/source"
  echo "3.0 (native)" > "$DEBIAN_DIR/source/format"

  echo "  ✓ debian/ structure created"
done

echo
echo "✅ Package preparation complete!"
echo "   All 18 packages have debian/ directories ready"
echo
echo "Next steps:"
echo "  1. Review debian/control files (adjust dependencies)"
echo "  2. Add package content (assets, themes, icons, etc.)"
echo "  3. Test builds: cd packages/aurora-themes && dpkg-buildpackage -us -uc"
