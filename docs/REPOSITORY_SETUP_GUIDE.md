# Aurora APT Repository Setup — Quick Reference Guide

**Purpose:** Step-by-step walkthrough to set up Aurora's APT repository from scratch.

---

## Prerequisites

```bash
# Required tools
sudo apt-get install -y \
  debhelper dh-make lintian dpkg-dev fakeroot \
  aptly gnupg2 git

# Check versions
dpkg-query -W debhelper lintian aptly gnupg2
```

---

## Step 1: Create GPG Repository Key

```bash
# Generate 4096-bit RSA key (one-time)
gpg --full-generate-key

# When prompted:
# Key type: RSA
# Key size: 4096
# Validity: 4y (4 years)
# Name: Aurora Linux Repository
# Email: aurora@example.com
# Passphrase: [strong password, store in 1Password/Vault]

# Export public key for distribution
gpg --export -a aurora@example.com > aurora-archive-keyring.gpg

# Verify key ID
gpg --list-keys aurora@example.com
# Output: 1234567890ABCDEF1234567890ABCDEF12345678
export GPG_KEY_ID="1234567890ABCDEF1234567890ABCDEF12345678"
```

---

## Step 2: Configure aptly

```bash
# Create ~/.aptly.conf
cat > ~/.aptly.conf <<'EOF'
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
      "rootDir": "/var/www/aurora-repo",
      "linkMethod": "hardlink"
    }
  }
}
EOF

# Create repository directory
mkdir -p /var/www/aurora-repo
```

---

## Step 3: Initialize Repository Suites

```bash
# Create three suites: stable, testing, unstable
aptly repo create -architectures="all" -comment="Aurora Stable" aurora-stable
aptly repo create -architectures="all" -comment="Aurora Testing" aurora-testing
aptly repo create -architectures="all" -comment="Aurora Unstable" aurora-unstable

# Verify
aptly repo list
```

---

## Step 4: Add Packages to Repository

```bash
# Build your packages first
cd ~/aurora
make build
# Generates: aurora_1.0.0_all.deb, aurora-themes_1.0.0_all.deb, etc.

# Add to testing repository
aptly repo add aurora-testing ~/aurora/aurora_1.0.0_all.deb
aptly repo add aurora-testing ~/aurora/aurora-themes_1.0.0_all.deb
# ... repeat for all packages ...

# Verify
aptly repo show -with-packages aurora-testing
```

---

## Step 5: Create Snapshots

Snapshots are immutable versions of repositories (for rollback capability).

```bash
# Create snapshot of testing repository
aptly snapshot create aurora-testing-v1.0.0 from repo aurora-testing

# Verify snapshot
aptly snapshot show aurora-testing-v1.0.0
```

---

## Step 6: Publish Repositories

```bash
# Publish stable repository
aptly publish snapshot aurora-stable-v1.0.0 filesystem:filesystem

# Publish testing repository
aptly publish snapshot aurora-testing-v1.0.0 filesystem:filesystem -skip-signing=false

# Publish unstable repository (nightly)
aptly publish snapshot aurora-unstable-v1.0.0 filesystem:filesystem -skip-signing=false

# Verify
ls -la /var/www/aurora-repo/dists/
```

---

## Step 7: Sign Release Files

aptly creates unsigned Release files. You must sign them:

```bash
# Go to repository directory
cd /var/www/aurora-repo/dists/stable

# Create detached GPG signature
gpg --default-key aurora@example.com \
    --detach-sign --armor \
    -o Release.gpg Release

# Create inline signature (preferred)
gpg --default-key aurora@example.com \
    --clearsign --armor \
    --output InRelease Release

# Verify signature
gpg --verify InRelease
# Output: Good signature from "Aurora Linux Repository <aurora@example.com>"

# Repeat for testing/ and unstable/
cd ../testing && gpg --default-key aurora@example.com --detach-sign --armor -o Release.gpg Release
cd ../unstable && gpg --default-key aurora@example.com --detach-sign --armor -o Release.gpg Release
```

---

## Step 8: Serve Repository via HTTP

### Option A: Using Nginx (Self-Hosted)

```bash
# Install Nginx
sudo apt-get install nginx

# Create Nginx configuration
sudo tee /etc/nginx/sites-available/aurora-repo > /dev/null <<'EOF'
server {
    listen 80;
    listen [::]:80;
    
    server_name archive.aurora.linux;
    
    # Redirect HTTP to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    
    server_name archive.aurora.linux;
    
    # TLS certificates (Let's Encrypt)
    ssl_certificate /etc/letsencrypt/live/archive.aurora.linux/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/archive.aurora.linux/privkey.pem;
    
    # Enable gzip compression
    gzip on;
    gzip_types text/plain text/css application/json;
    
    # Serve repository
    root /var/www/aurora-repo;
    
    # Caching strategy
    location ~ ^/dists/ {
        expires 1h;
        add_header Cache-Control "public, max-age=3600";
    }
    
    location ~ ^/pool/ {
        expires 365d;
        add_header Cache-Control "public, max-age=31536000, immutable";
    }
    
    # Directory listing (optional)
    autoindex on;
    autoindex_exact_size off;
}
EOF

# Enable site
sudo ln -s /etc/nginx/sites-available/aurora-repo /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

### Option B: GitHub Pages (Free)

```bash
# Create gh-pages branch
git checkout --orphan gh-pages

