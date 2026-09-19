#!/bin/bash
# Aurora Package Verification Script
# Validates all packages meet Debian standards

set -e

echo "🔍 Aurora Package Verification"
echo "=============================="
echo

if ! command -v lintian &> /dev/null; then
  echo "⚠️  lintian not installed"
  echo "Install with: sudo apt-get install lintian"
  exit 1
fi

ERRORS=0
WARNINGS=0

# Find all .deb files
DEB_FILES=$(find . -maxdepth 1 -name "aurora-*_*.deb" -type f 2>/dev/null)

if [ -z "$DEB_FILES" ]; then
  echo "No .deb files found to verify"
  echo
  echo "Build packages first:"
  echo "  cd ~/aurora && make build"
  exit 0
fi

echo "Checking $(echo "$DEB_FILES" | wc -l) packages..."
echo

while IFS= read -r deb_file; do
  pkg_name=$(basename "$deb_file")
  echo "📦 $pkg_name"

  # Run lintian with error reporting
  if lintian -EI "$deb_file" > /tmp/lintian-output.txt 2>&1; then
    echo "  ✅ Clean"
  else
    # Count errors/warnings
    E_COUNT=$(grep "^E:" /tmp/lintian-output.txt | wc -l || true)
    W_COUNT=$(grep "^W:" /tmp/lintian-output.txt | wc -l || true)

    if [ "$E_COUNT" -gt 0 ]; then
      echo "  ❌ $E_COUNT errors:"
      grep "^E:" /tmp/lintian-output.txt | head -3 | sed 's/^/     /'
      ERRORS=$((ERRORS + E_COUNT))
    fi

    if [ "$W_COUNT" -gt 0 ]; then
      echo "  ⚠️  $W_COUNT warnings (non-critical)"
      WARNINGS=$((WARNINGS + W_COUNT))
    fi
  fi
done <<< "$DEB_FILES"

echo
echo "═══════════════════════════════════════════"
echo
if [ "$ERRORS" -eq 0 ]; then
  echo "✅ All packages verified successfully!"
  echo "   Errors: 0"
  echo "   Warnings: $WARNINGS (non-critical)"
else
  echo "❌ $ERRORS critical error(s) found"
  echo "   Warnings: $WARNINGS"
  exit 1
fi

echo
echo "Packages ready for repository"
