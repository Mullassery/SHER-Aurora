# Aurora Architecture

## System Overview

Aurora is a **GNOME-focused design system** built on GTK4 and libadwaita. The architecture is layered:

```
┌─────────────────────────────────────────────────────────────────┐
│                    GNOME Application Layer                      │
│        Files  │  Settings  │  Calendar  │  Music  │  Custom    │
└──────────────────────────┬────────────────────────────────────────┘
                           │
┌──────────────────────────┴────────────────────────────────────────┐
│                  Aurora Component Layer (GTK4)                    │
│    Buttons  │  Cards  │  Inputs  │  Dialogs  │  Custom Widgets  │
└──────────────────────────┬────────────────────────────────────────┘
                           │
┌──────────────────────────┴────────────────────────────────────────┐
│           Core Design System (Rust + CSS Codegen)               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│  │ Design       │  │ Typography   │  │ Color        │            │
│  │ Tokens       │  │ Engine       │  │ System       │            │
│  └──────────────┘  └──────────────┘  └──────────────┘            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│  │ Motion       │  │ Icon         │  │ Sound        │            │
│  │ Engine       │  │ System       │  │ Design       │            │
│  └──────────────┘  └──────────────┘  └──────────────┘            │
│  ┌──────────────────────────────────────────────────────────────┐│
│  │                 Accessibility Layer (WCAG AAA)              ││
│  │  High Contrast │ Reduced Motion │ Screen Readers            ││
│  └──────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
                           │
┌──────────────────────────┴────────────────────────────────────────┐
│           GTK4 + libadwaita + GNOME Infrastructure              │
│    dconf  │  GNOME Settings  │  GNOME Shell  │  Wayland       │
└─────────────────────────────────────────────────────────────────┘
```

## Crate Organization

### Core Subsystems

#### `aurora-tokens`
**Design Token Definition & Codegen**

- YAML-based token definitions
- Codegen pipeline: YAML → Rust structs → JS/CSS exports
- Token validation (contrast ratios, spacing consistency)
- Runtime token resolution (<1ms target)

**Key Types**:
```rust
pub struct DesignTokens {
    pub spacing: SpacingScale,
    pub radius: RadiusScale,
    pub elevation: ElevationScale,
    pub motion: MotionScale,
    pub colors: ColorSystem,
}

pub struct SpacingScale {
    pub xxs: u16,  // 2px
    pub xs: u16,   // 4px
    // ... etc
}
```

**Exports**:
- `tokens.rs` — Rust module
- `tokens.css` — CSS custom properties
- `tokens.json` — JSON for tooling
- `tokens.ts` — TypeScript types

#### `aurora-typography`
**Typography Engine**

- Variable font loading & rendering
- Responsive type scales
- Optical sizing calculations
- i18n support (CJK, RTL, complex scripts)
- Subpixel anti-aliasing configuration
- Line height adjustments per script family

**Key Types**:
```rust
pub struct TypeScale {
    pub display: TypographyStyle,
    pub headline: TypographyStyle,
    pub title: TypographyStyle,
    pub body: TypographyStyle,
    pub caption: TypographyStyle,
    pub micro: TypographyStyle,
}

pub struct TypographyStyle {
    pub font_size: ResponsiveValue,
    pub font_weight: FontWeight,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub font_variant: FontVariant,
}

pub struct ResponsiveValue {
    pub mobile: u16,      // 11" laptop
    pub tablet: u16,      // 14" laptop
    pub desktop: u16,     // 24–27" monitor
    pub ultrawide: u16,   // >30" ultrawide
}
```

**Responsibilities**:
- Font file discovery and loading
- Fallback chain management
- Script-aware line height adjustment
- Baseline alignment calculations
- Subpixel rendering hints

#### `aurora-color`
**Color System & Theme Management**

- Theme definitions (Light, Dark, OLED, HDR)
- Semantic color token resolution
- Contrast ratio validation
- Color space conversions (sRGB, Display P3, Rec2020)
- Accessibility validation (WCAG AAA)

**Key Types**:
```rust
pub enum Theme {
    Light,
    Dark,
    OLED,
    HDR,
}

pub struct ColorSystem {
    pub surface: Color,
    pub surface_variant: Color,
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub outline: Color,
}

impl ColorSystem {
    pub fn validate_contrast(&self) -> ValidationResult;
    pub fn to_hdr(&self) -> HdrColorSystem;
}
```

**Responsibilities**:
- Theme selection and switching
- Color interpolation (for animations, gradients)
- Contrast ratio calculation
- Color space conversion (for HDR display support)
- Semantic color resolution

