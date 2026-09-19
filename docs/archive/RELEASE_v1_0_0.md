# Aurora v1.0.0 Release Workflow

**Complete end-to-end guide for releasing Aurora 1.0.0 to production.**

---

## Release Checklist

### Pre-Release: Week 6 Preparation

- [ ] **Testing Phase Complete**
  - [ ] Build successful on development system
  - [ ] All packages lintian-clean
  - [ ] Installation tested on Ubuntu 20.04, 22.04, 24.04
  - [ ] Installation tested on Debian 11, 12
  - [ ] Tested on GNOME, KDE Plasma, Xfce
  - [ ] Post-install scripts execute correctly
  - [ ] Post-remove cleanup verified

- [ ] **Hosting Ready**
  - [ ] Domain configured (archive.aurora.linux)
  - [ ] TLS certificate valid (https://)
  - [ ] CDN caching working
  - [ ] Installation script deployed
  - [ ] Repository structure tested

- [ ] **GPG Setup Complete**
  - [ ] Repository key generated (4096-bit RSA)
  - [ ] Key ID documented
  - [ ] Public key exported (aurora-archive-keyring.gpg)
  - [ ] Private key in GitHub Secrets
  - [ ] Signing workflow tested
  - [ ] Users can verify signatures

- [ ] **Documentation Finalized**
  - [ ] Installation guide published
  - [ ] Configuration guide complete
  - [ ] FAQ populated
  - [ ] Troubleshooting documented
  - [ ] Contributing guidelines ready

- [ ] **Release Artifacts Prepared**
  - [ ] CHANGELOG.md updated
  - [ ] Release notes written
  - [ ] Blog post drafted
  - [ ] Social media announcement ready

---

## Step-by-Step Release Process

### Stage 1: Pre-Release Verification (Day 1)

```bash
# 1. Verify git is clean
git status
# Should show: nothing to commit, working tree clean

# 2. Update version number in all files
# docs/APT_REPOSITORY_INDEX.md
# debian/control (if using meta-package root)
# Cargo.toml (if applicable)

# 3. Update CHANGELOG.md
cat >> CHANGELOG.md <<'EOF'
## [1.0.0] - 2026-08-09

### Added
- Complete Aurora design system (themes, icons, fonts)
- 18-package Debian distribution ecosystem
- APT repository infrastructure (stable/testing/unstable)
- GPG signing and verification
- GitHub Actions CI/CD pipeline
- Multi-desktop support (GNOME, KDE Plasma, Xfce, etc.)

### Features
- GTK/Qt/Plasma themes with light/dark variants
- 2000+ icons with multiple sizes
- System typography and color palettes
- Terminal, VS Code, JetBrains integrations
- KDE Plasma and GNOME Shell integration
- Accessibility variants (high-contrast, dyslexia-friendly)
- Plymouth boot splash
- 100% open-source (MIT/OFL-1.1 licenses)

### Installation
```bash
curl https://get.aurora.linux | sudo bash
```

### Documentation
- Complete architecture documentation (45,000+ words)
- Package setup guide
- Repository setup guide
- Production readiness checklist
- GPG signing guide

### Contributors
- Aurora Team
- Community contributors

### Links
- Website: https://aurora.linux
- Repository: https://github.com/aurora-linux/aurora
- Issues: https://github.com/aurora-linux/aurora/issues
EOF

# 4. Commit version bump
git add CHANGELOG.md
git commit -m "chore: Bump version to 1.0.0 for release"

# 5. Verify all tests pass
make lint
# Should show: All packages verified

# 6. Build final packages
make clean
make build
# Should create aurora-*_1.0.0_all.deb files

# 7. Verify all packages
make lint
# Should show: All packages clean
```

### Stage 2: Tag Release (Day 2)

```bash
# Create annotated tag
git tag -a v1.0.0 -m "Aurora 1.0.0 - First production release

Aurora is a comprehensive Linux design system providing a cohesive 
visual experience across desktop environments.

Features:
- GTK, Qt, Plasma themes
- Icon themes (2000+ icons)
- System typography
- Terminal/IDE integrations
- Multi-desktop support (GNOME, KDE, Xfce, etc.)
- Full open-source

Installation:
  curl https://get.aurora.linux | sudo bash

Documentation:
  https://aurora.linux/docs
  https://github.com/aurora-linux/aurora

License: MIT/OFL-1.1"

# Verify tag
git tag -v v1.0.0
# Should show: Good signature from "Your Name"

# Push tag (triggers GitHub Actions)
git push origin v1.0.0
```

### Stage 3: GitHub Actions Automation (Automatic)

When you push the tag, GitHub Actions automatically:

```yaml
1. Build all 18 packages
2. Import GPG key from Secrets
3. Sign packages with dpkg-sig
4. Generate repository metadata with aptly
5. Sign Release files with GPG
6. Create GitHub Release with artifacts
7. Upload to S3/R2 (if configured)
8. Deploy to GitHub Pages (if enabled)
9. Generate release notes
```

**Monitor workflow:**
```bash
# Check GitHub Actions
gh workflow list
gh run list

# Or check directly on GitHub:
# https://github.com/aurora-linux/aurora/actions

# Should see:
# ✅ Build successful
# ✅ Lintian passed
# ✅ Signing complete
# ✅ Release published
```

### Stage 4: Verify Repository Published (Day 2-3)

```bash
# Test repository is accessible
curl -I https://archive.aurora.linux/dists/stable/Release
# Should return: HTTP/1.1 200 OK

# Download Release file
curl https://archive.aurora.linux/dists/stable/Release | head -20

# Verify signature
curl -O https://archive.aurora.linux/dists/stable/InRelease
gpg --verify InRelease
# Should show: Good signature from "Aurora Linux Repository"

# Test package download
curl -O https://archive.aurora.linux/pool/main/a/aurora/aurora_1.0.0_all.deb
ls -lh aurora_1.0.0_all.deb

# Test installation
sudo dpkg -i aurora_1.0.0_all.deb
dpkg -L aurora | head -10
sudo dpkg -r aurora
```

### Stage 5: Publish Announcements (Day 3)

```bash
# 1. Create release notes
cat > /tmp/release-notes.md <<'EOF'
# 🎨 Aurora 1.0.0 Released!

Aurora is now available as a production-ready Debian/Ubuntu package.

## Installation

```bash
curl https://get.aurora.linux | sudo bash
```

Or manually:

```bash
wget https://archive.aurora.linux/aurora-archive-keyring.gpg
sudo apt-key add aurora-archive-keyring.gpg

echo "deb https://archive.aurora.linux/dists/stable main" | \
  sudo tee /etc/apt/sources.list.d/aurora.sources

sudo apt update
sudo apt install aurora
```

## What's Included

- **GTK/Qt/Plasma Themes**: Cohesive visual experience across all desktop environments
- **Icon System**: 2000+ icons in multiple sizes and variants
- **Typography**: Carefully curated font collection
- **Color Palettes**: Design tokens for consistent colors
- **Terminal Themes**: Color schemes for terminal emulators
- **IDE Themes**: VS Code and JetBrains integration
- **Desktop Integration**: Deep integration with GNOME, KDE Plasma
- **Accessibility**: High-contrast, dyslexia-friendly variants
- **Boot Splash**: Plymouth theme for boot experience

## Supported Platforms

### Linux Distributions
- Ubuntu 20.04 LTS
- Ubuntu 22.04 LTS
- Ubuntu 24.04 LTS
- Debian 11
- Debian 12

### Desktop Environments
- GNOME Shell
- KDE Plasma
- Xfce
- Cinnamon
- MATE
- LXQt

## Features

✅ **100% Open Source** — MIT and OFL-1.1 licenses  
✅ **Production Ready** — Thoroughly tested across platforms  
✅ **Easy Installation** — One-liner installation script  
✅ **Automatic Updates** — Standard apt package updates  
✅ **Secure** — GPG-signed packages and repository  
✅ **Scalable** — Global CDN for fast downloads  
✅ **Well Documented** — 45,000+ words of documentation  

## Links

- **Website**: https://aurora.linux
- **GitHub**: https://github.com/aurora-linux/aurora
- **Documentation**: https://aurora.linux/docs
- **Issues**: https://github.com/aurora-linux/aurora/issues
- **Discussions**: https://github.com/aurora-linux/aurora/discussions

## Release Notes

See [CHANGELOG.md](https://github.com/aurora-linux/aurora/blob/main/CHANGELOG.md) for full details.
EOF

# 2. Publish GitHub Release
gh release create v1.0.0 \
  --title "Aurora 1.0.0" \
  --notes-file /tmp/release-notes.md \
  --draft=false

# 3. Create blog post
# Publish to website: https://aurora.linux/blog/aurora-1-0-0-released

# 4. Social media announcements
# Twitter: "🎨 Aurora 1.0.0 is released! A comprehensive Linux design system..."
# Mastodon: Same announcement
# LinkedIn: Professional announcement

# 5. Email to users
# Newsletter (if applicable): Announce availability
```

### Stage 6: Verify End-to-End Installation (Day 3-4)

```bash
# Test on clean Ubuntu VM
docker run -it ubuntu:24.04 bash

# Inside container:
apt-get update
apt-get install -y curl

curl https://get.aurora.linux | bash
# Should complete successfully

sudo apt install aurora
# Should install without errors

dpkg -l | grep aurora
# Should show all 18 packages installed

# Verify themes available
gsettings list-schemas | grep aurora
# Or check GNOME Settings for Aurora theme

# Test on KDE
docker run -it kde:latest bash
# Similar testing process
```

### Stage 7: Post-Release Monitoring (Week 7+)

```bash
# Monitor for issues
gh issue list
# Track bug reports

# Watch for support requests
# Forum/Discussions: Respond to user questions

# Monitor repository health
# Download stats, bandwidth usage, etc.

# Plan v1.0.1 patch release (if bugs found)
# Or start planning v1.1.0 (new features)
```

---

## Release Communication

### Blog Post Template

```markdown
# Aurora 1.0.0: A Comprehensive Linux Design System

Aurora is now production-ready and available for installation via APT 
package manager on Ubuntu and Debian.

## One-Liner Installation

```bash
curl https://get.aurora.linux | sudo bash
```

## The Vision

Aurora is more than themes. It's a complete, thoughtfully-designed system 
that brings visual coherence to the Linux desktop.

[Story and vision...]

## What You Get

- Themes for GTK, Qt, and KDE Plasma
- 2000+ icons covering common applications
- System typography carefully selected
- Color palette designed for visual harmony
- Integrations for terminal, editors, IDEs
- Accessibility variants for all users

## Installation

[Detailed instructions...]

## Getting Started

[Configuration guide...]

## Contributing

[How to help...]

## Support

[How to report issues, get help...]
```

### Twitter/Mastodon

```
🎨 Aurora 1.0.0 is now available!

A comprehensive Linux design system bringing visual harmony to your desktop.

✅ One-liner installation
✅ 18 packages for complete experience  
✅ GPG-signed & secure
✅ 100% open source

Install: https://get.aurora.linux
Docs: https://aurora.linux

#Linux #DesignSystem #OpenSource
```

---

## Post-Release Maintenance

### Monitor Health

```bash
# Check repository status
curl -s https://archive.aurora.linux/dists/stable/Release | head -20

# Monitor download stats (if tracking available)
# CloudFlare Analytics / AWS CloudWatch / GitHub Releases stats

# Watch issue tracker
gh issue list --label bug

# Respond to community
# - Answer questions in Discussions
# - Triage bug reports
# - Help with installation issues
```

### Version Bump for v1.0.1 (if bugs found)

```bash
# Fix bug
# Update version in debian/control files
# Update CHANGELOG.md

git add -A
git commit -m "chore: Bump to v1.0.1 - Bugfix release"
git tag -a v1.0.1 -m "Aurora 1.0.1 - Bugfix release"
git push origin main v1.0.1
# Automatically triggers release workflow again
```

### Plan v1.1.0 (new features)

```bash
# Create v1.1.0 development branch
git checkout -b develop

# Plan features for 1.1.0:
# - New color variants
# - Extended icon set
# - New theme components
# - Additional integrations

# After 6-8 weeks:
# Branch to v1.1.0-beta
# Community testing
# Release v1.1.0
```

---

## Success Criteria

After v1.0.0 release, you should see:

✅ Repository accessible worldwide (CDN working)  
✅ Installation script working for users  
✅ Packages installable via `apt install aurora`  
✅ Post-install scripts execute without errors  
✅ Themes appear in GNOME Settings, KDE System Settings  
✅ Icons appear in applications  
✅ Fonts available system-wide  
✅ Community feedback and bug reports  
✅ First user installations outside development team  
✅ No critical issues reported  

---

## Quick Reference: Release Commands

```bash
# Update files
nano CHANGELOG.md
git add CHANGELOG.md
git commit -m "chore: Bump version to 1.0.0 for release"

# Build and verify
make clean
make build
make lint

# Tag and push (triggers everything)
git tag -a v1.0.0 -m "Aurora 1.0.0 release"
git push origin v1.0.0

# Monitor
gh run list --workflow=release.yml

# Test installation (on separate machine or VM)
curl https://get.aurora.linux | sudo bash
sudo apt install aurora
```

---

## Timeline Summary

| Day | Action | Status |
|-----|--------|--------|
| Day 1 | Pre-release verification | ✅ Manual |
| Day 2 | Tag v1.0.0, push tag | ✅ Manual |
| Day 2 | GitHub Actions builds/signs/publishes | ✅ Automatic |
| Day 2-3 | Verify repository published | ✅ Manual |
| Day 3 | Publish announcements | ✅ Manual |
| Day 3-4 | Test end-to-end installation | ✅ Manual |
| Day 4+ | Monitor and support | ✅ Ongoing |

---

**Aurora v1.0.0 Release Complete!**

Next phase: v1.0.1 (bugfixes) or v1.1.0 (new features)
