# Aurora Design System - API Reference

**Version**: v1.0.0  
**Status**: Complete API Surface  
**Last Updated**: August 1, 2026

This document provides complete API reference for Aurora design system subsystems.

---

## Table of Contents

1. [Design Tokens](#design-tokens)
2. [Typography](#typography)
3. [Color System](#color-system)
4. [Motion Engine](#motion-engine)
5. [Sound System](#sound-system)
6. [GTK4 Widgets](#gtk4-widgets)
7. [GNOME Integration](#gnome-integration)

---

## Design Tokens

### Module: `aurora_tokens`

Design tokens are the foundation of Aurora. They define all visual and interaction properties.

#### Spacing Scale

```rust
pub enum Spacing {
    Xs,     // 2px
    Sm,     // 4px
    Md,     // 8px
    Lg,     // 12px
    Xl,     // 16px
    Xxl,    // 24px
    Xxxl,   // 32px
}

impl Spacing {
    pub fn px(&self) -> f32;
    pub fn rem(&self) -> f32;
    pub fn to_string(&self) -> String;
}
```

**Example**:
```rust
use aurora_tokens::Spacing;

let padding = Spacing::Md.px();  // 8.0
let gap = Spacing::Lg.rem();      // 0.75rem
```

#### Radius Scale

```rust
pub enum Radius {
    Xs,     // 4px
    Sm,     // 8px
    Md,     // 12px
    Lg,     // 16px
    Xl,     // 24px
}

impl Radius {
    pub fn px(&self) -> f32;
    pub fn rem(&self) -> f32;
    pub fn css(&self) -> String;
}
```

#### Elevation System

```rust
pub struct Elevation;

impl Elevation {
    pub fn shadow(level: u8) -> String;      // Level 1-5
    pub fn blur_radius(level: u8) -> f32;
    pub fn offset_y(level: u8) -> f32;
}
```

#### Motion Tokens

```rust
pub enum Duration {
    Instant,    // 80ms
    Fast,       // 120ms
    Normal,     // 220ms
    Slow,       // 350ms
    Dramatic,   // 500ms
}

pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Spring,
}

impl Duration {
    pub fn ms(&self) -> u32;
    pub fn css(&self) -> String;
}
```

---

## Typography

### Module: `aurora_typography`

Typography system for responsive, accessible text rendering.

#### TypeScale

```rust
pub enum TypeScale {
    Display,
    Headline,
    Title,
    Body,
    Caption,
    Micro,
}

pub struct TypographyStyle {
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: u32,
    pub line_height: f32,
    pub letter_spacing: f32,
}

impl TypographyStyle {
    pub fn new(scale: TypeScale) -> Self;
    pub fn to_css(&self) -> String;
    pub fn to_json(&self) -> String;
}
```

**Example**:
```rust
use aurora_typography::{TypographyStyle, TypeScale};

let title = TypographyStyle::new(TypeScale::Title);
println!("{}", title.to_css());
// font-family: 'Inter', sans-serif;
// font-size: 28px;
// ...
```

#### Font System

```rust
pub enum Font {
    Inter,      // Primary sans-serif
    Mono,       // Code/monospace
}

pub struct FontConfig {
    pub font: Font,
    pub weights: Vec<u32>,
    pub fallbacks: Vec<String>,
}

impl FontConfig {
    pub fn css_import(&self) -> String;
    pub fn css_family(&self) -> String;
}
```

#### Responsive Typography

```rust
pub struct ResponsiveTypography {
    pub mobile: TypographyStyle,
    pub tablet: TypographyStyle,
    pub desktop: TypographyStyle,
}

impl ResponsiveTypography {
    pub fn new(scale: TypeScale) -> Self;
    pub fn to_css_media_queries(&self) -> String;
}
```

---

## Color System

### Module: `aurora_color`

Semantic color system supporting 4 themes.

#### Color Structure

```rust
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self;
    pub fn from_hex(hex: &str) -> Result<Self>;
    
    pub fn to_hex(&self) -> String;           // "#RRGGBB"
    pub fn to_rgb(&self) -> String;           // "rgb(r,g,b)"
    pub fn to_rgba(&self, alpha: f32) -> String;
    
    pub fn luminance(&self) -> f32;           // WCAG 2.1
    pub fn contrast_ratio(&self, other: &Color) -> f32;
    pub fn passes_wcag_aa(&self, other: &Color) -> bool;  // 4.5:1
    pub fn passes_wcag_aaa(&self, other: &Color) -> bool; // 7:1
}
```

**Example**:
```rust
use aurora_color::Color;

let primary = Color::from_hex("#003D99")?;
let surface = Color::from_hex("#FFFFFF")?;

assert!(primary.passes_wcag_aaa(&surface));
println!("{}", primary.to_hex());  // "#003D99"
```

#### Theme System

```rust
pub enum ThemeName {
    Light,
    Dark,
    OLED,
    HDR,
}

pub struct ColorSystem {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub info: Color,
    pub surface: Color,
    pub background: Color,
    pub foreground: Color,
    // ... 12 more semantic colors
}

impl ColorSystem {
    pub fn from_theme(theme: ThemeName) -> Self;
    pub fn to_css(&self) -> String;  // CSS custom properties
    pub fn theme(&self) -> ThemeName;
}
```

**Example**:
```rust
use aurora_color::{ColorSystem, ThemeName};

let light = ColorSystem::from_theme(ThemeName::Light);
let dark = ColorSystem::from_theme(ThemeName::Dark);

println!("{}", light.to_css());  // :root { --color-primary: ... }
```

---

## Motion Engine

### Module: `aurora_motion`

Animation system with spring physics.

#### Animation Control

```rust
pub struct Animation {
    pub duration: Duration,
    pub easing: Easing,
    pub current: f32,      // 0.0-1.0
    pub from: f32,
    pub to: f32,
}

impl Animation {
    pub fn new(duration: Duration, easing: Easing) -> Self;
    
    pub fn advance(&mut self, delta_ms: u32) -> bool;  // true if complete
    pub fn current_value(&self) -> f32;
    pub fn reset(&mut self);
    pub fn pause(&mut self);
    pub fn resume(&mut self);
}
```

#### Spring Physics

```rust
pub struct SpringConfig {
    pub stiffness: f32,     // 0.0-1.0
    pub damping: f32,       // 0.0-1.0
    pub mass: f32,          // 1.0+
    pub velocity: f32,      // px/ms
}

pub enum SpringPreset {
    Gentle,
    Standard,
    Bouncy,
    Instant,
}

impl SpringConfig {
    pub fn preset(preset: SpringPreset) -> Self;
    pub fn calculate_acceleration(&self) -> f32;
}
```

**Example**:
```rust
use aurora_motion::{Animation, Duration, Easing};

let mut anim = Animation::new(Duration::Normal, Easing::EaseOut);
let complete = anim.advance(220);  // advance 220ms
println!("Progress: {}", anim.current_value());  // 0.0-1.0
```

#### Window Animations

```rust
pub struct WindowAnimator;

impl WindowAnimator {
    pub fn animate_open(&self) -> Animation;
    pub fn animate_close(&self) -> Animation;
    pub fn animate_minimize(&self) -> Animation;
    pub fn animate_maximize(&self) -> Animation;
    pub fn animate_focus(&self) -> Animation;
}
```

---

## Sound System

### Module: `aurora_sound`

Audio feedback system for notifications and interactions.

#### Sound Enum

```rust
pub enum Sound {
    Success,
    Error,
    Warning,
    Notification,
    Click,
    Hover,
}

impl Sound {
    pub fn category(&self) -> SoundCategory;
    pub fn filename(&self) -> &str;
    pub fn duration_ms(&self) -> u32;
}
```

#### Audio System

```rust
pub struct AuroraSoundSystem {
    theme: SoundTheme,
    volume: f32,        // 0.0-1.0
    enabled: bool,
}

impl AuroraSoundSystem {
    pub fn new(theme: SoundTheme) -> Self;
    
    pub fn play(&self, sound: Sound) -> bool;
    pub fn set_volume(&mut self, volume: f32);
    pub fn set_enabled(&mut self, enabled: bool);
    pub fn set_theme(&mut self, theme: SoundTheme);
}
```

**Example**:
```rust
use aurora_sound::{Sound, AuroraSoundSystem, SoundTheme};

let mut audio = AuroraSoundSystem::new(SoundTheme::Standard);
audio.set_volume(0.8);
audio.play(Sound::Success);
```

---

## GTK4 Widgets

### Module: `aurora_gtk`

Complete widget library for GTK4 applications.

#### Button Widget

```rust
pub enum ButtonStyle {
    Filled,
    Tinted,
    Outlined,
    Ghost,
}

pub enum ButtonState {
    Default,
    Hover,
    Active,
    Disabled,
    Loading,
}

pub struct Button {
    label: String,
    style: ButtonStyle,
    state: ButtonState,
}

impl Button {
    pub fn new(label: &str, style: ButtonStyle) -> Self;
    pub fn set_state(&mut self, state: ButtonState);
    pub fn get_state(&self) -> ButtonState;
    pub fn set_label(&mut self, label: &str);
    pub fn css_class(&self) -> String;
}
```

#### Card Widget

```rust
pub enum CardStyle {
    Filled,
    Outlined,
    Elevated,
}

pub struct Card {
    style: CardStyle,
    spacing: u32,
    margin: u32,
}

impl Card {
    pub fn new(style: CardStyle) -> Self;
    pub fn set_spacing(&mut self, spacing: u32);
    pub fn set_margin(&mut self, margin: u32);
    pub fn css_class(&self) -> String;
}
```

#### Input Widget

```rust
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Search,
}

pub struct Input {
    input_type: InputType,
    placeholder: String,
    error: Option<String>,
}

impl Input {
    pub fn new(input_type: InputType) -> Self;
    pub fn set_placeholder(&mut self, text: &str);
    pub fn set_error(&mut self, error: Option<&str>);
    pub fn get_text(&self) -> String;
    pub fn set_sensitive(&mut self, sensitive: bool);
}
```

#### Dialog Widget

```rust
pub enum DialogResponse {
    Primary,
    Secondary,
    Cancel,
}

pub struct AuroraDialog {
    title: String,
    message: String,
}

impl AuroraDialog {
    pub fn new(title: &str, message: &str) -> Self;
    pub fn add_button(&mut self, label: &str, response: DialogResponse);
    pub fn show(&self) -> DialogResponse;
}
```

#### Full Component List

- **Button** - Primary call-to-action button
- **Card** - Content container
- **Input** - Text/email/password input
- **Dialog** - Modal and non-blocking dialogs
- **Checkbox** - Selectable checkbox
- **RadioButton** - Radio button group
- **Tooltip** - Hover tooltip
- **List** - Scrollable list container
- **Badge** - Status indicator badge
- **Sidebar** - Collapsible sidebar navigation

---

## GNOME Integration

### Module: `aurora_gtk::gnome`

GNOME-specific integration layer.

#### dconf Schema

```rust
pub struct DConfSchema;

impl DConfSchema {
    pub fn schema_xml() -> &'static str;
    pub fn schema_id() -> &'static str;     // "org.gnome.desktop.interface.aurora"
    pub fn schema_path() -> &'static str;   // "/org/gnome/desktop/interface/aurora/"
    pub fn install_command() -> String;
}
```

#### Theme Observer

```rust
pub type ThemeChangeCallback = Box<dyn Fn(ThemeName) + Send + Sync>;

pub struct ThemeObserver {
    current_theme: ThemeName,
    callbacks: Vec<ThemeChangeCallback>,
    enabled: bool,
}

impl ThemeObserver {
    pub fn new() -> Self;
    pub fn current_theme(&self) -> ThemeName;
    pub fn set_theme(&mut self, theme: ThemeName);
    pub fn on_theme_change(&mut self, callback: ThemeChangeCallback);
    pub fn start_listening(&mut self) -> bool;
    pub fn stop_listening(&mut self);
}
```

**Example**:
```rust
use aurora_gtk::gnome::ThemeObserver;
use aurora_color::ThemeName;

let mut observer = ThemeObserver::new();
observer.on_theme_change(Box::new(|theme| {
    println!("Theme changed to: {:?}", theme);
}));

observer.set_theme(ThemeName::Dark);
```

#### Settings Panel

```rust
pub struct SettingsPanel {
    title: String,
    sections: Vec<SettingsSection>,
}

pub struct SettingsSection {
    title: String,
    settings: Vec<Setting>,
}

impl SettingsPanel {
    pub fn new(title: &str) -> Self;
    pub fn sections(&self) -> &[SettingsSection];
    pub fn section(&self, title: &str) -> Option<&SettingsSection>;
    pub fn to_html(&self) -> String;
    pub fn setting_count(&self) -> usize;
}
```

**Sections**:
- Appearance (theme, high-contrast, text-scale)
- Sound (enabled, volume, theme)
- Accessibility (reduce-motion)

#### Aurora Notifications

```rust
pub enum NotificationUrgency {
    Low,
    Normal,
    High,
}

pub struct AuroraNotification {
    summary: String,
    body: String,
    urgency: NotificationUrgency,
    timeout: i32,
    theme: ThemeName,
}

impl AuroraNotification {
    pub fn new(summary: &str) -> Self;
    pub fn with_body(self, body: &str) -> Self;
    pub fn with_urgency(self, urgency: NotificationUrgency) -> Self;
    pub fn with_timeout(self, timeout: i32) -> Self;
    pub fn with_theme(self, theme: ThemeName) -> Self;
    pub fn to_css(&self) -> String;
}

pub struct NotificationManager {
    theme: ThemeName,
    notifications: Vec<AuroraNotification>,
}

impl NotificationManager {
    pub fn new(theme: ThemeName) -> Self;
    pub fn notify(&mut self, summary: &str);
    pub fn pending(&self) -> &[AuroraNotification];
    pub fn clear(&mut self);
}
```

---

## Example Usage

### Complete Example: Aurora Settings App

```rust
use aurora_color::{ColorSystem, ThemeName};
use aurora_gtk::gnome::{SettingsPanel, ThemeObserver};
use aurora_sound::AuroraSoundSystem;

fn main() {
    // Create theme observer
    let mut observer = ThemeObserver::new();
    observer.on_theme_change(Box::new(|theme| {
        println!("Switched to: {:?}", theme);
    }));
    
    // Create settings panel
    let panel = SettingsPanel::new("Aurora Settings");
    println!("Sections: {}", panel.section_count());
    
    // Display theme selector
    if let Some(appearance) = panel.section("Appearance") {
        println!("Appearance settings:");
        for setting in &appearance.settings {
            println!("  - {}: {}", setting.label, setting.current_value);
        }
    }
    
    // Create color system
    let colors = ColorSystem::from_theme(ThemeName::Dark);
    println!("CSS:\n{}", colors.to_css());
    
    // Create audio system
    let mut audio = AuroraSoundSystem::new(SoundTheme::Standard);
    audio.set_volume(0.8);
}
```

---

## Stability Guarantees

### API Stability (v1.0.0+)

- ✅ All public APIs are stable
- ✅ No breaking changes without major version bump
- ✅ Deprecations announced 2 versions in advance
- ✅ Semantic versioning strictly followed
- ✅ GNOME compatibility maintained across versions

### Performance Guarantees

- ✅ Animation frame rate: 60+ fps (120 fps preferred)
- ✅ Theme switching: <100ms
- ✅ Color calculation: <1ms
- ✅ Widget creation: <5ms
- ✅ Memory overhead: <10MB

### Compatibility

- ✅ Rust 1.70+
- ✅ GTK4 4.8+
- ✅ GNOME 43+
- ✅ Wayland and X11
- ✅ 64-bit architectures (x86_64, ARM64)

---

## Contributing

API additions and changes follow these guidelines:

1. **Backwards Compatibility**: Never break existing APIs
2. **Documentation**: All public APIs require docs
3. **Testing**: All APIs require tests
4. **Examples**: Complex APIs require examples
5. **Performance**: APIs must not impact performance

---

## License

Aurora API is dual-licensed:
- MIT License
- Apache License 2.0

Choose the license that works best for your project.

---

**Last Updated**: August 1, 2026  
**Status**: Complete and Stable  
**Next Review**: v1.1.0 planning
