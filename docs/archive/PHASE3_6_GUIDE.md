# Phase 3-6 Complete Implementation Guide

**Weeks 4-6: Testing → Hosting → Signing → Release**

---

## Overview

This guide walks through the final three phases of Aurora APT repository deployment:

- **Phase 3** (Week 4): Testing infrastructure & build verification
- **Phase 4** (Weeks 4-5): Hosting setup & repository deployment
- **Phase 5** (Week 5): GPG key management & signing
- **Phase 6** (Week 6): Production release & launch

---

## Phase 3: Testing (Week 4)

### 3.1 Prerequisites

Install build and testing dependencies:

```bash
sudo apt-get update
sudo apt-get install -y \
  debhelper dh-make dpkg-dev fakeroot lintian \
  gnupg2 curl git

# Verify installations
dpkg-buildpackage --version
lintian --version
gpg --version
```

### 3.2 Build First Package

Test the build system:

```bash
cd ~/aurora

# Test build script
chmod +x scripts/test-build.sh
./scripts/test-build.sh 1.0.0

# Expected output:
# ✅ Build successful
# 📦 Package: aurora-themes_1.0.0_all.deb (NN KB)
# ✅ Lintian passed
# ✅ Build test successful!
```

### 3.3 Build All Packages

```bash
cd ~/aurora

# Build all 18 packages
make clean
make build

# Watch for:
# 📦 Building aurora...
# 📦 Building aurora-themes...
# [... 18 packages total ...]
# ✅ Build complete!
# Packages available: 18 .deb files

# Verify packages created
ls -lh aurora-*_1.0.0_all.deb | wc -l
# Should show: 18
```

### 3.4 Validation

```bash
# Run lintian checks
make lint

# Expected:
# ✅ All packages verified successfully!
# Errors: 0
# Warnings: [some OK, non-critical]

# Test installation simulation
chmod +x scripts/test-installation.sh
./scripts/test-installation.sh 1.0.0

# Expected:
# ✅ Installation testing complete!
# ✅ All packages verified successfully!
```

### 3.5 Multi-Distro Testing

Test on different Linux distributions:

```bash
# Ubuntu 20.04
docker run -it ubuntu:20.04 bash
# Inside: apt update && apt install curl && curl ... | bash && sudo apt install aurora

# Ubuntu 22.04
docker run -it ubuntu:22.04 bash

# Ubuntu 24.04
docker run -it ubuntu:24.04 bash

# Debian 11
docker run -it debian:11 bash

# Debian 12
docker run -it debian:12 bash

# Expected: All installations succeed without errors
```

### 3.6 Desktop Environment Testing

```bash
# GNOME: Check theme appears in Settings → Appearance
# KDE: Check in System Settings → Appearance
# Xfce: Check in Appearance settings

# Verify post-install scripts ran:
fc-list | grep -i aurora
# Should show aurora fonts

dpkg -L aurora-themes | grep share/themes/Aurora
# Should show theme files
```

### ✅ Phase 3 Complete When

- ✅ All 18 packages build successfully
- ✅ Lintian validation passes (0 errors)
- ✅ Installation works on Ubuntu 20.04, 22.04, 24.04
- ✅ Installation works on Debian 11, 12
- ✅ Themes appear in desktop settings
- ✅ Fonts available after installation
- ✅ Post-install scripts execute properly

---

## Phase 4: Hosting (Weeks 4-5)

### 4.1 Choose Hosting Solution

Three options (select one):

| Option | Setup Time | Difficulty | Cost | Speed |
|--------|-----------|-----------|------|-------|
| GitHub Pages | 10 min | Very Easy | Free | ⭐⭐⭐⭐⭐ |
| Cloudflare R2 | 20 min | Easy | ~$15/mo | ⭐⭐⭐⭐⭐ |
| AWS S3+CF | 30 min | Medium | ~$50+/mo | ⭐⭐⭐⭐⭐ |

**For Phase 3-6, recommend: GitHub Pages (simplest, fastest)**

### 4.2 GitHub Pages Setup (if chosen)

```bash
cd ~/aurora

# 1. Create gh-pages branch
git checkout --orphan gh-pages
git reset --hard
echo "Aurora APT Repository" > index.html
git add index.html
git commit -m "Initialize GitHub Pages"
git push origin gh-pages

# 2. Return to main
git checkout main

# 3. Deploy repository
./scripts/setup-repository.sh
./scripts/publish-repository.sh 1.0.0 testing

# 4. Copy to gh-pages
git checkout gh-pages
mkdir -p dists pool
cp -r ~/aurora/repository/dists/* dists/
cp -r ~/aurora/repository/pool/* pool/
git add dists/ pool/
git commit -m "Deploy Aurora 1.0.0"
git push origin gh-pages
git checkout main

# 5. Repository accessible at:
# https://aurora-linux.github.io/repo/dists/stable/
```

