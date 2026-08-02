# Aurora: GNOME Enhancement Design System

**The most polished GNOME experience ever built.**

Aurora is an open-source design system and visual enhancement layer for GNOME that delivers professional-grade visual polish, consistency, motion, and accessibility while being deeply integrated with GNOME's existing infrastructure (GTK4, libadwaita, dconf, GNOME Settings).

## Philosophy

**Not a theme. A design language layer on top of GNOME.**

Aurora enhances GNOME by defining a unified design language, typography system, motion language, color system, and accessibility framework. The goal: make GNOME the most beautiful, polished, and professional desktop environment on Linux while preserving Linux's openness.

### Core Principles

- **GNOME-native integration** — Deep integration with GNOME, not platform-agnostic design
- **Consistency over customization** — All GNOME apps follow the same design language
- **Design systems over themes** — Tokens and semantic abstractions, not cosmetic themes
- **Motion over decoration** — Every animation clarifies interaction and feedback
- **Typography over visual effects** — Text is the primary interface; make it exceptional
- **Accessibility over aesthetics** — WCAG AAA compliance by default, not an afterthought
- **Polish over complexity** — Visual excellence over feature-richness
- **libadwaita integration** — Build on GNOME's modern toolkit, not around it

### Anti-Patterns

- Ignoring libadwaita or fighting GNOME's design patterns
- Excessive customization that breaks GNOME integration
- Visual effects without functional purpose
- Inconsistent widget styling within GNOME apps
- Hardcoded color values (all derive from semantic tokens)

## Architecture

```
GNOME Applications (Nautilus, Settings, Calendar, Music, Gedit, etc.)
        ↓
Aurora Enhancement Layer
├── Design Tokens (aurora-tokens)
│   └── Spacing, radius, elevation, motion, semantic colors
├── Typography Engine (aurora-typography)
│   └── Responsive scales, i18n, optical sizing
├── Color System (aurora-color)
│   └── Light, Dark, OLED themes with semantic tokens
├── Motion Engine (aurora-motion)
│   └── Spring physics, gesture tracking, animations
├── Icon System (aurora-icons)
│   └── GNOME icon set (1000+ system and app icons)
├── Sound System (aurora-sound)
│   └── Semantic notification and interaction sounds
├── Accessibility Layer (aurora-a11y)
│   └── WCAG AAA, high contrast, screen readers
├── Component Library (aurora-gtk)
│   └── GTK4 widgets and patterns built on libadwaita
└── GNOME Integration
    ├── Settings panel (GNOME Settings, dconf)
    ├── Shell theming (GTK theme, GNOME Shell CSS)
    ├── GDM integration (login screen theming)
    └── Notification system (GNOME Notification Daemon)
        ↓
GTK4 + libadwaita (GNOME's modern toolkit)
        ↓
Wayland Compositor (GNOME Shell)
```

## Workspace Structure

```
crates/
├── aurora-tokens/       # Design token definitions and codegen
├── aurora-typography/   # Typography engine with variable fonts
├── aurora-color/        # Color system, semantic tokens, themes
├── aurora-motion/       # Animation engine with spring physics
├── aurora-icons/        # Icon system and font generation
├── aurora-sound/        # Sound design definitions
├── aurora-a11y/         # Accessibility layer
├── aurora-core/         # Unified API over all subsystems
└── aurora-gtk/          # GTK4/libadwaita theme engine and components
```

## Key Design Decisions

1. **Token-Driven Everything** — No hardcoded values. Every spacing, color, motion value comes from tokens. GNOME apps consume semantic tokens only (e.g., `surface`, `primary`, not `#f0f0f0`).

2. **Deep libadwaita Integration** — Build on GNOME's modern toolkit, not around it. Leverage GTK4 + libadwaita for consistency and maintenance.

3. **GNOME-Native Only** — Focus exclusively on GNOME. No multi-desktop abstraction. Deep integration with GNOME Shell, Settings, dconf, and notifications.

4. **GNOME Application Consistency** — All GNOME apps (Files, Settings, Calendar, Music, etc.) follow the same design language. Achieved through:
   - Shared design tokens
   - Unified typography scales
   - Consistent motion language
   - Semantic color system
   - Standard component library

