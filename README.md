# Aurora Desktop Design System

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)

The most polished open desktop experience ever built.

**Aurora** is a next-generation, open-source desktop design system for Linux that achieves macOS-level visual polish, consistency, and user experience while remaining platform-neutral and compatible with GNOME, GTK4, Qt6, Electron, web applications, and future desktop environments.

This is **not a theme**. Aurora is a complete design language and rendering ecosystem that defines how modern Linux applications should look, feel, animate, sound, and interact.

## Vision

Aurora becomes for Linux what Human Interface Guidelines are for Apple and Material Design is for Google: a unified, modern design system that enables truly professional, polished desktop experiences.

### Core Principles

- **Consistency over customization** — Design systems define and enforce consistency
- **Design systems over themes** — Tokens and semantic abstractions, not skin layers
- **Motion over decoration** — Every animation clarifies interaction
- **Typography over visual effects** — Text is the primary interface
- **Accessibility over aesthetics** — WCAG AAA first
- **Performance over complexity** — <100ms startup, 60fps animations
- **Native integration over hacks** — Wayland-native, GNOME/KDE/Cosmic compatible

## Architecture

```
Aurora Core (Design Tokens, Typography, Color, Motion, Icons, Sound, A11y)
         ↓
Unified Abstraction Layer (aurora-core)
         ↓
Platform Renderers (GTK4, Qt6, Web/WASM, Electron)
         ↓
Applications (GNOME, KDE, Cosmic, Web, Electron apps)
```

### Crates

| Crate | Purpose | Status |
|-------|---------|--------|
| `aurora-tokens` | Design token definitions and resolution | 🟢 Active |
| `aurora-typography` | Typography engine, font loading, responsive scales | 🟡 Planning |
| `aurora-color` | Color system, themes, semantic tokens | 🟡 Planning |
| `aurora-motion` | Animation engine, spring physics, easing | 🟡 Planning |
| `aurora-icons` | Icon system (1000+ icons) | 🟡 Planning |
| `aurora-sound` | Sound design system | 🟡 Planning |
| `aurora-a11y` | Accessibility layer (WCAG AAA) | 🟡 Planning |
| `aurora-core` | Unified API over all subsystems | 🟡 Planning |
| `aurora-gtk` | GTK4 renderer | 🟡 Planning |
| `aurora-qt` | Qt6 renderer | 🟡 Planning |
| `aurora-web` | Web/WASM renderer | 🟡 Planning |

## Getting Started

### Build the Project

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Generate Documentation

```bash
cargo doc --open
```

## Design System Documentation

Read the full design language and architecture guides:

- **[CLAUDE.md](CLAUDE.md)** — Project vision and philosophy
- **[docs/DESIGN_LANGUAGE.md](docs/DESIGN_LANGUAGE.md)** — Visual language, typography, colors, motion
- **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)** — Technical architecture and design decisions
- **[docs/ROADMAP.md](docs/ROADMAP.md)** — Phase-by-phase development roadmap

## Development Phases

### Phase 1: Foundation (8–12 weeks)
Design language spec, tokens, typography system, color system, motion language.

### Phase 2: Core Renderers (12–16 weeks)
GTK4 and Qt6 renderers, complete icon system.

### Phase 3: SDK & Web (10–14 weeks)
Electron, Web/WASM, React/Vue/Svelte SDKs.

### Phase 4: Integration (10–12 weeks)
Desktop shell integration, accessibility suite, sound system.

### Phase 5: Intelligence (12+ weeks)
AI personalization, HDR/OLED optimization, ecosystem adoption.

## Success Criteria

Aurora succeeds when:

1. **Visual Indistinguishability** — GTK and Qt apps look identical
2. **User Perception** — Desktop feels polished, calm, and premium
3. **Developer Adoption** — Easy to build apps with Aurora
4. **Accessibility Excellence** — WCAG AAA compliance throughout
5. **Stability** — Design language stable for years
6. **Community Impact** — Linux gains a unified design system

## Contributing

Aurora is open source. Contributions welcome.

Please read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for technical guidelines.

## License

Dual-licensed under MIT and Apache 2.0.

## Contact

- **GitHub**: [@Mullassery](https://github.com/Mullassery)
- **Email**: mullassery@gmail.com

---

**Built with Rust. Designed for Linux. Inspired by macOS. Open for everyone.**