#### `aurora-motion`
**Motion Engine & Animation System**

- Spring physics engine
- Velocity-aware transitions
- Easing curve definitions
- GPU acceleration hints
- Reduced motion detection

**Key Types**:
```rust
pub struct SpringConfig {
    pub mass: f32,
    pub tension: f32,
    pub friction: f32,
}

pub enum AnimationDuration {
    Instant,   // 80ms
    Fast,      // 120ms
    Normal,    // 220ms
    Slow,      // 350ms
    Dramatic,  // 500ms
}

pub struct Animation {
    pub from: f32,
    pub to: f32,
    pub duration: AnimationDuration,
    pub easing: EasingFunction,
    pub spring_config: Option<SpringConfig>,
}

impl Animation {
    pub fn frame(&self, elapsed: Duration) -> f32;
    pub fn with_velocity(&self, velocity: f32) -> Animation;
}
```

**Responsibilities**:
- Spring physics calculations
- Easing function evaluation
- Animation timeline management
- GPU optimization (transform, opacity only)
- Reduced motion preference respect

#### `aurora-icons`
**Icon System**

- SVG icon library (1000+ icons)
- Icon metadata (category, tags, variants)
- Rendering at multiple sizes
- Font icon generation
- Figma plugin data export

**Key Types**:
```rust
pub struct IconLibrary {
    pub icons: HashMap<String, Icon>,
    pub categories: HashMap<String, Vec<String>>,
}

pub struct Icon {
    pub name: String,
    pub svg_path: String,
    pub viewbox: String,
    pub stroke_width: f32,
    pub tags: Vec<String>,
    pub variants: Vec<String>,  // e.g., "filled", "outlined"
}

impl Icon {
    pub fn render_svg(&self, size: u32) -> String;
    pub fn render_font(&self) -> FontGlyph;
}
```

**Responsibilities**:
- Icon discovery and retrieval
- SVG rendering at various scales
- Icon font generation
- Icon metadata management
- Figma plugin data export

#### `aurora-sound`
**Sound Design System**

- Sound effect definitions
- Audio asset management
- Multi-output support (speakers, Bluetooth, USB)
- Spatial audio metadata
- Accessibility alternatives

**Key Types**:
```rust
pub struct SoundEffect {
    pub name: String,
    pub category: SoundCategory,
    pub duration: Duration,
    pub file_path: String,
    pub volume: f32,
    pub supports_spatial: bool,
}

pub enum SoundCategory {
    Notification,
    Success,
    Error,
    Warning,
    WindowInteraction,
    WorkspaceTransition,
}

pub struct SoundSystem {
    pub effects: HashMap<String, SoundEffect>,
    pub enabled: bool,
    pub volume: f32,
}
```

**Responsibilities**:
- Sound effect management
- Audio device detection
- Volume normalization
- Accessibility alternatives (screen reader announcements)

#### `aurora-a11y`
**Accessibility Layer**

- High contrast mode
- Reduced motion detection
- Screen reader API integration
- Keyboard navigation support
- Magnification hints
- Voice interaction framework

**Key Types**:
```rust
pub struct AccessibilityContext {
    pub high_contrast_enabled: bool,
    pub prefers_reduced_motion: bool,
    pub screen_reader_enabled: bool,
    pub magnification_level: f32,
    pub keyboard_navigation_enabled: bool,
}

pub trait AccessibleComponent {
    fn accessibility_label(&self) -> String;
    fn accessibility_role(&self) -> AccessibilityRole;
    fn accessibility_state(&self) -> AccessibilityState;
    fn keyboard_shortcuts(&self) -> Vec<KeyboardShortcut>;
}

pub enum AccessibilityRole {
    Button,
    Link,
    MenuItem,
    Dialog,
    Textbox,
    Checkbox,
    RadioButton,
    Heading(HeadingLevel),
    List,
    Listitem,
}
```

**Responsibilities**:
- OS accessibility preference detection
- Semantic component labeling
- Keyboard navigation support
- Screen reader integration
- Magnification support

#### `aurora-core`
**Unified API**

Exposes all subsystems through a single, coherent Rust API:

```rust
pub struct Aurora {
    pub tokens: DesignTokens,
    pub typography: TypeSystem,
    pub colors: ColorSystem,
    pub motion: MotionEngine,
    pub icons: IconLibrary,
    pub sounds: SoundSystem,
    pub accessibility: AccessibilityLayer,
}

impl Aurora {
    pub fn new() -> Self;
    pub fn set_theme(&mut self, theme: Theme);
    pub fn set_accessibility(&mut self, context: AccessibilityContext);
    pub fn resolve_token(&self, token_path: &str) -> TokenValue;
    pub fn animate(&self, animation: Animation) -> AnimationHandle;
}
```

