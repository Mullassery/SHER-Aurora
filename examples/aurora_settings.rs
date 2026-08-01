//! Aurora Settings - System Preferences Example
//!
//! Demonstrates theme switching and settings management with Aurora.

use aurora_color::ThemeName;
use aurora_sound::SoundTheme;

/// Aurora settings manager
pub struct AuroraSettings {
    theme: ThemeName,
    sound_theme: SoundTheme,
    sound_enabled: bool,
    sound_volume: f32,
    high_contrast: bool,
    reduce_motion: bool,
    text_scale: f32,
}

impl AuroraSettings {
    /// Create new settings with defaults
    pub fn new() -> Self {
        Self {
            theme: ThemeName::Light,
            sound_theme: SoundTheme::Standard,
            sound_enabled: true,
            sound_volume: 0.8,
            high_contrast: false,
            reduce_motion: false,
            text_scale: 1.0,
        }
    }

    /// Get current theme
    pub fn theme(&self) -> ThemeName {
        self.theme
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: ThemeName) {
        self.theme = theme;
    }

    /// Get sound theme
    pub fn sound_theme(&self) -> SoundTheme {
        self.sound_theme
    }

    /// Set sound theme
    pub fn set_sound_theme(&mut self, theme: SoundTheme) {
        self.sound_theme = theme;
    }

    /// Check if sound is enabled
    pub fn sound_enabled(&self) -> bool {
        self.sound_enabled
    }

    /// Set sound enabled
    pub fn set_sound_enabled(&mut self, enabled: bool) {
        self.sound_enabled = enabled;
    }

    /// Get sound volume (0.0-1.0)
    pub fn sound_volume(&self) -> f32 {
        self.sound_volume
    }

    /// Set sound volume (0.0-1.0)
    pub fn set_sound_volume(&mut self, volume: f32) {
        self.sound_volume = volume.clamp(0.0, 1.0);
    }

    /// Check if high contrast is enabled
    pub fn high_contrast(&self) -> bool {
        self.high_contrast
    }

    /// Set high contrast
    pub fn set_high_contrast(&mut self, enabled: bool) {
        self.high_contrast = enabled;
    }

    /// Check if motion is reduced
    pub fn reduce_motion(&self) -> bool {
        self.reduce_motion
    }

    /// Set reduce motion
    pub fn set_reduce_motion(&mut self, enabled: bool) {
        self.reduce_motion = enabled;
    }

    /// Get text scale (0.5-2.0)
    pub fn text_scale(&self) -> f32 {
        self.text_scale
    }

    /// Set text scale (0.5-2.0)
    pub fn set_text_scale(&mut self, scale: f32) {
        self.text_scale = scale.clamp(0.5, 2.0);
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        self.theme = ThemeName::Light;
        self.sound_theme = SoundTheme::Standard;
        self.sound_enabled = true;
        self.sound_volume = 0.8;
        self.high_contrast = false;
        self.reduce_motion = false;
        self.text_scale = 1.0;
    }

    /// Save settings (would persist to dconf)
    pub fn save(&self) -> bool {
        // In real implementation, would save to dconf
        true
    }

    /// Load settings (would load from dconf)
    pub fn load(&mut self) -> bool {
        // In real implementation, would load from dconf
        true
    }
}

impl Default for AuroraSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_creation() {
        let settings = AuroraSettings::new();
        assert_eq!(settings.theme(), ThemeName::Light);
        assert!(settings.sound_enabled());
    }

    #[test]
    fn test_theme_switching() {
        let mut settings = AuroraSettings::new();
        settings.set_theme(ThemeName::Dark);
        assert_eq!(settings.theme(), ThemeName::Dark);
    }

    #[test]
    fn test_sound_volume() {
        let mut settings = AuroraSettings::new();
        settings.set_sound_volume(0.5);
        assert_eq!(settings.sound_volume(), 0.5);
    }

    #[test]
    fn test_volume_clamping() {
        let mut settings = AuroraSettings::new();
        settings.set_sound_volume(2.0);
        assert_eq!(settings.sound_volume(), 1.0);
    }

    #[test]
    fn test_accessibility_options() {
        let mut settings = AuroraSettings::new();
        settings.set_high_contrast(true);
        settings.set_reduce_motion(true);
        assert!(settings.high_contrast());
        assert!(settings.reduce_motion());
    }

    #[test]
    fn test_text_scale() {
        let mut settings = AuroraSettings::new();
        settings.set_text_scale(1.5);
        assert_eq!(settings.text_scale(), 1.5);
    }

    #[test]
    fn test_reset() {
        let mut settings = AuroraSettings::new();
        settings.set_theme(ThemeName::Dark);
        settings.set_sound_volume(0.2);
        settings.set_high_contrast(true);

        settings.reset();

        assert_eq!(settings.theme(), ThemeName::Light);
        assert_eq!(settings.sound_volume(), 0.8);
        assert!(!settings.high_contrast());
    }

    #[test]
    fn test_save_load() {
        let settings = AuroraSettings::new();
        assert!(settings.save());

        let mut new_settings = AuroraSettings::new();
        assert!(new_settings.load());
    }

    #[test]
    fn test_all_themes() {
        let mut settings = AuroraSettings::new();
        for theme in &[ThemeName::Light, ThemeName::Dark, ThemeName::OLED, ThemeName::HDR] {
            settings.set_theme(*theme);
            assert_eq!(settings.theme(), *theme);
        }
    }
}
