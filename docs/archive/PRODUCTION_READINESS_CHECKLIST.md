# Aurora APT Repository — Production Readiness Checklist

**Complete all items below before publishing to production.**

---

## ✅ Infrastructure & Hosting

- [ ] **Domain:** `aurora.linux` purchased and DNS configured
- [ ] **Archive subdomain:** `archive.aurora.linux` CNAME points to CDN
- [ ] **Hosting selected:** GitHub Pages OR Cloudflare R2 OR AWS S3+CloudFront
- [ ] **HTTPS certificate:** Valid TLS certificate for `archive.aurora.linux`
- [ ] **HTTPS enforced:** HTTP → HTTPS redirect configured
- [ ] **CDN configured:** Caching headers set appropriately
  - [ ] Pool files: `Cache-Control: max-age=31536000, immutable`
  - [ ] Dists files: `Cache-Control: max-age=3600`
- [ ] **Backups configured:** Automated backup of repository (daily)
- [ ] **Monitoring configured:** Uptime monitoring for archive.aurora.linux
- [ ] **Alerts configured:** Notification on downtime/errors

---

## ✅ Security

### GPG Keys
- [ ] **Repository key generated:** 4096-bit RSA, 4-year validity
- [ ] **Key ID documented:** Shared in installation instructions
- [ ] **Public key published:** `aurora-archive-keyring.gpg` available
- [ ] **Private key secured:** Stored in secrets manager (not git)
- [ ] **Private key encrypted:** Password-protected
- [ ] **Offline backup:** Key backed up on encrypted USB (optional but recommended)
- [ ] **Key fingerprint verified:** Published on website for user verification

### GitHub Actions Security
- [ ] **GPG key stored:** In GitHub Secrets (encrypted)
- [ ] **GPG passphrase stored:** In GitHub Secrets (encrypted)
- [ ] **AWS credentials stored:** In GitHub Secrets (if using S3)
- [ ] **Secrets not exposed:** Verified no secrets in logs/artifacts
- [ ] **Workflow permissions:** Limited to necessary scopes
- [ ] **Branch protection:** Main branch requires review for changes
- [ ] **Deployment branch policies:** Restrict deployment to protected branches
- [ ] **No hardcoded secrets:** Verified no passwords in repository

### Package Integrity
- [ ] **Packages signed:** All .deb files signed with GPG
- [ ] **Release file signed:** `Release` and `InRelease` files signed
- [ ] **Signature verification tested:** Users can verify packages locally
- [ ] **SHA256 hashes included:** All packages have checksums in Packages file
- [ ] **Reproducible builds:** Deterministic builds enabled (SOURCE_DATE_EPOCH)
- [ ] **Supply chain audit:** Dependencies checked for known CVEs

### Access Control
- [ ] **GitHub org security:** 2FA required for all maintainers
- [ ] **Deployment keys rotated:** No personal access tokens
- [ ] **S3 bucket policies:** Private by default (if using S3)
- [ ] **Cloud credentials rotated:** Quarterly rotation scheduled
- [ ] **RBAC configured:** Only authorized users can publish releases

---

## ✅ Debian Packaging

### Control Files
- [ ] **control file:** Valid format, no syntax errors
- [ ] **copyright file:** Complete with all authors/licenses
- [ ] **changelog:** Entry for every release, Debian format
- [ ] **standards-version:** Set to latest (4.6.2+)

### Debian Scripts
- [ ] **postinst script:** Tested, handles all upgrade scenarios
- [ ] **postrm script:** Cleans up after removal
- [ ] **preinst script:** Pre-flight checks run successfully
- [ ] **prerm script:** Pre-removal cleanup (if needed)
- [ ] **All scripts:** Fail gracefully on errors, exit with proper codes

