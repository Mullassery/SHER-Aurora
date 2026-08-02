#!/bin/bash
# Aurora Repository Publishing Script
# Adds packages to repository, creates snapshots, and publishes

set -e

VERSION="${1:-1.0.0}"
CHANNEL="${2:-testing}"
REPO_ROOT="${3:-.}"

echo "📦 Aurora Repository Publishing Script"
echo "======================================"
echo "Version: $VERSION"
echo "Channel: $CHANNEL"
echo "Repository: $REPO_ROOT/repository"
echo

# Verify aptly is available
if ! command -v aptly &> /dev/null; then
  echo "❌ aptly not found. Install with: sudo apt-get install aptly"
  exit 1
fi

# Define repository names
REPO_MAP_testing="aurora-testing"
REPO_MAP_stable="aurora-stable"
REPO_MAP_unstable="aurora-unstable"

REPO_NAME="${REPO_MAP_${CHANNEL}}"

if [ -z "$REPO_NAME" ]; then
  echo "❌ Invalid channel: $CHANNEL (use: testing, stable, unstable)"
  exit 1
fi

echo "ℹ️  Publishing to repository: $REPO_NAME"
echo

# Find all .deb files to add
DEB_FILES=$(find . -maxdepth 1 -name "aurora-*_${VERSION}_all.deb" -type f 2>/dev/null)

if [ -z "$DEB_FILES" ]; then
  echo "❌ No .deb files found for version $VERSION"
  echo ""
  echo "Expected: aurora-*_${VERSION}_all.deb"
  echo ""
  echo "Build packages first:"
  echo "  cd ~/aurora && make build"
  exit 1
fi

echo "📥 Adding packages to $REPO_NAME..."
echo

# Add each package
while IFS= read -r deb_file; do
  pkg_name=$(basename "$deb_file" | sed "s/_${VERSION}_all.deb//")
  echo "  Adding: $pkg_name..."
  aptly repo add "$REPO_NAME" "$deb_file" || echo "    (May already exist)"
done <<< "$DEB_FILES"

echo
echo "✅ Packages added"
echo

# Create snapshot
SNAPSHOT_NAME="aurora-${CHANNEL}-v${VERSION}"
echo "📸 Creating snapshot: $SNAPSHOT_NAME"

# Remove old snapshot if exists
if aptly snapshot list -raw | grep -q "^${SNAPSHOT_NAME}$"; then
  echo "  (Snapshot exists, using existing)"
else
  aptly snapshot create "$SNAPSHOT_NAME" from repo "$REPO_NAME"
  echo "  ✓ Snapshot created"
fi

echo

# Publish snapshot
echo "🚀 Publishing snapshot..."

# Remove old publish if exists for this channel
if aptly publish list -raw 2>/dev/null | grep -q "$(echo $CHANNEL | sed 's/testing/dists\/testing/; s/stable/dists\/stable/; s/unstable/dists\/unstable/')"; then
  echo "  Updating existing publication..."
  aptly publish update -force-overwrite "$CHANNEL" "filesystem:filesystem" || true
else
  echo "  Creating new publication..."
  aptly publish snapshot -skip-signing=false "$SNAPSHOT_NAME" "filesystem:filesystem" || true
fi

echo

echo "✅ Repository published!"
echo
echo "Repository location:"
echo "  $REPO_ROOT/repository/dists/$CHANNEL/"
echo
echo "Next steps:"
echo "  1. Generate and sign Release files (see below)"
echo "  2. Upload to hosting (GitHub Pages, S3, etc.)"
echo "  3. Users add repository:"
echo
echo "     wget https://archive.aurora.linux/aurora-archive-keyring.gpg"
echo "     sudo apt-key add aurora-archive-keyring.gpg"
echo
echo "     echo 'deb https://archive.aurora.linux/dists/$CHANNEL main' | \\"
echo "       sudo tee /etc/apt/sources.list.d/aurora.sources"
echo
echo "     sudo apt update"
echo "     sudo apt install aurora"
echo
echo "═══════════════════════════════════════════════════════════════"
echo
echo "Sign Release files (requires GPG key):"
echo
echo "  cd $REPO_ROOT/repository/dists/$CHANNEL"
echo "  gpg --clearsign -a -o InRelease Release"
echo "  gpg --detach-sign -a -o Release.gpg Release"
echo "  gpg --verify InRelease"
