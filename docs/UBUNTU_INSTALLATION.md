# Aurora v1.0.0: Ubuntu Installation Guide

**Complete guide for Ubuntu users to install and use Aurora**

---

## Quick Start (Ubuntu 22.04 LTS / 23.10 / 24.04)

Copy and paste this one command:

```bash
# Install everything Aurora needs
sudo apt update && sudo apt install -y libgtk-4-dev libadwaita-1-dev libglib2.0-dev build-essential pkg-config rustc cargo && \
git clone https://github.com/Mullassery/aurora.git && \
cd aurora && \
sudo cp crates/aurora-gtk/schemas/org.gnome.desktop.interface.aurora.gschema.xml /usr/share/glib-2.0/schemas/ && \
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/ && \
echo "✅ Aurora installed! Run: cargo run --example aurora_settings"
```

Done! ✨ You now have Aurora v1.0.0 ready to use.

---

## Ubuntu Version Compatibility

| Ubuntu Version | Support | Notes |
|---|---|---|
| **24.04 LTS** | ✅ Fully Supported | Latest, latest packages |
| **23.10** | ✅ Fully Supported | Current, most packages |
| **23.04** | ✅ Supported | Older but works |
| **22.04 LTS** | ✅ Supported | May need backports for newer GTK4 |
| **20.04 LTS** | ⚠️ Limited | GTK4 backport needed |
| **18.04 LTS** | ❌ Not Supported | Too old, GTK4 not available |

**Recommended:** Ubuntu 24.04 LTS (latest, longest support, newest GTK4)

---

## Step-by-Step Installation

### Step 1: Open Terminal

Press `Ctrl + Alt + T` to open a terminal, or search for "Terminal" in Activities.

### Step 2: Install System Dependencies

Copy and paste this entire block:

```bash
sudo apt update
```

This updates Ubuntu's package list (tells it what versions are available).

Then install everything:

```bash
sudo apt install -y \
    libgtk-4-dev \
    libadwaita-1-dev \
    libglib2.0-dev \
    build-essential \
    pkg-config \
    rustc \
    cargo
```

**What gets installed:**

| Package | Purpose | Size |
|---------|---------|------|
| `libgtk-4-dev` | Create windows, buttons, UI elements | ~50MB |
| `libadwaita-1-dev` | GNOME modern components (rounded buttons, dark mode) | ~30MB |
| `libglib2.0-dev` | Core system library (required by GTK4) | ~10MB |
| `build-essential` | Compiler (gcc), linker, make (compiles code) | ~200MB |
| `pkg-config` | Finds libraries on your system | ~1MB |
| `rustc` | Rust compiler (compiles Rust code) | ~500MB |
| `cargo` | Rust package manager (downloads dependencies) | Included with rustc |

**Total download:** ~1GB (first time only)

### Step 3: Download Aurora

```bash
git clone https://github.com/Mullassery/aurora.git
cd aurora
```

This downloads Aurora source code (~100MB).

### Step 4: Register Aurora Settings

Aurora stores preferences in GNOME's settings. You need to register the settings schema:

```bash
# Copy Aurora's settings definition to GNOME
sudo cp crates/aurora-gtk/schemas/org.gnome.desktop.interface.aurora.gschema.xml \
    /usr/share/glib-2.0/schemas/
```

Then compile it (GNOME needs binary format):

```bash
# Compile the schema
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/
```

Verify it worked:

```bash
# Check that Aurora appears in GNOME settings
gsettings list-schemas | grep aurora
```

You should see: `org.gnome.desktop.interface.aurora` ✅

### Step 5: Test Aurora

Run an example app to verify everything works:

```bash
# Navigate to Aurora folder (if not already there)
cd ~/aurora

# Run Aurora Settings app
cargo run --example aurora_settings
```

**What should happen:**
1. Cargo downloads dependencies (~300MB, first time only)
2. Compiles Aurora (~2-3 minutes)
3. Aurora Settings app launches
4. You see a beautiful GNOME app with theme, sound, and accessibility settings

🎉 **Success!** Aurora is working!

---

## Creating Your First Aurora App

### Method 1: Use crates.io (Recommended)

Create a new project:

```bash
cargo new my-aurora-app
cd my-aurora-app
```

Edit `Cargo.toml` and add Aurora dependencies:

```toml
[package]
name = "my-aurora-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# Aurora design system
aurora-gtk = "1.0"
aurora-color = "1.0"
aurora-tokens = "1.0"
aurora-motion = "1.0"
aurora-sound = "1.0"

# GNOME ecosystem
gtk4 = { version = "0.9", features = ["v4_10"] }
libadwaita = "0.5"
glib = "0.19"
```

Edit `src/main.rs`:

