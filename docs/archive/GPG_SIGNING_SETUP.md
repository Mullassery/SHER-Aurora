# Aurora GPG Signing & Key Management

**Complete guide for generating, managing, and using Aurora's GPG repository keys.**

---

## Overview

Aurora uses GPG (GnuPG) to cryptographically sign packages and Release files. This ensures:
- ✅ Package authenticity (came from Aurora maintainers)
- ✅ Package integrity (not tampered with)
- ✅ Package authorization (trusted source)
- ✅ Supply chain security (end-to-end verification)

---

## Phase 5: GPG Key Generation (Week 5)

### Step 1: Generate Repository Key

This creates the master signing key (one time, 4-year validity):

```bash
gpg --full-generate-key

# When prompted:
# Key type: RSA and RSA (default)
# Key size: 4096 (maximum recommended)
# Validity: 4y (4 years)
# Real name: Aurora Linux Repository
# Email: aurora@example.com
# Comment: Aurora APT Repository Key
# Passphrase: [very strong password, store in 1Password/Vault]

# Output: 
# gpg: key 1234567890ABCDEF marked as ultimately trusted
# pub   rsa4096 2026-08-02 [SC] [expires: 2030-08-01]
#       1234567890ABCDEF1234567890ABCDEF12345678
# uid           [ultimate] Aurora Linux Repository <aurora@example.com>
# sub   rsa4096 2026-08-02 [E] [expires: 2030-08-01]
```

**Save the Key ID:** `1234567890ABCDEF1234567890ABCDEF12345678`

### Step 2: Export Public Key

```bash
# Export as ASCII (for distribution)
gpg --export -a aurora@example.com > aurora-archive-keyring.gpg

# Verify export
gpg --import aurora-archive-keyring.gpg

# Display key information
gpg --list-keys aurora@example.com
# pub   rsa4096 2026-08-02 [SC] [expires: 2030-08-01]
#       1234567890ABCDEF1234567890ABCDEF12345678
# uid           [ultimate] Aurora Linux Repository <aurora@example.com>

# Get fingerprint
gpg --fingerprint aurora@example.com
# Key fingerprint = 1234 5678 90AB CDEF 1234  5678 90AB CDEF 1234 5678
```

### Step 3: Back Up Private Key

**Critical:** Secure your private key offline!

```bash
# Export private key (encrypted with passphrase)
gpg --export-secret-keys -a aurora@example.com > aurora-private-key.asc

# Encrypt for extra security (openssl)
openssl enc -aes-256-cbc -in aurora-private-key.asc \
  -out aurora-private-key.asc.enc

# Store safely:
# - Print QR code of fingerprint
# - Store aurora-private-key.asc.enc on encrypted USB
# - Keep in secure location (safe, safety deposit box, etc.)
# - NEVER commit to Git repository

echo "Backup locations:"
echo "  1. Encrypted USB: aurora-private-key.asc.enc"
echo "  2. Cloud vault: 1Password, LastPass, etc."
echo "  3. Paper backup: Print fingerprint + QR code"

rm aurora-private-key.asc  # Delete unencrypted copy
```

### Step 4: Add Key to GitHub Secrets

Store the private key in GitHub so CI/CD can sign releases:

```bash
# Encode private key for GitHub (base64)
gpg --export-secret-keys -a aurora@example.com | base64 -w0 > /tmp/gpg-key.b64

# Add to GitHub via web UI:
# Repository Settings → Secrets → New repository secret
# Name: GPG_PRIVATE_KEY
# Value: [paste base64 content]

# Also add:
# Name: GPG_KEY_ID
# Value: 1234567890ABCDEF

# Name: GPG_KEY_PASSPHRASE
# Value: [your very strong passphrase]
```

### Step 5: Test Signing

