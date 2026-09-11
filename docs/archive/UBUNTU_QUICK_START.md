# 🐧 Aurora on Ubuntu: Quick Reference

**TL;DR - Copy & Paste One Command**

```bash
sudo apt update && sudo apt install -y libgtk-4-dev libadwaita-1-dev libglib2.0-dev build-essential pkg-config rustc cargo && git clone https://github.com/Mullassery/aurora.git && cd aurora && sudo cp crates/aurora-gtk/schemas/org.gnome.desktop.interface.aurora.gschema.xml /usr/share/glib-2.0/schemas/ && sudo glib-compile-schemas /usr/share/glib-2.0/schemas/ && echo "✅ Aurora Ready!" && cargo run --example aurora_settings
```

---

## Quick Commands

| What | Command |
|-----|---------|
| **Install** | See above one-liner |
| **Run Settings App** | `cd aurora && cargo run --example aurora_settings` |
| **Run File Browser** | `cd aurora && cargo run --example aurora_files` |
| **Run Calendar** | `cd aurora && cargo run --example aurora_calendar` |
| **Run Music Player** | `cd aurora && cargo run --example aurora_music` |
| **Create New App** | `cargo new my-app && cd my-app` (then add dependencies) |
| **Build for Release** | `cargo build --release` |
| **Update Aurora** | `cd aurora && git pull && cargo build` |
| **Run Tests** | `cd aurora && cargo test --lib` |
| **View Documentation** | `cd aurora && cargo doc --open` |

---

## Installation Checklist

- [ ] Open Terminal (`Ctrl + Alt + T`)
- [ ] Run the one-liner above
- [ ] Wait for installation (5-10 minutes)
- [ ] See "✅ Aurora Ready!"
- [ ] Aurora Settings app opens automatically
- [ ] Close it and you're done!

---

## Ubuntu Versions

✅ **24.04 LTS** (Recommended - Latest)  
✅ **23.10** (Current)  
✅ **23.04**  
✅ **22.04 LTS** (May need backports)  
⚠️ **20.04 LTS** (Older, less ideal)  
❌ **18.04 LTS and older** (Not supported)

---

## If Something Breaks

```bash
# Fix broken installation
sudo apt update
sudo apt install --reinstall libgtk-4-dev libadwaita-1-dev libglib2.0-dev build-essential pkg-config rustc cargo

# Reregister Aurora settings
sudo cp crates/aurora-gtk/schemas/org.gnome.desktop.interface.aurora.gschema.xml /usr/share/glib-2.0/schemas/
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/

# Try again
cd aurora && cargo run --example aurora_settings
```

---

## Need Help?

- **Installation issues?** → [Full Ubuntu Guide](docs/UBUNTU_INSTALLATION.md)
- **How to use Aurora?** → [Integration Guide](docs/INTEGRATION_GUIDE.md)
- **API documentation?** → [API Reference](docs/API_REFERENCE.md)
- **Report bugs** → [GitHub Issues](https://github.com/Mullassery/aurora/issues)
- **Questions?** → [GitHub Discussions](https://github.com/Mullassery/aurora/discussions)

---

## One-Liner Breakdown

```bash
# 1. Update package list
sudo apt update

# 2. Install dependencies
sudo apt install -y libgtk-4-dev libadwaita-1-dev libglib2.0-dev build-essential pkg-config rustc cargo

# 3. Download Aurora
git clone https://github.com/Mullassery/aurora.git

# 4. Enter directory
cd aurora

# 5. Register settings with GNOME
sudo cp crates/aurora-gtk/schemas/org.gnome.desktop.interface.aurora.gschema.xml /usr/share/glib-2.0/schemas/

# 6. Compile settings
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/

# 7. Confirmation & run
echo "✅ Aurora Ready!" && cargo run --example aurora_settings
```

---

**What's Installed?**

| Package | What | Size |
|---------|------|------|
| libgtk-4-dev | GUI toolkit | 50MB |
| libadwaita-1-dev | GNOME widgets | 30MB |
| libglib2.0-dev | Core library | 10MB |
| build-essential | Compiler | 200MB |
| rustc + cargo | Rust toolchain | 500MB |
| Aurora source | Design system | 100MB |
| **Total** | | ~1GB |

---

**Recommended Ubuntu Specs**

- CPU: Intel i5 or AMD Ryzen 3 (or newer)
- RAM: 4GB (8GB+ recommended)
- Disk: 10GB free
- GPU: Any (integrated fine)

---

**Next Steps**

1. ✅ Install (run one-liner above)
2. 🚀 Try examples (`cargo run --example aurora_settings`)
3. 📖 Read [Integration Guide](docs/INTEGRATION_GUIDE.md)
4. 🛠️ Build your app (see guide)
5. 🌟 Star the repo & share!

---

Made for Ubuntu. Made for GNOME. Made for Linux. 💚

[Full Installation Guide](docs/UBUNTU_INSTALLATION.md) | [GitHub](https://github.com/Mullassery/aurora) | [API Docs](docs/API_REFERENCE.md)