```rust
use gtk4::{Application, ApplicationWindow};
use gtk4::prelude::*;
use aurora_gtk::AuroraGtk;

fn main() {
    let app = Application::builder()
        .application_id("com.example.myapp")
        .build();

    app.connect_activate(|app| {
        // Initialize Aurora
        let _aurora = AuroraGtk::new(aurora_gtk::Theme::Light)
            .expect("Aurora initialization failed");

        // Create window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("My Aurora App")
            .default_width(400)
            .default_height(300)
            .build();

        window.present();
    });

    app.run();
}
```

Run it:

```bash
cargo run --release
```

### Method 2: Use Source Code (Development)

If you want to modify Aurora itself:

```bash
# In your new app's Cargo.toml, use path dependencies:
[dependencies]
aurora-gtk = { path = "../aurora/crates/aurora-gtk" }
aurora-color = { path = "../aurora/crates/aurora-color" }
# etc...
```

This lets you make changes to Aurora and see them immediately.

---

## Run Example Applications

Aurora includes 4 complete example applications:

```bash
# Navigate to Aurora folder
cd ~/aurora

# Run Aurora Settings (preferences app)
cargo run --example aurora_settings

# Run Aurora Files (file browser)
cargo run --example aurora_files

# Run Aurora Calendar (event manager)
cargo run --example aurora_calendar

# Run Aurora Music (media player)
cargo run --example aurora_music
```

Each example shows different Aurora components and patterns you can use.

---

## Ubuntu-Specific Tips

### Tip 1: Using GNOME Settings with Aurora

After installing, open GNOME Settings (Activities → Settings):

1. Go to **Appearance**
2. Change **Style** to Light/Dark/OLED
3. Aurora apps will update automatically! ✨

### Tip 2: Enable Ubuntu Minimal Installation Features

If you're on a minimal Ubuntu install, you might need:

```bash
# For GUI desktop environment
sudo apt install -y gnome-shell gnome-shell-extensions

# For Activities overview
sudo apt install -y gnome-shell-extensions-dash-to-panel
```

### Tip 3: Speed Up Compilation

Aurora compiles can be slow on first run. Speed it up:

```bash
# Use multiple CPU cores for compilation
export CARGO_BUILD_JOBS=$(nproc)

# Then compile
cargo build --release
```

This uses all your CPU cores, dramatically speeding up builds.

### Tip 4: Clean Up Old Builds

If you run out of disk space:

```bash
# Delete build artifacts (safe)
cargo clean

# This removes: ~/.cargo/registry/cache/ downloads
rm -rf ~/.cargo/registry/cache/
```

This frees up 1-2GB.

### Tip 5: Update Aurora Regularly

Keep Aurora up to date:

```bash
cd ~/aurora
git pull origin main
cargo build --release
```

This gets the latest features, bug fixes, and performance improvements.

---

## Troubleshooting (Ubuntu)

### Problem: "E: Unable to locate package libgtk-4-dev"

**Cause:** Your package list is outdated

**Solution:**
```bash
sudo apt update
sudo apt install libgtk-4-dev
```

### Problem: "error: linker `cc` not found"

**Cause:** Build tools not installed

**Solution:**
```bash
sudo apt install build-essential
```

### Problem: "error: failed to resolve: use of undeclared type `gtk4`"

**Cause:** GTK4 development files not installed

**Solution:**
```bash
sudo apt install libgtk-4-dev pkg-config
```

### Problem: "gsettings list-schemas" doesn't show Aurora

**Cause:** Schema not registered properly

**Solution:**
```bash
# Reregister the schema
sudo cp crates/aurora-gtk/schemas/org.gnome.desktop.interface.aurora.gschema.xml \
    /usr/share/glib-2.0/schemas/

# Recompile (this is important!)
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/

# Verify
gsettings list-schemas | grep aurora
```

### Problem: Application won't start / crashes immediately

**Cause:** Ubuntu version too old or missing libraries

**Solution:**
```bash
# Check Ubuntu version
lsb_release -a

# If older than 22.04, upgrade:
sudo do-release-upgrade

# Or reinstall dependencies
sudo apt install --reinstall libgtk-4-0 libadwaita-1-0 libglib2.0-0
```

### Problem: "Permission denied" when running cargo

**Cause:** Usually not a real permission issue, but path problem

**Solution:**
```bash
# Make sure you're in the right directory
pwd  # Should show: /home/username/aurora

# If not, navigate there
cd ~/aurora

# Then try again
cargo run --example aurora_settings
```

---

## Advanced: Building a Snap Package (Optional)

For distribution on Ubuntu's Snap Store:

Create `snap/snapcraft.yaml`:

```yaml
name: aurora-example
version: '1.0.0'
summary: Example Aurora application
description: Beautiful GNOME app using Aurora design system

grade: stable
confinement: strict

apps:
  aurora-example:
    command: bin/aurora-example
    plugs:
      - home
      - network

parts:
  aurora-example:
    plugin: rust
    source: .
    build-packages:
      - libgtk-4-dev
      - libadwaita-1-dev
      - libglib2.0-dev
      - pkg-config
```

Build it:

```bash
sudo snap install snapcraft --classic
snapcraft
```

### Building a .deb Package (Optional)

For Ubuntu package distribution:

```bash
# Install cargo-deb
cargo install cargo-deb

# Build .deb
cargo deb

# Install locally
sudo dpkg -i target/debian/my-aurora-app*.deb
```

---

## Performance Tips

### Tip 1: Build in Release Mode

```bash
# Development (fast compile, slow runtime)
cargo run

# Production (slow compile, fast runtime) - use this!
cargo run --release
```

Release mode is 10-20x faster for Aurora apps.

### Tip 2: Check Your System

Aurora runs smoothly on:
- **CPU:** Intel i5 or better, AMD Ryzen 3 or better
- **RAM:** 4GB minimum, 8GB recommended
- **GPU:** Any modern GPU (integrated is fine)
- **Storage:** 10GB free space minimum

Check your specs:

```bash
# CPU info
lscpu

# RAM info
free -h

# Disk space
df -h
```

### Tip 3: Update Drivers

For best performance, update graphics drivers:

```bash
# For Intel GPU
sudo apt install intel-media-driver

# For AMD GPU
sudo apt install mesa-vulkan-drivers

# For NVIDIA GPU
sudo apt install nvidia-driver-latest-dkms

# Then reboot
sudo reboot
```

---

## Ubuntu Desktop Environments Supported

Aurora works with:

| Desktop | Support | Notes |
|---------|---------|-------|
| **GNOME** | ✅ Full | Official support, best experience |
| **Ubuntu** (with GNOME) | ✅ Full | Default, recommended |
| **Kubuntu** (KDE) | ⚠️ Partial | GTK4 works, some features limited |
| **Xubuntu** (XFCE) | ⚠️ Partial | GTK4 works, minimal GNOME integration |
| **Lubuntu** (LXQt) | ⚠️ Partial | GTK4 works, no GNOME integration |

**Recommendation:** Use standard Ubuntu with GNOME for best Aurora experience.

---

## Getting Help (Ubuntu-Specific)

### Ubuntu-Specific Issues

Ask on:
- **Ubuntu Forums:** https://ubuntuforums.org/
- **Ask Ubuntu:** https://askubuntu.com/ (tag: aurora)
- **Ubuntu Subreddit:** https://reddit.com/r/Ubuntu/

### Aurora Issues

Ask on:
- **GitHub Issues:** https://github.com/Mullassery/aurora/issues
- **GitHub Discussions:** https://github.com/Mullassery/aurora/discussions
- **Email:** mullassery@gmail.com

### Include This Info

When asking for help:

```bash
# Ubuntu version
lsb_release -a

# GTK4 version
dpkg -l | grep gtk

# Rust version
rustc --version
cargo --version

# Error message
# (copy the full error text)
```

---

## Ubuntu Community Contribution

Want to help Aurora adoption on Ubuntu?

1. **Report bugs:** https://github.com/Mullassery/aurora/issues
2. **Suggest features:** https://github.com/Mullassery/aurora/discussions
3. **Write tutorials:** Blog posts, YouTube videos
4. **Create Ubuntu packages:** Help distribute via snap/deb
5. **Port GNOME apps:** Use Aurora in Files, Settings, etc.

---

## What's Next?

Now that you have Aurora installed:

1. **Read the docs:** https://github.com/Mullassery/aurora/blob/main/docs/INTEGRATION_GUIDE.md
2. **Explore examples:** `cargo run --example aurora_settings`
3. **Create your app:** Build something amazing!
4. **Join community:** Star the repo, share your projects

---

## Resources

- **GitHub:** https://github.com/Mullassery/aurora
- **API Reference:** `docs/API_REFERENCE.md`
- **Integration Guide:** `docs/INTEGRATION_GUIDE.md`
- **Architecture:** `docs/ARCHITECTURE_V2.md`
- **Roadmap:** `PHASE5_ROADMAP.md`

---

**Made for Ubuntu users. Made with ❤️ for GNOME.**

Let's make GNOME the most beautiful desktop on Linux! 🚀

---

**Last Updated:** August 1, 2026  
**Aurora Version:** v1.0.0  
**Ubuntu Tested:** 22.04 LTS, 23.10, 24.04 LTS