5. **Performance Targets**
   - Design token resolution: <1ms
   - Animation smoothness: 60fps minimum, 120fps preferred
   - Window animations: Fluid, responsive
   - Memory overhead: Minimal (no extra system load)

6. **Accessibility as First-Class** — WCAG AAA by default, not an afterthought:
   - High contrast mode
   - Reduced motion mode (respect `prefers-reduced-motion`)
   - Screen reader support (Linux accessibility bus)
   - 100% keyboard navigation
   - Magnification support
   - Assistive technology integration

## Development Phases (GNOME-Focused)

### Phase 1: Foundation (Design Language) ✅ COMPLETE
- [x] Design language specification
- [x] Typography system (Inter with fallbacks)
- [x] Color system (Light, Dark, OLED)
- [x] Motion language (spring physics)
- [x] Design tokens and codegen
- [x] Design documentation

### Phase 2: GTK4 Components & GNOME Integration (Aug–Oct 2026)
- [ ] GTK4 component library (Button, Card, Input, Dialog, etc.)
- [ ] GNOME Shell integration (GTK theme, CSS, dconf)
- [ ] GDM theme (GNOME login screen)
- [ ] Motion engine in GTK4 (CSS animations)
- [ ] Icon system (SVG, 1000+ system and application icons)
- [ ] libadwaita widget implementation

### Phase 3: Color System & GNOME App Enhancement (Nov–Dec 2026)
- [ ] Color engine (semantic tokens, HDR support)
- [ ] GNOME core app integration (Nautilus, Settings, Calendar, GNOME Music, Epiphany)
- [ ] Sound design system (notification, interaction, event sounds)
- [ ] Theme management (dconf, gsettings, GTK theme system)

