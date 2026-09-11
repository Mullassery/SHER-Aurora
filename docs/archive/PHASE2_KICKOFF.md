# Aurora Phase 2: GTK4 Components & GNOME Integration

**Timeline**: Aug 2026 – Oct 2026 (12–16 weeks)  
**Target**: Deliver production-ready GTK4 component library with GNOME integration  
**Team**: 4–6 engineers  
**Status**: 🟡 In Progress

---

## Phase 2 Deliverables

### 2.1 GTK4 Component Library (Weeks 1–8)

**Core Components** (10 essential widgets):
- [ ] **Button** (filled, tinted, outlined, ghost variants)
  - [ ] Hover/active/disabled states
  - [ ] Icon support
  - [ ] Motion animations
  - [ ] Accessibility (ARIA labels, keyboard focus)
  - [ ] Tests (10+)

- [ ] **Card** (filled, outlined, elevated)
  - [ ] Shadow elevation system
  - [ ] Hover states
  - [ ] Padding/content area
  - [ ] Accessibility
  - [ ] Tests (8+)

- [ ] **Input** (text, password, search)
  - [ ] Focus states
  - [ ] Error states
  - [ ] Placeholder text
  - [ ] Icon support (prefix/suffix)
  - [ ] Accessibility
  - [ ] Tests (10+)

- [ ] **Dialog** (modal, non-blocking)
  - [ ] Open/close animations
  - [ ] Button actions
  - [ ] Focus trapping
  - [ ] Accessibility
  - [ ] Tests (8+)

- [ ] **Checkbox** 
  - [ ] Checked/unchecked/indeterminate states
  - [ ] Animation
  - [ ] Accessibility (ARIA)
  - [ ] Tests (6+)

- [ ] **Radio Button**
  - [ ] Single/multiple selection
  - [ ] Animation
  - [ ] Accessibility
  - [ ] Tests (6+)

- [ ] **Tooltip**
  - [ ] Positioning
  - [ ] Accessibility (screen reader)
  - [ ] Delay/timing
  - [ ] Tests (5+)

- [ ] **List** / **ListItem**
  - [ ] Item selection
  - [ ] Focus navigation
  - [ ] Accessibility
  - [ ] Tests (8+)

- [ ] **Badge** / **Chip**
  - [ ] Variants (default, success, warning, error)
  - [ ] Dismissible chips
  - [ ] Icon support
  - [ ] Tests (6+)

- [ ] **Sidebar**
  - [ ] Navigation items
  - [ ] Active state
  - [ ] Collapse/expand animation
  - [ ] Accessibility
  - [ ] Tests (8+)

**Target**: 50+ tests per component, 100% keyboard navigable

---

### 2.2 Motion Engine in GTK4 (Weeks 1–4)

**Goals**:
- Integrate aurora-motion into GTK4 widgets
- Window animations (open, close, minimize, maximize)
- Component animations (button hover, card elevation, dialog appear)
- Gesture tracking (if applicable)

**Deliverables**:
- [ ] GTK animation callback integration
- [ ] Spring physics applied to GTK properties
- [ ] CSS animation generation
- [ ] 60fps minimum on modern hardware
- [ ] Reduced motion support (`prefers-reduced-motion`)
- [ ] Tests (20+)

---

### 2.3 CSS Provider & Token Export (Weeks 2–4)

**Status**: Partially complete (aurora-gtk/src/css.rs)

**Remaining Work**:
- [ ] CSS custom properties for all tokens
  - `--spacing-*` (8 values)
  - `--radius-*` (5 values)
  - `--color-*` (12+ semantic colors)
  - `--motion-*` (5 durations)
  - `--elevation-*` (5 shadow definitions)

- [ ] Theme switching (Light/Dark/OLED)
- [ ] dconf integration for theme preference
- [ ] GNOME Settings integration
- [ ] Tests (15+)

---

### 2.4 GNOME Shell Integration (Weeks 5–8)

**Deliverables**:
- [ ] Theme switching via GNOME Settings
- [ ] Accent color integration
- [ ] Notification styling
- [ ] Cursor styling
- [ ] High contrast mode support
- [ ] Wallpaper integration hooks
- [ ] Tests (10+)

---

### 2.5 Example Application (Weeks 8–12)

**Deliverable**: Complete GTK4 application showcasing:
- [ ] All 10 core components
- [ ] Motion language in action
- [ ] Theme switching
- [ ] Accessibility features
- [ ] Dark mode
- [ ] Multi-language support (English, Spanish, Chinese)
- [ ] ~500 lines of well-documented Rust

---

### 2.6 Documentation & Tooling (Weeks 12–16)

**Deliverables**:
- [ ] Component API reference
- [ ] Integration guide for GNOME app developers
- [ ] Code examples for each component
- [ ] Storybook-style component showcase
- [ ] Accessibility audit checklist
- [ ] Performance benchmarks

---

## Success Criteria