### 4.3 Domain Configuration

```bash
# If using custom domain (optional):
# 1. Buy domain: aurora.linux
# 2. Add DNS CNAME record:
#    archive.aurora.linux CNAME aurora-linux.github.io

# Or use GitHub Pages default:
# https://aurora-linux.github.io/repo/
```

### 4.4 Test Hosting

```bash
# Test repository accessibility
curl -I https://archive.aurora.linux/dists/stable/Release
# Should return: HTTP/1.1 200 OK

# Download Release file
curl https://archive.aurora.linux/dists/stable/Release

# Test package download
curl -O https://archive.aurora.linux/pool/main/a/aurora/aurora_1.0.0_all.deb
```

### ✅ Phase 4 Complete When

- ✅ Repository accessible via HTTPS
- ✅ Domain configured (or gh-pages URL working)
- ✅ TLS certificate valid
- ✅ Release files downloadable
- ✅ Package files downloadable
- ✅ Installation script deployed

---

## Phase 5: GPG Signing (Week 5)

### 5.1 Generate Repository Key

```bash
gpg --full-generate-key

# Prompts:
# - Type: RSA and RSA (default)
# - Size: 4096
# - Validity: 4y
# - Name: Aurora Linux Repository
# - Email: aurora@example.com
# - Passphrase: [VERY STRONG - 20+ characters]

# Save output:
# Key ID: 1234567890ABCDEF...
```

### 5.2 Export Public Key

```bash
# Export key
gpg --export -a aurora@example.com > aurora-archive-keyring.gpg

# Copy to repository for distribution
cp aurora-archive-keyring.gpg ~/aurora/

# Upload to repository hosting
# (GitHub Pages, etc.)
```

### 5.3 Test Signing

```bash
# Create test Release file
cd ~/aurora/repository/dists/stable
cat > test-release <<'EOF'
Origin: Aurora Linux
Label: Aurora Stable
Suite: stable
Codename: stable
Date: Fri, 02 Aug 2026 00:00:00 +0000
Architectures: all
Components: main
Description: Aurora
MD5Sum:
 d41d8cd98f00b204e9800998ecf8427e                0 main/binary-all/Packages
EOF

# Sign
gpg --default-key aurora@example.com --clearsign -a -o test-signed test-release

# Verify
gpg --verify test-signed

# Expected: Good signature from "Aurora Linux Repository"
```

### 5.4 Add to GitHub Secrets

For CI/CD signing:

```bash
# Encode key
gpg --export-secret-keys -a aurora@example.com | base64 -w0 > /tmp/key.b64

# Add to GitHub Settings → Secrets:
# GPG_PRIVATE_KEY = [base64 content]
# GPG_KEY_ID = 1234567890ABCDEF
# GPG_KEY_PASSPHRASE = [your passphrase]
```

### 5.5 Test CI/CD Signing

```bash
# Make a test commit and tag
git tag -a v1.0.0-test -m "Test release"
git push origin v1.0.0-test

# Watch GitHub Actions:
# gh run list

# Should see:
# ✅ Build successful
# ✅ Signing successful
# ✅ Release published

# Clean up test tag
git push origin :v1.0.0-test
git tag -d v1.0.0-test
```

### ✅ Phase 5 Complete When

- ✅ GPG key generated (4096-bit RSA)
- ✅ Public key exported and distributed
- ✅ Signing works manually
- ✅ GitHub Secrets configured
- ✅ CI/CD signing works automatically
- ✅ Users can verify signatures

---

## Phase 6: Production Release (Week 6)

### 6.1 Pre-Release Checklist

```bash
# ✅ All testing complete (Phase 3)
# ✅ Hosting ready (Phase 4)
# ✅ GPG signing configured (Phase 5)
# ✅ Documentation finalized
# ✅ CHANGELOG.md updated
# ✅ Release notes prepared
# ✅ Announcements drafted

# Verify:
git status
# Should show: nothing to commit, working tree clean

make build
# Should succeed

make lint
# Should show: 0 errors
```

### 6.2 Update Version

```bash
# Update CHANGELOG.md
nano CHANGELOG.md
# Add entry for 1.0.0

# Commit version bump
git add CHANGELOG.md
git commit -m "chore: Bump version to 1.0.0 for release"
```

### 6.3 Create Release Tag

