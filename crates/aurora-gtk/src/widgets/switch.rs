/// Aurora Switch (toggle) component
///
/// A binary on/off toggle control, the canonical GNOME/GTK4 alternative to a
/// checkbox for settings that take effect immediately.
#[derive(Debug, Clone)]
pub struct Switch {
    active: bool,
    sensitive: bool,
    css_classes: Vec<String>,
}

impl Switch {
    /// Create a new switch, off by default
    pub fn new() -> Self {
        Self {
            active: false,
            sensitive: true,
            css_classes: vec!["aurora-switch".to_string()],
        }
    }

    /// Set the active (on/off) state
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Get the active state
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Enable/disable the switch
    pub fn set_sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = sensitive;
        self
    }

    /// Check if sensitive
    pub fn is_sensitive(&self) -> bool {
        self.sensitive
    }

    /// Add CSS class
    pub fn add_css_class(mut self, class: &str) -> Self {
        self.css_classes.push(class.to_string());
        self
    }

    /// Get CSS classes
    pub fn css_classes(&self) -> &[String] {
        &self.css_classes
    }

    /// Build a real `gtk4::Switch` widget from this descriptor.
    ///
    /// Constructs an actual GTK4 switch widget: active state, sensitivity,
    /// and Aurora CSS classes are applied through the real `gtk4` widget
    /// API. Callers must have already initialized GTK before calling this.
    pub fn build(&self) -> gtk4::Switch {
        use gtk4::prelude::*;

        let switch = gtk4::Switch::builder()
            .active(self.active)
            .state(self.active)
            .sensitive(self.sensitive)
            .build();

        for class in &self.css_classes {
            switch.add_css_class(class);
        }

        switch
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_new() {
        let switch = Switch::new();
        assert!(!switch.is_active());
    }

    #[test]
    fn test_switch_active() {
        let switch = Switch::new().active(true);
        assert!(switch.is_active());
    }

    #[test]
    fn test_switch_sensitive() {
        let switch = Switch::new().set_sensitive(false);
        assert!(!switch.is_sensitive());
    }

    #[test]
    fn test_switch_css_class() {
        let switch = Switch::new().add_css_class("custom");
        assert!(switch.css_classes().contains(&"custom".to_string()));
    }

    #[test]
    fn test_switch_default() {
        let switch = Switch::default();
        assert!(!switch.is_active());
        assert!(switch.is_sensitive());
    }

    #[test]
    fn test_switch_chaining() {
        let _switch = Switch::new()
            .active(true)
            .set_sensitive(true)
            .add_css_class("test");
    }

    // Real GTK4 widget-construction tests. These call into the actual
    // `gtk4` crate and require a real, initialized GTK4 instance. GTK4's
    // Cocoa backend requires `gtk4::init()` to run on the process's true
    // OS main thread, which Rust's default multi-threaded `#[test]` runner
    // cannot provide on macOS (verified empirically: `gtk4::init()` called
    // from any spawned/worker thread panics with "Attempted to initialize
    // GTK on OSX from non-main thread", and `#[gtk4::test]`'s worker-thread
    // pool hits the same wall). On Linux (X11/Wayland — this is exactly
    // what CI's ubuntu-latest runners use) this restriction does not exist,
    // so these run for real there. The equivalent real-GTK4 proof for local
    // macOS development is `cargo run --example gtk4_harness -p aurora-gtk`,
    // which runs on the true process main thread.
    #[cfg(not(target_os = "macos"))]
    mod gtk_real {
        use super::*;

        #[gtk4::test]
        fn test_switch_build_is_real_gtk4_widget() {
            use gtk4::prelude::*;
            let switch = Switch::new().active(true).build();
            assert!(switch.is_active());
            assert!(switch.css_classes().iter().any(|c| c == "aurora-switch"));
        }
    }
}