| Metric | Target | Status |
|--------|--------|--------|
| **Components** | 10 core widgets, production-ready | 🟡 In Progress |
| **Tests** | 80+ tests, >90% coverage | 🟡 In Progress |
| **Keyboard Nav** | 100% of UI navigable | 🟡 Target |
| **Accessibility** | WCAG AAA compliant | 🟡 Target |
| **Motion** | 60fps minimum, <10ms latency | 🟡 Target |
| **Documentation** | Complete API reference + examples | 🟡 Planned |
| **Example App** | Showcases all components | 🟡 Planned |

---

## Current Status (Aug 1, 2026)

### ✅ Complete
- Phase 1: Design language, tokens, typography, color system
- aurora-tokens: All spacing, radius, elevation, motion, color tokens
- aurora-typography: Type scales, responsive typography, i18n
- aurora-color: Light/Dark/OLED themes with validation
- aurora-motion: Spring physics, animation system, window animations
- aurora-gtk: CSS provider scaffolding

### 🟡 In Progress
- aurora-gtk: Component library (partial)
  - Window motion system ✅
  - Button component (partial)
  
### 🔴 Not Started
- Card, Input, Dialog, Checkbox, Radio, Tooltip, List, Badge, Sidebar components
- GNOME Settings integration
- Example application
- Full documentation

---

## Architecture (GTK4 Layer)

```
GNOME Application
    ↓
Aurora Components (aurora-gtk)
├── src/widgets/
│   ├── button.rs
│   ├── card.rs
│   ├── input.rs
│   ├── dialog.rs
│   ├── checkbox.rs
│   ├── radio.rs
│   ├── tooltip.rs
│   ├── list.rs
│   ├── badge.rs
│   └── sidebar.rs
├── src/css/
│   └── provider.rs (token → CSS custom properties)
├── src/motion/
│   └── gtk_animator.rs (integration with GTK)
├── src/gnome/
│   └── settings.rs (GNOME Settings integration)
└── src/lib.rs (exports all components)
    ↓
Aurora Core (Rust)
├── aurora-tokens
├── aurora-typography
├── aurora-color
├── aurora-motion
└── aurora-a11y
    ↓
GTK4 + libadwaita
    ↓
Wayland Compositor
```

---

## Week-by-Week Breakdown

### Weeks 1–2: Button & Card
- [ ] Button widget (filled, tinted, outlined, ghost)
- [ ] Card widget (filled, outlined, elevated)
- [ ] Motion animations
- [ ] CSS generation
- [ ] Tests (15+)

### Weeks 3–4: Input & Dialog
- [ ] Input widget (text, password, search)
- [ ] Dialog widget (modal, non-blocking)
- [ ] Focus management
- [ ] Accessibility features
- [ ] Tests (15+)

### Weeks 5–6: Checkbox, Radio, Tooltip
- [ ] Checkbox widget
- [ ] Radio button widget
- [ ] Tooltip widget
- [ ] List widget
- [ ] Tests (12+)

### Weeks 7–8: Badge, Chip, Sidebar
- [ ] Badge widget
- [ ] Chip widget
- [ ] Sidebar widget
- [ ] Polish all components
- [ ] Tests (12+)

### Weeks 9–12: Example App & Integration
- [ ] GNOME Settings integration
- [ ] Example application
- [ ] Documentation
- [ ] Performance optimization

### Weeks 13–16: Final Polish & v2.0 RC
- [ ] Bug fixes
- [ ] Accessibility audit
- [ ] Performance profiling
- [ ] Release candidate (v2.0-rc1)

---

## Key Decisions for Phase 2

### 1. Component Scope
Focus on 10 core components that cover ~90% of GNOME UI patterns. Avoid over-specialization.

### 2. Motion Strategy
- Use CSS animations where possible (GPU-accelerated)
- Fall back to GTK animation callbacks for complex spring physics
- Respect `prefers-reduced-motion` always

### 3. Accessibility First
- WCAG AAA compliance from day 1
- Keyboard navigation built-in
- Screen reader support via ARIA
- High contrast mode support

### 4. Testing Strategy
- Unit tests for each component
- Integration tests with GTK
- Accessibility audit tests
- Performance benchmarks

### 5. Documentation
- API reference (rustdoc)
- Integration guide for app developers
- Example code snippets
- Storybook-style showcase

---

## Getting Started (Next Steps)

1. **Review this document** — Align on Phase 2 scope and timeline
2. **Set up component structure** — Create widget modules
3. **Implement Button component** — Weeks 1–2 kickoff
4. **Write tests** — Ensure >90% coverage from the start
5. **Create example app** — Showcase components as they're built

---

## Questions & Blockers

- [ ] Should we use `gtk-rs` bindings or hand-written FFI? (Recommendation: gtk-rs for stability)
- [ ] Component state management pattern? (Recommendation: Internal mutability with Rc<RefCell<>>)
- [ ] Animation framework? (Recommendation: Leverage aurora-motion + GTK callbacks)
- [ ] Documentation tool? (Recommendation: rustdoc + custom examples)

---

## Sign-Off

Phase 2 is ready to begin. Success criteria are clear. Timeline is realistic.

**v2.0 Target**: Q4 2026 (component library release)

**Let's build beautiful GNOME components.** 🚀
