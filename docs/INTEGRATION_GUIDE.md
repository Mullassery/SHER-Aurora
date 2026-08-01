# Aurora GNOME Integration Guide

**Version**: v1.0.0  
**Target Audience**: GNOME Application Developers  
**Status**: Complete

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Installation](#installation)
3. [Basic Setup](#basic-setup)
4. [Theming Your App](#theming-your-app)
5. [Using Components](#using-components)
6. [Settings Integration](#settings-integration)
7. [Sound Integration](#sound-integration)
8. [Accessibility](#accessibility)
9. [Best Practices](#best-practices)
10. [Examples](#examples)

---

## Getting Started

Aurora transforms GNOME applications with professional-grade design, animation, and accessibility. This guide walks you through integrating Aurora into your GNOME app.

### What Aurora Provides

- **Design System**: Tokens, typography, color system
- **Component Library**: 10 GTK4 widgets
- **Animations**: Spring physics-based motion engine
- **Accessibility**: WCAG AAA compliance by default
- **Sound Design**: Notification and interaction sounds
- **GNOME Integration**: dconf settings, Settings app panel, theme observer

### Minimum Requirements

- Rust 1.70+
- GTK4 4.8+
- libadwaita 1.2+
- GNOME 43+
- Linux kernel 5.10+

---

## Installation

### 1. Add Aurora Dependencies to Cargo.toml

```toml
[dependencies]
# Aurora subsystems
aurora-tokens = "1.0"
aurora-typography = "1.0"
aurora-color = "1.0"
aurora-motion = "1.0"
aurora-sound = "1.0"
aurora-gtk = "1.0"

# GNOME ecosystem
gtk4 = { version = "0.9", features = ["v4_10"] }
gtk4-macros = "0.1"
glib = "0.19"
libadwaita = "0.5"
gio = "0.19"
```

### 2. Install dconf Schema

Copy Aurora's dconf schema to your system:

```bash
# Copy schema file
sudo cp schemas/org.gnome.desktop.interface.aurora.gschema.xml \
    /usr/share/glib-2.0/schemas/

# Compile schema
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/
```

Verify installation:

```bash
gsettings list-schemas | grep aurora
# Should output: org.gnome.desktop.interface.aurora
```

---

## Basic Setup

### Minimal Application

```rust
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use aurora_gtk::AuroraGtk;
use aurora_color::ThemeName;

fn main() {
    let app = Application::builder()
        .application_id("com.example.aurora-app")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    // Initialize Aurora
    let aurora = AuroraGtk::new(aurora_gtk::Theme::Light)
        .expect("Failed to initialize Aurora");

    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .title("Aurora Application")
        .build();

    // Load Aurora CSS
    // (In production: use GTK's CSS provider)
    
    window.present();
}
```

### With libadwaita

```rust
use libadwaita::{Application, ApplicationWindow, adw};

fn main() {
    let app = Application::builder()
        .application_id("com.example.aurora-app")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .build();

        window.present();
    });

    app.run();
}
```

---

## Theming Your App

### Dynamic Theme Switching

```rust
use aurora_gtk::gnome::ThemeObserver;
use aurora_color::{ColorSystem, ThemeName};

fn setup_theme_observer() {
    let mut observer = ThemeObserver::new();
    
    // Listen for theme changes
    observer.on_theme_change(Box::new(|theme| {
        println!("Theme changed to: {:?}", theme);
        apply_theme(theme);
    }));
    
    observer.start_listening();
}

fn apply_theme(theme: ThemeName) {
    let colors = ColorSystem::from_theme(theme);
    
    // Update application colors
    // Apply to GTK CSS provider, update UI state, etc.
    
    println!("Applied theme CSS:\n{}", colors.to_css());
}
```

### Respecting System Preferences

```rust
use gio::Settings;

fn get_user_theme() -> ThemeName {
    // Read from dconf
    match Settings::new("org.gnome.desktop.interface.aurora")
        .string("theme")
        .as_str()
    {
        "dark" => ThemeName::Dark,
        "oled" => ThemeName::OLED,
        "hdr" => ThemeName::HDR,
        _ => ThemeName::Light,
    }
}
```

---

## Using Components

### Button Component

```rust
use aurora_gtk::widgets::{Button, ButtonStyle, ButtonState};

// Create a filled primary button
let button = Button::new("Click Me", ButtonStyle::Filled);
assert_eq!(button.state(), ButtonState::Default);

// Change button state
let mut btn = button;
btn.set_state(ButtonState::Active);

// Apply CSS class
let css_class = btn.css_class();
// "aurora-button aurora-button-filled aurora-button-active"
```

### Card Component

```rust
use aurora_gtk::widgets::{Card, CardStyle};

let card = Card::new(CardStyle::Elevated);
let mut card = card;
card.set_spacing(16);  // 16px spacing
card.set_margin(8);    // 8px margin

println!("{}", card.css_class());
// "aurora-card aurora-card-elevated"
```

### Input Component

```rust
use aurora_gtk::widgets::{Input, InputType};

let mut email = Input::new(InputType::Email);
email.set_placeholder("your@email.com");

let text = email.get_text();
println!("User entered: {}", text);

// Set error state
email.set_error(Some("Invalid email format"));
```

### Dialog Component

```rust
use aurora_gtk::widgets::{AuroraDialog, DialogResponse};

let mut dialog = AuroraDialog::new(
    "Delete File?",
    "Are you sure you want to delete this file?"
);

dialog.add_button("Delete", DialogResponse::Primary);
dialog.add_button("Cancel", DialogResponse::Cancel);

match dialog.show() {
    DialogResponse::Primary => {
        // Delete the file
    }
    _ => {
        // User cancelled
    }
}
```

### All 10 Widgets

| Widget | Use Case | Example |
|--------|----------|---------|
| Button | Primary actions | "Save", "Send", "Delete" |
| Card | Content containers | Blog posts, settings panels |
| Input | Text entry | Search, forms, filters |
| Dialog | Confirmations | Delete, save, quit dialogs |
| Checkbox | Multiple selection | Preferences, filters |
| RadioButton | Single selection | Options, variants |
| Tooltip | Help text | "Click to save" on hover |
| List | Item collections | File listings, playlists |
| Badge | Status indicators | "New", "Updated", errors |
| Sidebar | Navigation | Collapsible app sections |

---

## Settings Integration

### Create a Settings App

```rust
use aurora_gtk::gnome::SettingsPanel;

fn show_settings() {
    let panel = SettingsPanel::new("My App Settings");
    
    // Get sections
    for section in panel.sections() {
        println!("Section: {}", section.title);
        for setting in &section.settings {
            println!("  - {} ({})", setting.label, setting.key);
        }
    }
    
    // Generate UI
    let html = panel.to_html();
    println!("{}", html);
}

// Output:
// Section: Appearance
//   - Color Scheme (theme)
//   - High Contrast (high-contrast)
//   - Text Scaling (text-scale)
// Section: Sound
//   - Sound Feedback (sound-enabled)
//   - Volume (sound-volume)
//   - Sound Theme (sound-theme)
// Section: Accessibility
//   - Reduce Motion (reduce-motion)
```

### Persist to dconf

```rust
use gio::Settings;

fn save_settings(theme: &str, volume: f32) {
    let settings = Settings::new("org.gnome.desktop.interface.aurora");
    
    // Save theme
    settings.set_string("theme", theme)
        .expect("Failed to save theme");
    
    // Save volume
    settings.set_double("sound-volume", volume as f64)
        .expect("Failed to save volume");
}

fn load_settings() -> (String, f32) {
    let settings = Settings::new("org.gnome.desktop.interface.aurora");
    
    let theme = settings.string("theme").to_string();
    let volume = settings.double("sound-volume") as f32;
    
    (theme, volume)
}
```

---

## Sound Integration

### Play Notification Sounds

```rust
use aurora_sound::{Sound, AuroraSoundSystem, SoundTheme};

fn setup_audio() {
    let mut audio = AuroraSoundSystem::new(SoundTheme::Standard);
    audio.set_volume(0.8);
    audio.set_enabled(true);
    
    // Play sounds on events
    audio.play(Sound::Success);      // Operation completed
    audio.play(Sound::Error);        // Operation failed
    audio.play(Sound::Warning);      // Caution needed
    audio.play(Sound::Click);        // Button clicked
}
```

### Theme-Aware Notifications

```rust
use aurora_gtk::gnome::{AuroraNotification, NotificationUrgency};
use aurora_color::ThemeName;

fn notify_user() {
    let notif = AuroraNotification::new("File Saved")
        .with_body("Your changes have been saved.")
        .with_urgency(NotificationUrgency::Low)
        .with_theme(ThemeName::Dark)
        .with_timeout(3000);  // 3 seconds
    
    println!("{}", notif.to_css());
}
```

---

## Accessibility

### WCAG AAA Compliance

All Aurora colors are verified for WCAG AAA contrast:

```rust
use aurora_color::Color;

let primary = Color::from_hex("#003D99")?;
let surface = Color::from_hex("#FFFFFF")?;

// Verify contrast ratios
assert!(primary.passes_wcag_aa(&surface));   // 4.5:1
assert!(primary.passes_wcag_aaa(&surface));  // 7:1

// Calculate exact ratio
let ratio = primary.contrast_ratio(&surface);
println!("Contrast ratio: {:.1}:1", ratio);  // "7.8:1"
```

### Respecting Preferences

```rust
use gio::Settings;

fn respect_a11y_preferences() {
    let settings = Settings::new("org.gnome.desktop.interface.aurora");
    
    // Reduce motion
    let reduce_motion = settings.boolean("reduce-motion");
    if reduce_motion {
        disable_animations();
    }
    
    // High contrast
    let high_contrast = settings.boolean("high-contrast");
    if high_contrast {
        apply_high_contrast_theme();
    }
    
    // Text scaling
    let text_scale = settings.double("text-scale");
    if text_scale != 1.0 {
        apply_text_scale(text_scale as f32);
    }
}
```

### Keyboard Navigation

```rust
// Aurora components are keyboard-navigable by default
// - Tab: Move between widgets
// - Shift+Tab: Move backwards
// - Enter/Space: Activate button
// - Arrow keys: Navigate lists, radio buttons
// - Escape: Close dialogs
```

---

## Best Practices

### 1. Initialize Aurora Early

```rust
// In your main function
fn main() {
    let app = Application::new(Some("com.example.app"));
    
    app.connect_activate(|app| {
        // Initialize Aurora immediately
        let _aurora = AuroraGtk::new(Theme::Light)
            .expect("Aurora initialization failed");
        
        build_ui(app);
    });
    
    app.run();
}
```

### 2. Use Semantic Colors

```rust
// ✅ Good: Use semantic names
let primary = color_system.primary;
let success = color_system.success;

// ❌ Avoid: Hardcoded colors
let color = Color::from_hex("#0066CC")?;
```

### 3. Respect User Preferences

```rust
// ✅ Good: Always respect system settings
let theme = get_user_theme();
apply_theme(theme);

// ❌ Avoid: Force a specific theme
apply_theme(ThemeName::Dark);  // Ignores user preference
```

### 4. Performance Optimization

```rust
// ✅ Good: Cache CSS and color systems
let colors = ColorSystem::from_theme(theme);
let css = colors.to_css();  // Generate once, reuse

// ❌ Avoid: Regenerating on every event
fn on_event() {
    let colors = ColorSystem::from_theme(theme);  // Inefficient
    let css = colors.to_css();  // Generates every time
}
```

### 5. Test Accessibility

```rust
// ✅ Good: Verify contrast ratios
#[test]
fn test_color_contrast() {
    let primary = ColorSystem::from_theme(ThemeName::Light).primary;
    let background = ColorSystem::from_theme(ThemeName::Light).background;
    
    assert!(primary.passes_wcag_aaa(&background));
}

// ❌ Avoid: Assuming colors work without testing
```

---

## Examples

### Example 1: Simple Counter App

```rust
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Label};
use aurora_gtk::AuroraGtk;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app = Application::builder()
        .application_id("com.example.counter")
        .build();
    
    app.connect_activate(|app| {
        let counter = Rc::new(RefCell::new(0));
        
        let label = Label::new(Some("0"));
        
        let inc_button = Button::with_label("Increment");
        let counter_clone = Rc::clone(&counter);
        inc_button.connect_clicked(move |_| {
            let mut count = counter_clone.borrow_mut();
            *count += 1;
            label.set_text(&count.to_string());
        });
        
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Aurora Counter")
            .default_width(300)
            .default_height(150)
            .build();
        
        let container = Box::new(gtk4::Orientation::Vertical, 12);
        container.append(&label);
        container.append(&inc_button);
        window.set_child(Some(&container));
        
        window.present();
    });
    
    app.run();
}
```

### Example 2: Settings App with Theme Switching

See `examples/aurora_settings.rs` in the Aurora repository.

### Example 3: File Browser

See `examples/aurora_files.rs` in the Aurora repository.

---

## Troubleshooting

### Theme Not Applying

**Problem**: Application doesn't use Aurora colors
**Solution**:
1. Verify dconf schema is installed: `gsettings list-schemas | grep aurora`
2. Ensure CSS provider is loaded
3. Check GTK theme inheritance

### Animations Too Slow

**Problem**: Animations feel sluggish
**Solution**:
1. Enable GPU acceleration: `GSK_RENDERER=gl`
2. Check system load: `top`
3. Profile with: `GSK_DEBUG=sync`

### Accessibility Issues

**Problem**: Text too small or colors unclear
**Solution**:
1. Check `reduce-motion` setting
2. Verify `text-scale` setting
3. Run contrast checker on colors

---

## Next Steps

1. **Clone the Examples**: Start with `examples/aurora_settings.rs`
2. **Read the API Docs**: Check `docs/API_REFERENCE.md`
3. **Join the Community**: Contribute to Aurora on GitHub
4. **File Issues**: Report bugs and request features

---

## Support

- **Documentation**: https://github.com/Mullassery/aurora/tree/main/docs
- **Issues**: https://github.com/Mullassery/aurora/issues
- **Discussions**: GNOME Discourse + GitHub Discussions
- **Email**: mullassery@gmail.com

---

**Integration Guide v1.0** | Last Updated: August 1, 2026