### Package Content
- [ ] **No hardcoded paths:** All paths use standard locations (/usr/share/*, etc.)
- [ ] **File permissions:** Correct (644 for files, 755 for dirs)
- [ ] **Ownership:** Files owned by root:root
- [ ] **No duplicate files:** No conflicts between packages
- [ ] **No executable scripts:** CSS/JSON/icons not executable
- [ ] **Symlinks intact:** Resolved correctly after installation
- [ ] **No architecture binaries:** All packages marked Architecture: all
- [ ] **Reasonable size:** No excessive disk usage

### Linting
- [ ] **Lintian clean:** `lintian -EviI *.deb` passes
  - [ ] No errors (E:)
  - [ ] Warnings reviewed and acceptable (W:)
  - [ ] Info messages noted (I:)
- [ ] **Duplicate files:** No duplicates between packages
- [ ] **Dependency chain:** Valid (no circular dependencies)
- [ ] **Conflicts/Replaces accurate:** Properly represent actual conflicts

---

## ✅ Repository Management

### Aptly Setup
- [ ] **aptly installed:** Version 1.12+ (check with `aptly version`)
- [ ] **aptly configured:** ~/.aptly.conf exists and valid
- [ ] **Repositories created:** aurora-stable, aurora-testing, aurora-unstable
- [ ] **GPG signing configured:** gpgPersonalKey set in config
- [ ] **Publishing endpoint:** Filesystem path correctly configured

### Repository Content
- [ ] **All packages added:** aurora, aurora-themes, aurora-icons, etc.
- [ ] **Snapshots created:** Immutable snapshots for rollback
- [ ] **Metadata generated:** Packages, Release files created
- [ ] **Release files signed:** Release.gpg and InRelease valid
- [ ] **Pool structure correct:** Files in `pool/main/[a-z]/[package]/`
- [ ] **Contents file generated:** File-to-package mappings (optional but recommended)

### Repository Verification
- [ ] **Packages listed:** `apt-cache search aurora` returns packages
- [ ] **Metadata valid:** `apt-cache show aurora` displays correct info
- [ ] **Dependencies correct:** `apt-cache depends aurora` shows expected packages
- [ ] **Hash verification:** `sha256sum` of packages matches Packages file
- [ ] **Mirror sync:** Repository can be mirrored by third parties
- [ ] **HTTP/S access:** Repository accessible via `curl`/`wget`

---

## ✅ Installation & User Experience

### Installation Methods
- [ ] **One-line installer:** Curl script tested end-to-end
- [ ] **Manual installation:** Step-by-step guide tested
- [ ] **Installation tested:** On Ubuntu 20.04, 22.04, 24.04
- [ ] **Installation tested:** On Debian 11, 12
- [ ] **Different DEs tested:** GNOME, KDE Plasma, Xfce tested
- [ ] **Fresh systems tested:** Clean Ubuntu/Debian installation works

### Post-Installation
- [ ] **Themes appear:** Selectable in system settings
- [ ] **Icons installed:** Icon cache updated
- [ ] **Fonts available:** `fc-list | grep -i aurora` shows fonts
- [ ] **Color schemes loaded:** Color palette accessible
- [ ] **Wallpapers available:** Backgrounds appear in settings
- [ ] **No errors:** No package errors during installation

### Upgrade Experience
- [ ] **Auto-upgrade works:** `apt update && apt upgrade` updates Aurora
- [ ] **Clean removal:** `apt remove aurora` removes all packages cleanly
- [ ] **No orphaned files:** No leftover configuration after removal
- [ ] **Downgrade possible:** Can install specific older version if needed

### User Documentation
- [ ] **Installation guide:** Clear, step-by-step instructions
- [ ] **Configuration guide:** How to use Aurora on different DEs
- [ ] **FAQ page:** Common questions answered
- [ ] **Troubleshooting guide:** Solutions for common issues
- [ ] **Support contacts:** How to report bugs/get help

---

## ✅ CI/CD Pipeline

### GitHub Actions
- [ ] **Release workflow:** Triggers on `git tag v*`
- [ ] **Build succeeds:** Packages build without errors
- [ ] **Tests pass:** Lintian, validation tests pass
- [ ] **Signing works:** Packages signed successfully
- [ ] **Publishing works:** Packages uploaded to repository
- [ ] **Artifacts attached:** GitHub Release has all artifacts

### Automation
- [ ] **Version bumping automated:** Script bumps version automatically
- [ ] **Changelog auto-generated:** Release notes generated from commits
- [ ] **GPG signing automated:** No manual key entry required
- [ ] **Repository refresh automated:** aptly commands run automatically
- [ ] **Deployment gated:** Manual approval before production deployment (optional)

### Monitoring & Logging
- [ ] **Workflow logs accessible:** GitHub Actions logs viewable
- [ ] **Build failures alert:** Notifications on failed builds
- [ ] **Deployment logged:** Repository changes logged/audited
- [ ] **Secret rotation logged:** Key changes tracked

---

## ✅ Release Channels

### Stable Channel
- [ ] **dists/stable configured:** Serves production releases
- [ ] **Archive policy:** Old versions available for downgrade
- [ ] **Update frequency:** Documented (e.g., "Every 6-8 weeks")

### Testing Channel
- [ ] **dists/testing configured:** Serves beta/RC versions
- [ ] **Separate from stable:** Testing updates don't affect stable users
- [ ] **Clear communication:** Users know testing may have bugs
- [ ] **Upgrade path clear:** Testing → Stable path documented

### Unstable Channel (Optional)
- [ ] **dists/unstable configured:** Nightly builds
- [ ] **Clear labeling:** Marked as experimental/development
- [ ] **Automated nightly:** Daily builds from develop branch

---

## ✅ Documentation

### User Documentation
- [ ] **README.md:** Project overview in repository
- [ ] **INSTALLATION.md:** Installation instructions for all methods
- [ ] **CONTRIBUTING.md:** How to contribute to Aurora
- [ ] **docs/FAQ.md:** Common questions and answers
- [ ] **docs/TROUBLESHOOTING.md:** Solutions for common problems
- [ ] **docs/SECURITY.md:** Security policy and reporting

### Developer Documentation
- [ ] **docs/ARCHITECTURE.md:** Design and architecture overview
- [ ] **docs/RELEASE_PROCESS.md:** How to make releases
- [ ] **docs/REPOSITORY_SETUP.md:** Setting up repository (this file)
- [ ] **docs/PACKAGE_STRUCTURE.md:** Package layout explained
- [ ] **Makefile:** Build targets documented with `make help`
- [ ] **Docker/DevContainer:** Development environment documented

### Website Documentation
- [ ] **https://aurora.linux/install:** Installation page with one-liner
- [ ] **https://aurora.linux/docs:** Documentation hub
- [ ] **https://archive.aurora.linux:** Repository home page
- [ ] **https://github.com/aurora-linux/aurora:** GitHub repository

---

## ✅ Testing

### Installation Testing
- [ ] **Ubuntu 20.04 LTS:** Tested on latest updates
- [ ] **Ubuntu 22.04 LTS:** Tested on latest updates
- [ ] **Ubuntu 24.04 LTS:** Tested on latest updates
- [ ] **Debian 11:** Tested on latest updates
- [ ] **Debian 12:** Tested on latest updates
- [ ] **Raspberry Pi OS:** Tested on ARM64 if targeting ARM

### Desktop Environment Testing
- [ ] **GNOME:** Full installation and theme selection tested
- [ ] **KDE Plasma:** Full installation and theme selection tested
- [ ] **Xfce:** Themes installed and selectable
- [ ] **Cinnamon:** Icons/themes integrated (if applicable)
- [ ] **MATE:** Themes selectable (if applicable)

### Upgrade Testing
- [ ] **Minor version:** 1.0.0 → 1.0.1 upgrade tested
- [ ] **Minor version:** 1.0.0 → 1.1.0 upgrade tested
- [ ] **Major version:** 1.x.x → 2.0.0 upgrade tested (if applicable)
- [ ] **Clean removal:** `apt remove aurora` leaves no orphans
- [ ] **Rollback:** Can downgrade to previous version if needed

### Regression Testing
- [ ] **Existing packages:** Installation doesn't break other packages
- [ ] **System functionality:** System still boots/runs after installation
- [ ] **Other themes:** No conflict with Ubuntu/GNOME default themes
- [ ] **Font system:** Other fonts still work after aurora-fonts install

---

## ✅ Performance & Scalability

### Repository Performance
- [ ] **Repository download speed:** Metadata downloads in <5 seconds
- [ ] **Concurrent users:** Repository handles 10+ simultaneous downloads
- [ ] **Bandwidth usage:** CDN logs show acceptable bandwidth
- [ ] **Cache hit rates:** CDN achieving >80% cache hit ratio
- [ ] **Latency:** <200ms global latency (via CDN)

### Package Size
- [ ] **Total size reasonable:** All packages <1GB combined
- [ ] **Largest single package:** Not >500MB
- [ ] **Compression working:** .gz files significantly smaller than uncompressed

---

## ✅ Compliance & Standards

### Debian Policy
- [ ] **Policy Manual compliance:** Follows Debian Policy Manual §5
- [ ] **Control format:** Valid RFC 822 format
- [ ] **Relationships:** Depends/Recommends/Suggests properly used
- [ ] **Standards version:** Latest (4.6.2+)
- [ ] **Priority correct:** Set to `optional` for most packages

### Licensing
- [ ] **All assets licensed:** Every file has valid open-source license
- [ ] **Licenses included:** Full license text in copyright file
- [ ] **No proprietary:** No commercial/proprietary assets included
- [ ] **Compliance verified:** License compatibility checked

### Accessibility
- [ ] **WCAG 2.1 AA:** Website meets accessibility standards (if published)
- [ ] **Color contrast:** Themes meet minimum contrast ratios
- [ ] **Accessibility variants:** High-contrast/dyslexia-friendly included
- [ ] **Documentation accessible:** Guides work with screen readers

---

## ✅ Launch Preparation

### Pre-Launch
- [ ] **Soft launch:** Beta testing with limited audience
- [ ] **Feedback collection:** Issues gathered from beta users
- [ ] **Bug fixes:** Critical issues resolved before public launch
- [ ] **Documentation reviewed:** All guides proofread and tested
- [ ] **Website ready:** https://aurora.linux fully functional

### Launch
- [ ] **Announcement post:** Blog post written
- [ ] **Social media plan:** Posts scheduled for launch
- [ ] **Press kit:** Available for community sharing
- [ ] **Changelog complete:** Full release notes prepared
- [ ] **Support channels open:** Email/Discord/GitHub ready for support

### Post-Launch
- [ ] **Monitor issues:** Watch for bug reports
- [ ] **Update documentation:** Based on user feedback
- [ ] **Security monitoring:** Watch for CVEs in dependencies
- [ ] **Usage metrics:** Track adoption/downloads
- [ ] **Community engagement:** Respond to issues/PRs promptly

---

## ✅ Long-Term Operations

### Ongoing Maintenance
- [ ] **Release schedule defined:** E.g., "Every 6-8 weeks"
- [ ] **Security updates policy:** How quickly CVEs are addressed
- [ ] **Maintenance window:** When updates can be deployed
- [ ] **Rollback procedure:** Documented and tested

### Support Plan
- [ ] **Support duration:** How long old versions are supported
- [ ] **Bug fix backports:** Policy for backporting fixes
- [ ] **Feature requests:** Process for accepting new features
- [ ] **Community guidelines:** Code of conduct established

### Monitoring & Alerting
- [ ] **Repository uptime:** Monitored (e.g., Uptime Robot)
- [ ] **Certificate expiration:** Alert before renewal needed
- [ ] **GPG key expiration:** Alert before key needs rotation
- [ ] **Disk space:** Alert if repository grows too large
- [ ] **Build failures:** Notification on release pipeline failures

---

## Sign-Off

**Project Lead:** ________________  **Date:** ______________

**Quality Assurance:** ________________  **Date:** ______________

**Infrastructure Lead:** ________________  **Date:** ______________

**Security Review:** ________________  **Date:** ______________

---

**All items must be checked before production launch. Any unchecked items block launch until resolved.**
