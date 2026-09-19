# Aurora Repository Hosting Setup

**Choose one hosting solution and configure following the appropriate guide below.**

---

## Hosting Options Quick Comparison

| Option | Cost | Bandwidth | Setup Time | Scalability | CDN |
|--------|------|-----------|-----------|-------------|-----|
| GitHub Pages | Free | 1GB soft limit | 10 min | 1-10k users | ✅ GitHub |
| Cloudflare R2 | ~$15/mo | Unlimited | 20 min | 100k+ users | ✅ Cloudflare |
| AWS S3+CF | ~$50-200/mo | Unlimited | 30 min | Unlimited | ✅ CloudFront |
| DigitalOcean Spaces | $6.50/mo | Limited | 15 min | 10-50k users | Limited |
| Self-hosted VPS | $20-50/mo | Limited | 45 min | 1-10k users | ❌ Manual |

**Recommendation for Phase 3-6:** GitHub Pages (simplest for v1.0 launch)  
**Recommendation for scale:** Cloudflare R2 (best price/performance)

---

## Option A: GitHub Pages (Recommended for Launch)

### Prerequisites
- ✅ GitHub repository (https://github.com/aurora-linux/aurora)
- ✅ `gh` CLI installed
- ✅ GitHub authentication configured

### Setup (10 minutes)

#### Step 1: Create gh-pages branch
```bash
cd ~/aurora
git checkout --orphan gh-pages
git reset --hard
echo "Aurora Repository" > index.html
git add index.html
git commit -m "Initialize GitHub Pages"
git push origin gh-pages
```

#### Step 2: Configure repository settings
```bash
# Via GitHub web UI:
# Settings → Pages → Source: Deploy from branch
# Branch: gh-pages, Directory: / (root)
# Click Save

# Or via CLI:
gh repo edit --enable-wiki=false
```

#### Step 3: Return to main branch
```bash
git checkout main
```

#### Step 4: Set up deployment script
```bash
cat > scripts/deploy-github-pages.sh <<'DEPLOY'
#!/bin/bash
# Deploy repository to GitHub Pages

set -e

REPO_SOURCE="$1"
VERSION="${2:-1.0.0}"

if [ ! -d "$REPO_SOURCE" ]; then
  echo "Error: Repository source not found: $REPO_SOURCE"
  exit 1
fi

echo "📤 Deploying to GitHub Pages..."

# Temporarily switch to gh-pages
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
git checkout gh-pages

# Copy repository content
cp -r "$REPO_SOURCE"/repository/dists .
cp -r "$REPO_SOURCE"/repository/pool .

# Commit and push
git add dists/ pool/
git commit -m "Deploy Aurora v$VERSION" || true
git push origin gh-pages

# Return to original branch
git checkout "$CURRENT_BRANCH"

echo "✅ Deployed to https://aurora-linux.github.io/repo/"
DEPLOY

chmod +x scripts/deploy-github-pages.sh
```

#### Step 5: Test
```bash
# After first deploy, verify:
curl https://aurora-linux.github.io/repo/dists/stable/Release
# Should return Release file
```

### Installation URL
```
https://aurora-linux.github.io/repo/dists/stable/
```

### Pros/Cons
✅ Free  
✅ Automatic HTTPS  
✅ GitHub-integrated  
❌ 1GB soft bandwidth limit  
❌ Throttled if exceeded  

---

## Option B: Cloudflare R2 (Recommended for Scale)

### Prerequisites
- ✅ Cloudflare account (free tier OK)
- ✅ AWS CLI installed (`apt install awscli`)

### Setup (20 minutes)

#### Step 1: Create R2 bucket
```bash
# Via Cloudflare dashboard:
# 1. Login to dashboard.cloudflare.com
# 2. R2 → Create Bucket
# 3. Name: aurora-repo
# 4. Create bucket
```

#### Step 2: Generate API token
```bash
# Cloudflare dashboard:
# Account → API Tokens → Create Token
# Template: "Edit Cloudflare R2"
# Add to limited scope
# Copy token and save securely
```

#### Step 3: Configure AWS CLI
```bash
cat > ~/.aws/config <<'AWS'
[profile cloudflare-r2]
service_name = s3
region = auto
endpoint_url = https://YOUR_ACCOUNT_ID.r2.cloudflaireapis.com
AWS

cat > ~/.aws/credentials <<'AWS'
[cloudflare-r2]
aws_access_key_id = YOUR_ACCESS_KEY_ID
aws_secret_access_key = YOUR_SECRET_ACCESS_KEY
AWS

chmod 600 ~/.aws/credentials
```

#### Step 4: Create deployment script
```bash
cat > scripts/deploy-cloudflare-r2.sh <<'DEPLOY'
#!/bin/bash
# Deploy repository to Cloudflare R2

set -e

REPO_SOURCE="$1"
BUCKET="${2:-aurora-repo}"
PROFILE="${3:-cloudflare-r2}"

if [ ! -d "$REPO_SOURCE" ]; then
  echo "Error: Repository source not found: $REPO_SOURCE"
  exit 1
fi

echo "📤 Deploying to Cloudflare R2..."

# Sync repository to R2
aws s3 sync "$REPO_SOURCE/repository/dists" \
  "s3://$BUCKET/dists" \
  --profile "$PROFILE" \
  --delete

aws s3 sync "$REPO_SOURCE/repository/pool" \
  "s3://$BUCKET/pool" \
  --profile "$PROFILE"

echo "✅ Deployed to R2 bucket: $BUCKET"
echo "   Accessible via Cloudflare Workers proxy"
DEPLOY

chmod +x scripts/deploy-cloudflare-r2.sh
```

#### Step 5: Create Cloudflare Worker
```javascript
// Cloudflare Worker to proxy R2 bucket
export default {
  async fetch(request) {
    const url = new URL(request.url);
    const objectKey = url.pathname.slice(1);
    
    return fetch(
      new Request(
        `https://aurora-repo.YOUR_ACCOUNT.r2.cloudflaireapis.com/${objectKey}`,
        request
      ),
      {
        cf: { cacheTtl: 3600 }
      }
    );
  }
};
```

#### Step 6: Set up custom domain
```bash
# DNS: Create CNAME record
# archive.aurora.linux CNAME aurora-repo.YOUR_ACCOUNT.r2.cloudflaireapis.com
```

### Installation URL
```
https://archive.aurora.linux/dists/stable/
```

### Pros/Cons
✅ Low cost (~$15/month)  
✅ Unlimited bandwidth  
✅ Global CDN  
✅ Cloudflare security  
❌ Requires Cloudflare account  
❌ Slightly more setup  

---

## Option C: AWS S3 + CloudFront (Enterprise)

### Prerequisites
- ✅ AWS account (free tier available)
- ✅ AWS CLI installed

### Setup (30 minutes)

#### Step 1: Create S3 bucket
```bash
aws s3 mb s3://aurora-repo --region us-east-1
aws s3api put-bucket-versioning \
  --bucket aurora-repo \
  --versioning-configuration Status=Enabled