```bash
# Create a test Release file
cat > /tmp/test-release <<'RELEASE'
Origin: Aurora Linux
Label: Aurora Stable Repository
Suite: stable
Codename: stable
Date: Fri, 02 Aug 2026 00:00:00 +0000
Valid-Until: Fri, 09 Aug 2026 00:00:00 +0000
Architectures: all
Components: main
Description: Aurora Linux Design System
MD5Sum:
 d41d8cd98f00b204e9800998ecf8427e                0 main/binary-all/Packages
SHA256:
 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855     0 main/binary-all/Packages
RELEASE

# Sign with detached signature
gpg --default-key aurora@example.com \
    --detach-sign --armor \
    -o /tmp/test-release.gpg \
    /tmp/test-release

# Create inline signature
gpg --default-key aurora@example.com \
    --clearsign --armor \
    -o /tmp/test-release.signed \
    /tmp/test-release

# Verify detached signature
gpg --verify /tmp/test-release.gpg /tmp/test-release
# Output: Good signature from "Aurora Linux Repository <aurora@example.com>"

# Verify inline signature
gpg --verify /tmp/test-release.signed
# Output: Good signature from "Aurora Linux Repository <aurora@example.com>"

echo "✅ GPG signing working correctly"
```

---

## Production Release Signing Workflow

### In GitHub Actions (.github/workflows/release.yml)

```yaml
- name: Import GPG key
  run: |
    echo "${{ secrets.GPG_PRIVATE_KEY }}" | base64 -d | gpg --import
    gpg --trust-model always --batch --yes --import-ownertrust \
      <(echo "${{ secrets.GPG_KEY_FINGERPRINT }}:6:")

- name: Sign Release files
  run: |
    cd dists/stable
    gpg --default-key "${{ secrets.GPG_KEY_ID }}" \
        --detach-sign --armor \
        -o Release.gpg Release
    gpg --default-key "${{ secrets.GPG_KEY_ID }}" \
        --clearsign --armor \
        --output InRelease Release
    gpg --verify InRelease
    cd - > /dev/null
```

### Manual Signing (if not using CI/CD)

```bash
# After publishing to repository:
cd repository/dists/stable

# Sign Release file
gpg --default-key aurora@example.com \
    --detach-sign --armor \
    -o Release.gpg Release

# Create inline signature
gpg --default-key aurora@example.com \
    --clearsign --armor \
    --output InRelease Release

# Verify both
gpg --verify Release.gpg Release
gpg --verify InRelease

# Also sign for testing and unstable
cd ../testing && gpg --clearsign -a --output InRelease Release
cd ../unstable && gpg --clearsign -a --output InRelease Release
```

---

## User Key Import Workflow

### Installation Script (One-Time)

Users run this once to import Aurora's public key:

```bash
#!/bin/bash
# Aurora Repository Installation Script

echo "🔑 Importing Aurora GPG key..."

# Download public key
wget https://archive.aurora.linux/aurora-archive-keyring.gpg -O /tmp/aurora-key.gpg

# Import to system keyring
sudo apt-key add /tmp/aurora-key.gpg

# Or (modern method, Ubuntu 20.04+):
sudo mkdir -p /usr/share/keyrings
sudo cp /tmp/aurora-key.gpg /usr/share/keyrings/aurora-archive-keyring.gpg

echo "✅ Key imported"

# Verify key fingerprint (optional but recommended)
# gpg --with-fingerprint /usr/share/keyrings/aurora-archive-keyring.gpg
```

### APT Verification

After key import, APT automatically verifies on `apt update`:

```bash
sudo apt update

# Output shows:
# Get:1 https://archive.aurora.linux/dists/stable Release
# Reading state information...
# (automatically verifies Release.gpg signature)
```

If signature fails:
```
E: GPG error: https://archive.aurora.linux/dists/stable Release: The following signatures couldn't be verified because the public key is not available: NO_PUBKEY 1234567890ABCDEF
```

User should re-import key:
```bash
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 1234567890ABCDEF
sudo apt update
```

---

## Key Rotation (2030-08-01)

