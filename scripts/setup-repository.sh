#!/bin/bash
# Aurora APT Repository Setup Script
# Initializes aptly configuration and creates repository structure

set -e

echo "🔧 Aurora APT Repository Setup"
echo "==============================="
echo

# Check if aptly is installed
if ! command -v aptly &> /dev/null; then
  echo "❌ aptly not found"
  echo "Install with: sudo apt-get install aptly"
  echo ""
  echo "Or on macOS: brew install aptly"
  exit 1
fi

# Create repository directory
REPO_ROOT="${1:-.}"
REPO_PATH="$REPO_ROOT/repository"

echo "📁 Creating repository structure..."
mkdir -p "$REPO_PATH"/{pool,dists,indices}

echo "✓ Repository directory: $REPO_PATH"
echo

# Create aptly configuration
echo "⚙️  Creating aptly configuration..."

APTLY_CONFIG="$HOME/.aptly.conf"

# Backup existing config if present
if [ -f "$APTLY_CONFIG" ]; then
  echo "  Backing up existing $APTLY_CONFIG"
  cp "$APTLY_CONFIG" "$APTLY_CONFIG.backup"
fi

# Create new aptly configuration
cat > "$APTLY_CONFIG" <<'EOF'
{
  "architectures": ["all"],
  "dependencyFollowSuggests": false,
  "dependencyFollowRecommends": false,
  "dependencyFollowAllVariants": false,
  "dependencyFollowSource": false,
  "gpgDisableSign": false,
  "gpgDisableVerify": false,
  "gpgPersonalKey": "aurora@example.com",
  "downloadSourcePackages": false,
  "skipContentsPublishing": false,
  "ppaDistributorID": "ubuntu",
  "ppaCodename": "",
  "downloadWithSource": false,
  "skipLegacyPool": true,
  "FileSystemPublishEndpoints": {
    "filesystem": {
      "rootDir": "REPO_PATH_PLACEHOLDER/repository",
      "linkMethod": "hardlink"
    }
  }
}
EOF

# Replace placeholder with actual path
sed -i "s|REPO_PATH_PLACEHOLDER|$REPO_ROOT|g" "$APTLY_CONFIG"

echo "✓ Configuration saved to: $APTLY_CONFIG"
echo

# Initialize repositories
echo "📦 Initializing APT repositories..."
echo

# Remove existing repositories if they exist
for repo in aurora-stable aurora-testing aurora-unstable; do
  if aptly repo list | grep -q "^  \* \[$repo\]"; then
    echo "  Removing existing repository: $repo"
    aptly repo drop -force "$repo" || true
  fi
done

# Create new repositories
echo "  Creating aurora-stable..."
aptly repo create -architectures="all" -comment="Aurora Stable Repository" aurora-stable

echo "  Creating aurora-testing..."
aptly repo create -architectures="all" -comment="Aurora Testing Repository" aurora-testing

echo "  Creating aurora-unstable..."
aptly repo create -architectures="all" -comment="Aurora Unstable Repository" aurora-unstable

echo
echo "✅ Repositories created:"
aptly repo list -raw | sed 's/^/   /'

echo
echo "📊 Repository setup complete!"
echo
echo "Next steps:"
echo "  1. Build packages: cd ~/aurora && make build"
echo "  2. Add to repository: aptly repo add aurora-testing *.deb"
echo "  3. Create snapshot: aptly snapshot create aurora-testing-v1.0.0 from repo aurora-testing"
echo "  4. Publish: aptly publish snapshot aurora-testing-v1.0.0 filesystem:filesystem"
echo
echo "Configuration file: ~/.aptly.conf"
echo "Repository root: $REPO_PATH"
