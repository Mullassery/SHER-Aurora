#!/bin/bash
# Update debian/control files with proper dependencies for each Aurora package

set -e

echo "🔧 Updating package dependencies..."
echo

# Function to update a package's dependencies
update_pkg_deps() {
  local pkg=$1
  local deps=$2
  local recommends=$3
  local suggests=$4

  local ctrl="packages/$pkg/debian/control"

  if [ ! -f "$ctrl" ]; then
    echo "⚠️  Missing: $ctrl"
    return
  fi

  echo "📝 Updating $pkg..."

  # Create new control file with updated dependencies
  {
    # Print everything up to Depends/Recommends/Suggests
    sed '/^Depends:/,/^[A-Z]/d' "$ctrl" | sed '$d'

    # Add new Depends line
    if [ -z "$deps" ]; then
      echo "Depends: \${misc:Depends}"
    else
      echo "Depends: \${misc:Depends}, $deps"
    fi

    # Add Recommends if specified
    if [ -n "$recommends" ]; then
      echo "Recommends: $recommends"
    fi

    # Add Suggests if specified
    if [ -n "$suggests" ]; then
      echo "Suggests: $suggests"
    fi

    echo ""

    # Add description back
    sed -n '/^Description:/,$p' "$ctrl"
  } > "$ctrl.tmp" && mv "$ctrl.tmp" "$ctrl"

  echo "  ✓ Updated"
}

# Update each package with its dependencies
update_pkg_deps "aurora" \
  "aurora-themes, aurora-icons, aurora-cursors, aurora-fonts, aurora-colors, aurora-wallpapers, aurora-branding" \
  "aurora-terminal-themes, aurora-vscode, aurora-jetbrains" \
  "aurora-kde-themes, aurora-kde-integration, aurora-gnome-integration, aurora-sddm, aurora-gdm, aurora-accessibility, aurora-plymouth"

update_pkg_deps "aurora-themes" \
  "" \
  "aurora-icons, aurora-cursors" \
  "aurora-fonts, aurora-accessibility"

update_pkg_deps "aurora-icons" \
  "" \
  "aurora-themes, aurora-cursors" \
  "aurora-accessibility"

update_pkg_deps "aurora-cursors" \
  "" \
  "aurora-icons" \
  ""

update_pkg_deps "aurora-fonts" \
  "fontconfig" \
  "aurora-themes" \
  ""

update_pkg_deps "aurora-wallpapers" \
  "" \
  "aurora-themes" \
  ""

update_pkg_deps "aurora-branding" \
  "" \
  "aurora-fonts" \
  ""

update_pkg_deps "aurora-terminal-themes" \
  "" \
  "" \
  ""

update_pkg_deps "aurora-vscode" \
  "" \
  "" \
  "aurora-fonts"

update_pkg_deps "aurora-jetbrains" \
  "" \
  "" \
  "aurora-fonts"

update_pkg_deps "aurora-kde-themes" \
  "" \
  "aurora-icons, aurora-cursors, aurora-fonts" \
  "aurora-sddm"

update_pkg_deps "aurora-sddm" \
  "sddm" \
  "aurora-kde-themes" \
  ""

update_pkg_deps "aurora-gdm" \
  "gdm3 | gdm" \
  "aurora-themes" \
  ""

update_pkg_deps "aurora-gnome-integration" \
  "gnome-shell (>= 40)" \
  "aurora-themes, aurora-icons, aurora-fonts, aurora-wallpapers" \
  ""

update_pkg_deps "aurora-kde-integration" \
  "plasma-framework (>= 5.80)" \
  "aurora-kde-themes, aurora-icons" \
  ""

update_pkg_deps "aurora-colors" \
  "" \
  "" \
  ""

update_pkg_deps "aurora-accessibility" \
  "" \
  "aurora-themes, aurora-fonts" \
  ""

update_pkg_deps "aurora-plymouth" \
  "plymouth" \
  "" \
  ""

update_pkg_deps "aurora-cursors" \
  "" \
  "aurora-icons" \
  ""

echo
echo "✅ Dependency updates complete!"
echo
echo "Verify with:"
echo "  grep -A 3 '^Depends:' packages/aurora-themes/debian/control"
