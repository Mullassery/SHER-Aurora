# Aurora Architecture

## System Overview

Aurora is a **token-first design system** with platform-agnostic rendering backends. The architecture is layered:

```
┌─────────────────────────────────────────────────────────────────┐
│                     Application Layer                           │
│          GTK4 Apps  │  Qt6 Apps  │  Web Apps  │  Electron      │
└──────────────────────────┬────────────────────────────────────────┘
                           │
┌──────────────────────────┴────────────────────────────────────────┐
│                    Renderer Layer                                 │
│    GTK Renderer  │  Qt Renderer  │  Web/WASM Renderer            │
└──────────────────────────┬────────────────────────────────────────┘
                           │
┌──────────────────────────┴────────────────────────────────────────┐
│              Core Design System (Rust + WASM)                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│  │ Design       │  │ Typography   │  │ Color        │            │
│  │ Tokens       │  │ Engine       │  │ System       │            │
│  └──────────────┘  └──────────────┘  └──────────────┘            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│  │ Motion       │  │ Icon         │  │ Sound        │            │
│  │ Engine       │  │ System       │  │ Design       │            │
│  └──────────────┘  └──────────────┘  └──────────────┘            │
│  ┌──────────────────────────────────────────────────────────────┐│
│  │                 Accessibility Layer                         ││
│  │  High Contrast │ Reduced Motion │ Screen Readers            ││
│  └──────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
                           │
┌──────────────────────────┴────────────────────────────────────────┐
│                   Data & Configuration                           │
│    Design Tokens (YAML)  │  Font Manifests  │  Icon Metadata    │
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

### Platform Renderers

#### `aurora-gtk`
**GTK4 Renderer**

- GTK4-native widget implementations
- CSS provider for token-to-CSS-properties conversion
- Event handling and interaction
- Wayland/X11 compatibility

**Structure**:
- `src/widgets/` — Button, Card, Dialog, etc.
- `src/css/` — CSS generation from tokens
- `src/events/` — Event handling, animation callbacks
- `src/accessible/` — GTK accessibility integration

#### `aurora-qt`
**Qt6 Renderer (via FFI)**

- Qt6 C++ bindings
- QSS (Qt Style Sheets) generation
- Qt Quick integration
- Platform integration (KDE, Cosmic, XFCE)

**Structure**:
- `crates/aurora-qt/` — Rust wrapper
- `crates/aurora-qt-cpp/` — C++ bindings
- `src/widgets/` — C++ implementations
- `src/styles/` — QSS generation

#### `aurora-web`
**Web/WASM Renderer**

- WASM compilation of core Aurora
- CSS custom properties generation
- React/Vue/Svelte component library
- Web standard compliance

**Structure**:
- `src/lib.rs` — WASM entry point
- `src/css/` — CSS generation
- `packages/aurora-web-react/` — React components
- `packages/aurora-web-vue/` — Vue components
- `packages/aurora-web-svelte/` — Svelte components

## Data Flow

### Token Resolution

```
Application requests "primary color"
        ↓
aurora-core resolves request
        ↓
ColorSystem looks up in current Theme
        ↓
Theme provides Color value
        ↓
Renderer converts to platform-specific (CSS, Qt, GTK)
        ↓
Application receives platform-native value
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
- Consistency across platforms
- Design tooling integration (Figma, XD)

**Tradeoff**: Initial setup overhead, but pays dividends at scale.

### 3. Separate Renderers Per Platform

**Why**:
- Leverage platform-native capabilities
- Optimize for each platform's rendering model
- Maintain accessibility compliance (GTK accessibility, Qt accessibility)

**Tradeoff**: Code duplication, but consistency enforced at token layer.

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
3. **Local testing** — Test in GTK/Qt/Web renderers
4. **Platform testing** — Run on GNOME, KDE, Cosmic, web
5. **Accessibility audit** — Validate WCAG compliance
6. **Documentation** — Update design language spec

## Future Extensibility

### Phase 5: AI Personalization

- Local adaptation model (no cloud)
- Signals: time of day, ambient light, display type, interaction patterns
- Adaptations: contrast, density, font size, motion intensity

### HDR/OLED Optimization

- Extended color gamut rendering
- OLED true black optimization
- HDR metadata and display integration

### Voice Interaction

- Voice command support
- Screen reader integration
- Audio interface

### Desktop Shell Integration

- GNOME dynamic theme support
- KDE Plasma color scheme integration
- Cosmic native integration
- XFCE theme support

---

This architecture is **designed for scale and longevity**. Each layer is independently testable, replaceable, and maintainable.