```bash
# Create annotated tag (triggers GitHub Actions)
git tag -a v1.0.0 -m "Aurora 1.0.0 - Production Release

Aurora Linux Design System v1.0.0

Features:
- 18 packages covering complete design system
- Themes for GTK, Qt, KDE Plasma
- 2000+ icon set
- System typography
- Terminal/IDE integrations
- Multi-desktop support
- GPG-signed packages
- Secure APT repository

Installation:
  curl https://get.aurora.linux | sudo bash

Documentation:
  https://aurora.linux
  https://github.com/aurora-linux/aurora/docs

License: MIT/OFL-1.1"

# Push tag (triggers everything)
git push origin v1.0.0

# Watch GitHub Actions
gh run list
# Should build → sign → publish automatically
```

### 6.4 Monitor Release

```bash
# Watch actions
gh run watch

# Or check online:
# https://github.com/aurora-linux/aurora/actions

# Expected timeline:
# 1. Build starts (5 min)
# 2. GPG signing (2 min)
# 3. Repository publish (3 min)
# 4. GitHub Release created (1 min)
# 5. Deployment complete (11 min total)
```

### 6.5 Verify Released Packages

```bash
# Check GitHub Release
gh release list

# Check repository
curl https://archive.aurora.linux/dists/stable/Release

# Try installation (on test system)
curl https://get.aurora.linux | sudo bash
sudo apt install aurora
```

### 6.6 Publish Announcements

```bash
# Create blog post on https://aurora.linux/blog

# Announce on social media:
# - Twitter/X
# - Mastodon
# - Reddit (r/linux)
# - Linux forums

# Email newsletter (if applicable)

# Post in community channels:
# - Discord
# - Slack
# - etc.
```

### 6.7 Post-Release Support

```bash
# Monitor issues
gh issue list

# Respond to questions
# Answer in discussions
# Help with installation issues

# Watch for bugs
# Plan v1.0.1 if issues found
# Or start v1.1.0 feature development
```

### ✅ Phase 6 Complete When

- ✅ v1.0.0 tag pushed
- ✅ GitHub Actions completed successfully
- ✅ Packages published to repository
- ✅ GitHub Release created
- ✅ Users can install via `apt install aurora`
- ✅ Announcements published
- ✅ First users installing from APT

---

## Quick Reference Timeline

| Week | Phase | Key Actions | Expected Result |
|------|-------|-------------|-----------------|
| 4 | Phase 3 | Build, test, verify | 18 packages built & tested |
| 4-5 | Phase 4 | Set up hosting | Repository accessible |
| 5 | Phase 5 | Configure GPG | Signing works |
| 6 | Phase 6 | Release | v1.0.0 published |

---

## Troubleshooting Guide

### Builds failing
```bash
# Check build dependencies
dpkg-buildpackage --version

# Check for missing dependencies
sudo apt-get install debhelper dh-make dpkg-dev fakeroot

# Try single package first
cd packages/aurora-themes
dpkg-buildpackage -us -uc
```

### Lintian errors
```bash
# Review specific errors
lintian -E aurora-themes_*.deb

# Fix common issues:
# - File permissions: chmod 644 <file>
# - Executable scripts: only debian/ scripts
# - Correct copyright format
```

### Repository not accessible
```bash
# Check hosting is up
curl -I https://archive.aurora.linux/

# Verify DNS
nslookup archive.aurora.linux

# Check TLS certificate
openssl s_client -connect archive.aurora.linux:443
```

### GPG signature errors
```bash
# Re-test signing
gpg --default-key aurora@example.com --clearsign -a -o test Release

# Verify key imported
gpg --list-keys aurora@example.com

# Check GitHub Secrets
gh secret list
```

### GitHub Actions failing
```bash
# Check logs
gh run view <run-id>

# Common issues:
# - GPG key not imported
# - Passphrase incorrect
# - SSH key missing
# - Permissions issues
```

---

## Success Metrics

After Phase 3-6, you should have:

✅ 18 Aurora packages built and tested  
✅ Repository hosted and accessible  
✅ GPG signing working end-to-end  
✅ v1.0.0 released to production  
✅ Users installing via APT  
✅ Community engagement started  
✅ First feedback/bug reports received  

---

## Next Steps After v1.0.0

### v1.0.1 (Patch)
- Fix critical bugs
- Security updates
- Release in 1-2 weeks if needed

### v1.1.0 (Minor)
- New components (e.g., extended icons)
- New integrations
- Enhanced documentation
- Release in 6-8 weeks

### Parallel Efforts
- Snap package
- Flatpak
- AUR package
- Documentation improvements

---

**Phase 3-6 Complete. Aurora is production-ready for v1.0.0 release!**
