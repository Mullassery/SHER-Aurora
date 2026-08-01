//! Theme observer for dynamic GNOME theme switching

use aurora_color::ThemeName;

/// Theme change callback type
pub type ThemeChangeCallback = Box<dyn Fn(ThemeName) + Send + Sync>;

/// GNOME theme observer
pub struct ThemeObserver {
    current_theme: ThemeName,
    callbacks: Vec<ThemeChangeCallback>,
    enabled: bool,
}

impl ThemeObserver {
    /// Create a new theme observer
    pub fn new() -> Self {
        Self {
            current_theme: ThemeName::Light,
            callbacks: Vec::new(),
            enabled: true,
        }
    }

    /// Get current theme
    pub fn current_theme(&self) -> ThemeName {
        self.current_theme
    }

    /// Set current theme
    pub fn set_theme(&mut self, theme: ThemeName) {
        if self.current_theme != theme && self.enabled {
            self.current_theme = theme;
            self.notify_callbacks(theme);
        }
    }

    /// Add theme change callback
    pub fn on_theme_change(&mut self, callback: ThemeChangeCallback) {
        self.callbacks.push(callback);
    }

    /// Notify all callbacks of theme change
    fn notify_callbacks(&self, theme: ThemeName) {
        for callback in &self.callbacks {
            callback(theme);
        }
    }

    /// Enable observer
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable observer
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if observer is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Listen for GNOME Settings theme changes
    /// Returns true if successful
    pub fn start_listening(&mut self) -> bool {
        // In a real implementation, this would connect to:
        // - org.freedesktop.Appearance.ColorSchemeChanged signal
        // - dconf changes at /org/gnome/desktop/interface/aurora/theme
        // For now, return success to indicate readiness
        self.enabled = true;
        true
    }

    /// Stop listening for theme changes
    pub fn stop_listening(&mut self) {
        self.disable();
    }

    /// Get callback count
    pub fn callback_count(&self) -> usize {
        self.callbacks.len()
    }

    /// Clear all callbacks
    pub fn clear_callbacks(&mut self) {
        self.callbacks.clear();
    }
}

impl Default for ThemeObserver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_creation() {
        let observer = ThemeObserver::new();
        assert_eq!(observer.current_theme(), ThemeName::Light);
        assert!(observer.is_enabled());
    }

    #[test]
    fn test_set_theme() {
        let mut observer = ThemeObserver::new();
        observer.set_theme(ThemeName::Dark);
        assert_eq!(observer.current_theme(), ThemeName::Dark);
    }

    #[test]
    fn test_add_callback() {
        let mut observer = ThemeObserver::new();
        let callback: ThemeChangeCallback = Box::new(|_theme| {});
        observer.on_theme_change(callback);
        assert_eq!(observer.callback_count(), 1);
    }

    #[test]
    fn test_disable_prevents_theme_change() {
        let mut observer = ThemeObserver::new();
        observer.disable();
        observer.set_theme(ThemeName::Dark);
        // Theme should not change when disabled
        assert_eq!(observer.current_theme(), ThemeName::Light);
    }

    #[test]
    fn test_enable_allows_theme_change() {
        let mut observer = ThemeObserver::new();
        observer.disable();
        observer.enable();
        observer.set_theme(ThemeName::Dark);
        assert_eq!(observer.current_theme(), ThemeName::Dark);
    }

    #[test]
    fn test_clear_callbacks() {
        let mut observer = ThemeObserver::new();
        let callback1: ThemeChangeCallback = Box::new(|_| {});
        let callback2: ThemeChangeCallback = Box::new(|_| {});
        observer.on_theme_change(callback1);
        observer.on_theme_change(callback2);
        assert_eq!(observer.callback_count(), 2);

        observer.clear_callbacks();
        assert_eq!(observer.callback_count(), 0);
    }

    #[test]
    fn test_start_listening() {
        let mut observer = ThemeObserver::new();
        assert!(observer.start_listening());
        assert!(observer.is_enabled());
    }

    #[test]
    fn test_stop_listening() {
        let mut observer = ThemeObserver::new();
        observer.stop_listening();
        assert!(!observer.is_enabled());
    }

    #[test]
    fn test_default() {
        let observer = ThemeObserver::default();
        assert_eq!(observer.current_theme(), ThemeName::Light);
        assert!(observer.is_enabled());
    }
}