### Phase 4: Accessibility & GNOME Refinement (Jan–Feb 2027)
- [ ] Accessibility layer (WCAG AAA compliance)
- [ ] High contrast mode (increased color contrast for vision impairment)
- [ ] Reduced motion mode (respect GNOME's `prefers-reduced-motion` setting)
- [ ] Screen reader testing (Orca integration)
- [ ] Full keyboard navigation audit (no mouse required)

### Phase 5: Polish & GNOME v1.0 Launch (Mar–Apr 2027)
- [ ] Final refinement and optimization for GNOME
- [ ] GNOME community feedback integration
- [ ] Comprehensive documentation and user guides
- [ ] v1.0 release (GNOME-native design system)

## Design Subsystems

### Design Tokens
Everything is a token. Define and maintain:
- **Spacing**: xxs (2px), xs (4px), sm (8px), md (12px), lg (16px), xl (24px), xxl (32px), xxxl (48px)
- **Radius**: xs (4px), sm (8px), md (12px), lg (16px), xl (24px)
- **Elevation**: level1–level5 (shadow definitions)
- **Motion**: instant (80ms), fast (120ms), normal (220ms), slow (350ms), dramatic (500ms)
- **Semantic Colors**: surface, surfaceVariant, background, foreground, primary, secondary, accent, success, warning, error, info

### Typography System

**Highest-priority subsystem.** Every interaction is typography-first.

Requirements:
- Variable font support (single file, multiple weights/widths)
- Optical sizing (size-responsive adjustments)
- Responsive typography (scale with viewport)
- High-DPI optimization (subpixel rendering)
- Accessibility scaling (user preference override)
- i18n support (CJK, RTL, complex scripts)

Type scales:
- **Display** — Large, attention-grabbing headlines
- **Headline** — Section headings
- **Title** — Card titles, dialog titles
- **Body** — Primary reading content
- **Caption** — Secondary, supplementary text
- **Micro** — Tags, badges, labels

For each scale, define:
- Font size (base + responsive variants)
- Font weight (400, 500, 600, 700)
- Letter spacing (tracking)
- Line height (1.4x–1.6x for body, tighter for display)
- Contrast ratio (WCAG AAA minimum)

### Motion Design Engine

Motion is a first-class system. Every interaction must feel intentional.

Implement:
- **Window Actions** — Open, close, minimize, maximize, restore
- **Desktop Actions** — Workspace switching, app launching, notifications
- **Menu Actions** — Dropdowns, context menus, tooltips, dialogs
- **Transition Actions** — Focus changes, state transitions, data loading

Motion language:
- Spring physics (overshoot, damping) — primary animation language
- Velocity-aware transitions (inherit momentum from gestures)
- GPU acceleration (transform, opacity only—avoid layout thrashing)
- Easing curves (custom, not linear)

### Color System

Support four themes:
- **Light** — Default for daytime
- **Dark** — Low-light environments
- **OLED** — True blacks for OLED displays
- **HDR** — Wide color gamut for HDR-capable displays

Semantic color tokens (never raw hex values to applications):
- `surface` — Interactive element backgrounds
- `surfaceVariant` — Secondary surface (cards, sidebars)
- `background` — Canvas background
- `foreground` — Primary text
- `primary` — Brand color, primary actions
- `secondary` — Secondary actions
- `accent` — Highlights, selected states
- `success`, `warning`, `error`, `info` — Semantic states

Applications compose from semantic tokens only. Raw color values are internal rendering concern.

### Icon System

Build a modern, scalable icon family:
- **1000+ icons** covering system, application, and interaction needs
- **SVG native** — infinitely scalable
- **Consistent geometry** — same visual weight across all icons
- **Consistent stroke width** — typically 1.5–2px at 24x24
- **Pixel-perfect rendering** — subpixel alignment for common sizes

Inspired by SF Symbols and Fluent Icons. Deliver:
- System icons (file, folder, settings, etc.)
- Application icons (browser, terminal, editor, etc.)
- Symbol library (for app developers)
- Figma plugin for design tool integration

### Sound Design System

Sounds are UI feedback, not entertainment. Define:
- **Notifications** — Alerts, messages
- **Success** — Confirmation sounds
- **Error** — Attention, something went wrong
- **Warnings** — Caution, proceeding carefully
- **Window Interactions** — Open, close, focus
- **Workspace Transitions** — Switch workspace, app launch

Requirements:
- Subtle and non-intrusive (optional, disabled by default)
- Spatial audio capable
- Support Bluetooth, USB, built-in speakers
- Accessible, not necessary for understanding (always paired with visual feedback)

### Accessibility Layer (WCAG AAA)

Accessibility is mandatory, not optional. Support:
- **High Contrast Mode** — Increased color contrast for vision impairment
- **Reduced Motion Mode** — Disable spring animations, respect `prefers-reduced-motion`
- **Screen Reader Integration** — Semantic HTML, ARIA labels, announcements
- **Keyboard Navigation** — Full keyboard support, no mouse requirement
- **Voice Interaction** — Voice commands for accessibility (future)
- **Magnification** — OS-level zoom support

Target WCAG AAA where possible. Measure accessibility compliance regularly.

## Success Criteria

Aurora succeeds when:

1. **Visual Cohesion** — All GNOME applications (Files, Settings, Calendar, Music, etc.) feel visually consistent and premium
2. **User Perception** — Users describe GNOME as "as polished as macOS, but open"
3. **Developer Adoption** — >70% of GNOME applications adopt Aurora components
4. **Accessibility Excellence** — WCAG AAA compliance throughout, exceeding commercial OSs (macOS, Windows)
5. **Stability** — Design language remains stable for years without breaking GNOME apps
6. **Ecosystem Impact** — GNOME becomes recognized as the most beautiful desktop environment on Linux

## References

- **Typography**: SF Pro, Inter, IBM Plex Sans, Noto Sans
- **Design Systems**: Human Interface Guidelines (Apple), Material Design (Google), Fluent Design (Microsoft)
- **Motion**: Framer Motion, Spring easing references
- **Color Science**: WCAG, color-contrast-analyzer
- **Icons**: SF Symbols, Fluent Icons
- **Accessibility**: WCAG 2.1, ARIA Authoring Practices

## Contributing

Aurora is open source. All contributions welcome. Please read ARCHITECTURE.md for technical guidelines.

## License

Dual-licensed under MIT and Apache 2.0.

---

**Georgi Mammen Mullassery** | GitHub: Mullassery | Email: mullassery@gmail.com
