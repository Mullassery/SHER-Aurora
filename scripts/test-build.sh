#!/bin/bash
# Aurora Build Testing Script
# Tests building and installing packages locally

set -e

VERSION="${1:-1.0.0}"
TEST_DIR="${2:-.}"

echo "🧪 Aurora Package Build Testing"
echo "==============================="
echo "Version: $VERSION"
echo

# Check prerequisites
echo "Checking build dependencies..."
MISSING=""

for cmd in dpkg-buildpackage debhelper lintian; do
  if ! command -v $cmd &> /dev/null; then
    MISSING="$MISSING $cmd"
  fi
done

if [ -n "$MISSING" ]; then
  echo "❌ Missing build tools:$MISSING"
  echo
  echo "Install with:"
  echo "  sudo apt-get install -y debhelper dh-make dpkg-dev fakeroot lintian"
  exit 1
fi

echo "✅ All build tools available"
echo

# Test building one package
TEST_PKG="aurora-themes"
echo "📦 Testing build of $TEST_PKG..."

cd "packages/$TEST_PKG"

# Clean first
dpkg-buildpackage -T clean 2>/dev/null || true

# Build
echo "  Building..."
if dpkg-buildpackage -us -uc -b > /tmp/build-output.log 2>&1; then
  echo "  ✅ Build successful"
else
  echo "  ❌ Build failed"
  tail -20 /tmp/build-output.log
  exit 1
fi

# Check package created
DEB_FILE="../${TEST_PKG}_${VERSION}_all.deb"
if [ -f "$DEB_FILE" ]; then
  SIZE=$(du -h "$DEB_FILE" | cut -f1)
  echo "  📦 Package: $DEB_FILE ($SIZE)"
else
  echo "  ❌ Package file not found"
  exit 1
fi

# Run lintian
echo "  Linting..."
if lintian -EI "$DEB_FILE" > /tmp/lintian-output.txt 2>&1; then
  echo "  ✅ Lintian passed"
else
  E_COUNT=$(grep "^E:" /tmp/lintian-output.txt | wc -l || true)
  if [ "$E_COUNT" -gt 0 ]; then
    echo "  ❌ Lintian errors found"
    grep "^E:" /tmp/lintian-output.txt | head -5
    exit 1
  fi
fi

# Check package contents
echo "  Verifying contents..."
if dpkg -c "$DEB_FILE" > /dev/null 2>&1; then
  FILE_COUNT=$(dpkg -c "$DEB_FILE" | tail -1 | awk '{print NF}' || echo "?")
  echo "  ✅ Package valid (contains files)"
else
  echo "  ❌ Invalid package format"
  exit 1
fi

cd - > /dev/null

echo
echo "✅ Build test successful!"
echo
echo "Next steps:"
echo "  1. Build all packages: cd ~/aurora && make build"
echo "  2. Verify all packages: cd ~/aurora && make lint"
echo "  3. Test installation: sudo dpkg -i aurora-themes_${VERSION}_all.deb"