When key expires (after 4 years):

### Step 1: Generate New Key
```bash
gpg --full-generate-key
# Same process as initial generation
```

### Step 2: Cross-Sign Keys
```bash
# Export both keys
gpg --export -a aurora@example.com > new-aurora-keyring.gpg

# Users import new key before old expires
```

### Step 3: Re-sign Releases
```bash
# Re-sign all Release files with new key
cd repository/dists/stable
gpg --default-key <NEW_KEY_ID> --clearsign -a -o InRelease Release
```

### Step 4: Publish New Key
```bash
# Update aurora-archive-keyring.gpg
# Update aurora repository package
# Notify users
```

### Step 5: Retire Old Key
```bash
# Generate revocation certificate (create NOW, use in future)
gpg --gen-revoke aurora@example.com > aurora-key-revocation.asc

# If key compromised, publish revocation:
gpg --import aurora-key-revocation.asc
gpg --send-keys --keyserver keyserver.ubuntu.com <OLD_KEY_ID>
```

---

## Maintaining aurora-archive-keyring Package

Create a package to distribute the public key:

```bash
mkdir -p packages/aurora-archive-keyring
cd packages/aurora-archive-keyring

# debian/control
cat > debian/control <<'CONTROL'
Package: aurora-archive-keyring
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Depends: debian-archive-keyring | ubuntu-keyring
Priority: standard
Description: Aurora repository signing key
 This package contains the GPG key to verify Aurora packages.
 .
 Install this package to automatically trust Aurora packages.
CONTROL

# debian/postinst
cat > debian/postinst <<'POSTINST'
#!/bin/bash
install -D -m 0644 keyrings/aurora.gpg \
  /usr/share/keyrings/aurora-archive-keyring.gpg
POSTINST

mkdir -p keyrings
cp /path/to/aurora-archive-keyring.gpg keyrings/aurora.gpg
```

Then users install with:
```bash
sudo apt install aurora-archive-keyring
# Automatically installs key to /usr/share/keyrings/
```

---

## Security Best Practices Checklist

- ✅ Use 4096-bit RSA (maximum)
- ✅ Set 4-year key validity (rotate periodically)
- ✅ Store private key securely (encrypted, offline backup)
- ✅ Never commit private key to Git
- ✅ Use strong passphrase (20+ characters)
- ✅ Require passphrase for GitHub Actions (via Secrets)
- ✅ Publish key fingerprint on website
- ✅ Use subkeys for signing (advanced)
- ✅ Keep backup of revocation certificate
- ✅ Rotate key every 2 years
- ✅ Test signing before production release

---

## Troubleshooting

### "gpg: error writing secring"
```bash
# Fix ownership
chmod 700 ~/.gnupg
chmod 600 ~/.gnupg/secring.gpg
```

### "Bad Signature" on Release file
```bash
# Re-sign Release file
gpg --clearsign -a -o InRelease Release
```

### "NO_PUBKEY" error in apt
```bash
# User needs to import key
wget https://archive.aurora.linux/aurora-archive-keyring.gpg
sudo apt-key add aurora-archive-keyring.gpg
sudo apt update
```

### "Signature not created"
```bash
# Verify key exists and is usable
gpg --list-secret-keys aurora@example.com

# Try with explicit key ID
gpg --default-key 1234567890ABCDEF --clearsign -a -o InRelease Release
```

---

## Phase 6 Integration

When Phase 6 (Release) begins:

```bash
# Tag new release
git tag -a v1.0.0 -m "Aurora 1.0.0"

# GitHub Actions automatically:
# 1. Builds packages
# 2. Imports GPG key from Secrets
# 3. Signs Release files
# 4. Creates signed InRelease
# 5. Publishes to repository
# 6. All packages now verified

# Users can verify:
gpg --verify https://archive.aurora.linux/dists/stable/InRelease
```

---

**GPG Setup Complete. Ready for Phase 6: Release**