# Copy repository to gh-pages
cp -r /var/www/aurora-repo/* .
git add .
git commit -m "Aurora APT repository"
git push origin gh-pages

# Repository now served at:
# https://github-username.github.io/aurora/dists/stable/

# For custom domain:
# 1. Create CNAME file with archive.aurora.linux
# 2. Add DNS CNAME record: archive.aurora.linux CNAME github-username.github.io
# 3. Enable HTTPS in GitHub Pages settings
```

### Option C: Cloudflare R2

```bash
# Upload to Cloudflare R2
aws configure set aws_access_key_id YOUR_KEY_ID
aws configure set aws_secret_access_key YOUR_SECRET_KEY
aws configure set default.s3.signature_version s3v4

# Upload repository
aws s3 sync /var/www/aurora-repo s3://aurora-repo/ \
  --endpoint-url https://YOUR_ACCOUNT.r2.cloudflaireapis.com

# Enable public access in R2 settings
# Create Cloudflare Worker to proxy
```

---

## Step 9: Test Installation (From User Perspective)

```bash
# Add repository
wget https://archive.aurora.linux/aurora-archive-keyring.gpg
sudo apt-key add aurora-archive-keyring.gpg

echo "deb https://archive.aurora.linux/dists/stable main" | \
  sudo tee /etc/apt/sources.list.d/aurora.sources

# Update + Install
sudo apt update
sudo apt install aurora

# Verify installation
dpkg -l | grep aurora
# aurora-themes         1.0.0   all  Aurora themes
# aurora-icons          1.0.0   all  Aurora icons
# aurora-fonts          1.0.0   all  Aurora fonts
# aurora-cursors        1.0.0   all  Aurora cursors
# aurora                1.0.0   all  Aurora meta-package
```

---

## Step 10: Automate Releases (GitHub Actions)

Create `.github/workflows/release.yml`:

```yaml
name: Release Aurora

on:
  push:
    tags:
      - 'v*'

jobs:
  build-and-publish:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Build packages
        run: |
          sudo apt-get install -y debhelper lintian dpkg-dev fakeroot
          ./scripts/build-all-packages.sh
      
      - name: Import GPG key
        run: |
          echo "${{ secrets.GPG_PRIVATE_KEY }}" | gpg --import
      
      - name: Publish to repository
        run: |
          ./scripts/publish-to-repo.sh "${{ secrets.REPO_URL }}" \
                                       "${{ secrets.GPG_KEY_ID }}"
      
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            *.deb
            dists/stable/Release
            dists/stable/InRelease
```

---

## Maintenance Commands

```bash
# Show repository contents
aptly repo show -with-packages aurora-stable

# List snapshots
aptly snapshot list

# Remove old snapshot
aptly snapshot drop aurora-testing-v0.9.0

# Re-publish repository (after manual changes)
aptly publish update stable filesystem:filesystem

# Verify packages
lintian -EviI aurora-themes_1.0.0_all.deb

# Check GPG signature
gpg --verify dists/stable/Release.gpg dists/stable/Release

# Test as user
apt-cache policy aurora
apt-cache show aurora

# Search packages
apt-cache search aurora

# Download package info
curl https://archive.aurora.linux/dists/stable/Release
curl https://archive.aurora.linux/dists/stable/main/binary-all/Packages.gz
```

---

## Troubleshooting

### Repository key not trusted
```bash
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 1234567890ABCDEF
sudo apt update
```

### "W: GPG error: signature invalid"
```bash
# Re-sign Release file
gpg --clearsign -o dists/stable/InRelease dists/stable/Release
```

### "W: Release file expired"
```bash
# Check Release file
cat dists/stable/Release | grep Valid-Until
# Add new expiration during publishing
aptly publish update -force-overwrite stable filesystem:filesystem
```

### Package not found after adding repository
```bash
# Verify repository file exists
ls -la /var/www/aurora-repo/dists/stable/main/binary-all/Packages*

# Rebuild Packages file
aptly repo show -with-packages aurora-stable

# Republish
aptly publish update stable filesystem:filesystem
```

---

**Complete! Your Aurora APT repository is now operational.**

For ongoing maintenance, see `docs/MAINTENANCE.md`.