```

#### Step 2: Create CloudFront distribution
```bash
# Via AWS console:
# CloudFront → Create distribution
# Origin: S3 bucket aurora-repo
# Origin access: OAC (Origin Access Control)
# Default root object: dists/stable/Release
# Viewer protocol: HTTPS only
# Cache TTL: dists/ = 1 hour, pool/ = 1 year
```

#### Step 3: Create deployment script
```bash
cat > scripts/deploy-aws-s3.sh <<'DEPLOY'
#!/bin/bash
# Deploy repository to AWS S3 + CloudFront

set -e

REPO_SOURCE="$1"
BUCKET="${2:-aurora-repo}"
DISTRO_ID="${3:-YOUR_DISTRO_ID}"

echo "📤 Deploying to AWS S3..."

# Sync with cache control headers
aws s3 sync "$REPO_SOURCE/repository/dists" \
  "s3://$BUCKET/dists" \
  --cache-control "public, max-age=3600" \
  --delete

aws s3 sync "$REPO_SOURCE/repository/pool" \
  "s3://$BUCKET/pool" \
  --cache-control "public, max-age=31536000, immutable"

# Invalidate CloudFront
if [ -n "$DISTRO_ID" ]; then
  aws cloudfront create-invalidation \
    --distribution-id "$DISTRO_ID" \
    --paths "/*"
  echo "✅ CloudFront cache invalidated"
fi

echo "✅ Deployed to S3: s3://$BUCKET/"
DEPLOY

chmod +x scripts/deploy-aws-s3.sh
```

### Installation URL
```
https://archive.aurora.linux/dists/stable/
```

### Pros/Cons
✅ Industry standard  
✅ Unlimited bandwidth  
✅ Global CDN (CloudFront)  
✅ Enterprise features  
❌ Higher cost (~$50-200/month)  
❌ More complex setup  

---

## Quick Start (GitHub Pages)

For fastest launch in Phase 3-6:

```bash
# 1. Create gh-pages branch
git checkout --orphan gh-pages
git reset --hard
echo "Aurora APT Repository" > index.html
git add index.html
git commit -m "Initialize GitHub Pages"
git push origin gh-pages
git checkout main

# 2. Build packages
make build

# 3. Initialize repository
./scripts/setup-repository.sh

# 4. Deploy to GitHub Pages
mkdir -p repository/dists repository/pool
cp -r ~/aurora/repository/* repository/

git checkout gh-pages
git add -A
git commit -m "Add Aurora repository"
git push origin gh-pages
git checkout main

# 5. Users install from:
# https://aurora-linux.github.io/repo/dists/stable/
```

---

## Post-Hosting Checklist

After hosting is set up:

- [ ] Domain accessible (HTTPS)
- [ ] Repository metadata downloadable
  - [ ] dists/stable/Release accessible
  - [ ] dists/stable/InRelease accessible
  - [ ] Packages.gz downloadable
- [ ] TLS certificate valid
- [ ] CDN cache working (test with curl -I)
- [ ] Installation script works
  ```bash
  curl https://get.aurora.linux | bash
  ```
- [ ] Users can run `apt update` successfully
- [ ] Users can install packages

---

## Monitoring & Maintenance

### Health Check
```bash
# Test repository is accessible
curl -I https://archive.aurora.linux/dists/stable/Release

# Test package download
curl -O https://archive.aurora.linux/pool/main/a/aurora/aurora_1.0.0_all.deb

# Verify signature
gpg --verify Release.gpg Release
```

### Cache Invalidation
```bash
# After publishing new packages:
# GitHub Pages: Automatic (git push)
# Cloudflare: Automatic (via Worker cache)
# AWS S3: Manual invalidation or TTL expiry
```

### Monitoring
- GitHub Pages: Automatic GitHub Actions logs
- Cloudflare: R2 analytics dashboard
- AWS S3: CloudWatch metrics

---

## Phase 6 Integration

When you reach Phase 6 (Release), hosting should be:
- ✅ Configured and tested
- ✅ Domain pointing to repository
- ✅ TLS certificate valid
- ✅ CDN caching working
- ✅ Installation script deployed
- ✅ Users can access and install

Then:
```bash
git tag v1.0.0
git push origin v1.0.0
# GitHub Actions automatically:
# 1. Builds packages
# 2. Signs with GPG
# 3. Publishes to repository
# 4. Deploys to hosting
# 5. Creates release notes
```

---

**Choose your hosting solution and proceed to Phase 5: GPG Signing Setup**
