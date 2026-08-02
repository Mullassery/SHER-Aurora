//! Aurora Sound System
//!
//! A semantic sound design system for GNOME applications.
//! Provides feedback sounds for user interactions with accessibility alternatives.

pub mod feedback;
pub mod sound;

pub use feedback::{Feedback, SoundFeedback};
pub use sound::{Sound, SoundCategory, SoundTheme};

/// Aurora sound system context
#[derive(Debug, Clone)]
pub struct AuroraSoundSystem {
    enabled: bool,
    volume: f32,
    theme: SoundTheme,
}

impl AuroraSoundSystem {
    /// Create a new sound system
    pub fn new() -> Self {
        Self {
            enabled: true,
            volume: 0.8,
            theme: SoundTheme::Subtle,
        }
    }

    /// Check if sound is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable/disable sound
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Get volume (0.0-1.0)
    pub fn volume(&self) -> f32 {
        self.volume.clamp(0.0, 1.0)
    }

    /// Set volume (0.0-1.0)
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Get current sound theme
    pub fn theme(&self) -> SoundTheme {
        self.theme
    }

    /// Set sound theme
    pub fn set_theme(&mut self, theme: SoundTheme) {
        self.theme = theme;
    }

    /// Play a sound
    pub fn play(&self, sound: Sound) {
        if self.enabled {
            // Sound would be played here in actual implementation
            let _ = sound;
        }
    }

    /// Play feedback sound
    pub fn play_feedback(&self, feedback: Feedback) {
        if self.enabled {
            let sound = match feedback {
                Feedback::Success => Sound::Success,
                Feedback::Error => Sound::Error,
                Feedback::Warning => Sound::Warning,
                Feedback::Notification => Sound::Notification,
                Feedback::Click => Sound::Click,
                Feedback::Hover => Sound::Hover,
            };
            self.play(sound);
        }
    }
}

impl Default for AuroraSoundSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_system_creation() {
        let system = AuroraSoundSystem::new();
        assert!(system.is_enabled());
    }

    #[test]
    fn test_sound_system_volume() {
        let mut system = AuroraSoundSystem::new();
        system.set_volume(0.5);
        assert_eq!(system.volume(), 0.5);
    }

    #[test]
    fn test_volume_clamping() {
        let mut system = AuroraSoundSystem::new();
        system.set_volume(1.5);
        assert_eq!(system.volume(), 1.0);

        system.set_volume(-0.5);
        assert_eq!(system.volume(), 0.0);
    }

    #[test]
    fn test_sound_enabled_disabled() {
        let mut system = AuroraSoundSystem::new();
        system.set_enabled(false);
        assert!(!system.is_enabled());

        system.set_enabled(true);
        assert!(system.is_enabled());
    }

    #[test]
    fn test_sound_theme() {
        let mut system = AuroraSoundSystem::new();
        system.set_theme(SoundTheme::Rich);
        assert_eq!(system.theme(), SoundTheme::Rich);
    }

    #[test]
    fn test_play_sound() {
        let system = AuroraSoundSystem::new();
        system.play(Sound::Success);
        // Should not panic
    }

    #[test]
    fn test_play_feedback() {
        let system = AuroraSoundSystem::new();
        system.play_feedback(Feedback::Success);
        // Should not panic
    }

    #[test]
    fn test_default() {
        let system = AuroraSoundSystem::default();
        assert!(system.is_enabled());
        assert!(system.volume() > 0.0);
    }
}