### GNOME Renderer

#### `aurora-gtk`
**GTK4 Component Library**

- GTK4-native widget implementations (Button, Card, Dialog, Input, etc.)
- CSS provider for token-to-CSS-properties conversion
- Motion engine integration (spring animations in GTK)
- Event handling and interaction
- GNOME Shell integration
- Wayland-native support

**Structure**:
- `src/widgets/` — Button, Card, Dialog, Input, Tooltip, etc.
- `src/css/` — CSS generation from tokens
- `src/motion/` — Animation integration with GTK
- `src/accessible/` — WCAG AAA accessibility features
- `src/gnome/` — GNOME Settings and dconf integration
- Web standard compliance

**Structure**:
## Data Flow

### Token Resolution

```
GNOME Application requests "primary color"
        ↓
aurora-core resolves request
        ↓
ColorSystem looks up in current Theme (Light/Dark/OLED)
        ↓
Theme provides Color value
        ↓
GTK Renderer converts to CSS custom property
        ↓
Application receives GNOME-native value
```

### Animation Execution

```
Application requests animation (from → to over duration)
        ↓
MotionEngine calculates animation timeline
        ↓
Renderer schedules frame callbacks
        ↓
Each frame: MotionEngine calculates intermediate value
        ↓
Renderer applies to platform-native properties
        ↓
GPU acceleration (transform, opacity) where possible
```

### Theme Switching

```
System notifies theme change (light → dark)
        ↓
Aurora receives notification
        ↓
ColorSystem switches active theme
        ↓
Each renderer re-renders with new colors
        ↓
Transition animated over `motion.normal` (220ms)
```

## Design Decisions

### 1. Rust as Core Language

**Why**:
- Type safety ensures token consistency
- Performance (WASM compilation target)
- Memory efficiency
- FFI capabilities for platform bindings

**Tradeoff**: Steeper learning curve for contributors, but eliminates entire classes of bugs.

### 2. Token-Driven Everything

**Why**:
- Single source of truth
- Easy theme switching
- Consistency across GNOME applications
- Design tooling integration (Figma, XD)
- CSS codegen for GTK

**Tradeoff**: Initial setup overhead, but pays dividends at scale.

### 3. GTK4-First Design

**Why**:
- Leverage GTK4/libadwaita as standard GNOME toolkit
- Native Wayland support
- Deep GNOME Shell integration
- Optimal accessibility compliance

**Tradeoff**: GNOME-specific focus, but maximum polish for GNOME users.

### 4. Spring Physics Over Curves

**Why**:
- Spring physics feel more natural and responsive
- Velocity-aware (inherit momentum from gestures)
- Consistent feel across animations

**Tradeoff**: Slightly more CPU overhead, but imperceptible on modern hardware.

### 5. Accessibility as Mandatory First-Class System

**Why**:
- WCAG AAA compliance from the start
- Not an afterthought
- Entire system designed with accessibility in mind

**Tradeoff**: Constraints, but constraints breed elegance.

## Performance Targets

| Metric | Target | Method |
|--------|--------|--------|
| Token Resolution | <1ms | Hash map lookup, compiled tokens |
| Animation Frame | 60fps (16.67ms) | GPU acceleration, optimized calculations |
| Cold Startup | <100ms | Lazy loading, compiled tokens |
| System Memory | <100MB | Efficient data structures, WASM optimization |
| TTL (Time to Interactive) | <200ms | Preload fonts, cache colors |

## Development Workflow

1. **Design changes** — Update YAML tokens
2. **Codegen** — Run `cargo build` to regenerate Rust/CSS/JSON
3. **Local testing** — Test GTK4 components with libadwaita
4. **GNOME testing** — Test in GNOME Shell, Settings, Files, Calendar
5. **Accessibility audit** — Validate WCAG AAA compliance
6. **Documentation** — Update design language spec

## Future Enhancements (Post-v1.0)

### GNOME Integration Deepening

- GNOME dynamic theme support
- dconf preference integration
- GNOME Shell animation integration
- Notification system styling

### Display Support

- Extended color gamut rendering (HDR)
- OLED true black optimization
- HDR metadata and display integration
- Multi-monitor optimization

### Accessibility Expansion

- Voice command support (future)
- Enhanced screen reader integration
- Audio feedback customization

---

This architecture is **designed for scale and longevity**. Each layer is independently testable, replaceable, and maintainable.
