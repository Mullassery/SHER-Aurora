# Installation Guide

Aurora is a Rust workspace (design tokens, typography, color, motion, sound,
accessibility, and a GTK4 widget layer) — not a Python package. An earlier
version of this file described a nonexistent `pip install aurora` flow left
over from an unrelated template; it never applied to this project and has
been replaced with what's actually here.

## Requirements

- Rust 1.70+ (install via [rustup](https://rustup.rs) if you don't have it:
  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- System GTK4 (>= 4.12) development libraries, needed because `aurora-gtk`
  links against the real `gtk4` crate

## Install system GTK4

```bash
# macOS
brew install gtk4

# Ubuntu / Debian
sudo apt update && sudo apt install -y libgtk-4-dev libglib2.0-dev build-essential pkg-config

# Fedora
sudo dnf install -y gtk4-devel glib2-devel gcc make pkg-config

# Arch
sudo pacman -S gtk4 glib2 base-devel
```

## Use Aurora in your own project

Aurora is not published to crates.io (see the License section of the root
[README.md](../README.md)). Add it as a git dependency:

```toml
[dependencies]
aurora-gtk = { git = "https://github.com/Mullassery/aurora", tag = "v1.2.0" }
gtk4 = { version = "0.11", features = ["v4_12"] }
glib = "0.22"
```

## Build and test this repository

```bash
git clone https://github.com/Mullassery/aurora.git
cd aurora
cargo build --workspace
cargo test --workspace
cargo run --example gtk4_harness -p aurora-gtk   # non-interactive proof of real GTK4 rendering
cargo run --example showcase -p aurora-gtk       # real, interactive GTK4 window
```

## Troubleshooting

### `pkg-config` can't find `gtk4` / build fails linking against GTK4
Confirm the system library is actually installed and on `PKG_CONFIG_PATH`:
```bash
pkg-config --modversion gtk4
```
If that fails, revisit the "Install system GTK4" step above for your platform.

### macOS: widget-construction tests don't run
GTK4's Cocoa backend requires `gtk4::init()` to run on the process's real OS
main thread, which Rust's `#[test]` harness never provides on macOS. Those
tests are `#[cfg(not(target_os = "macos"))]` for that reason and run for
real on Linux CI; on macOS, use `cargo run --example gtk4_harness -p
aurora-gtk` instead — it runs as a plain `main()`, which is the real main
thread, and asserts real GTK4 state the same way the tests do on Linux.

## Next steps

- [README.md](../README.md) — what's real today, quick start, widget list
- [CHANGELOG.md](../CHANGELOG.md) — detailed, corrected release history
