/// Aurora sound types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sound {
    Success,
    Error,
    Warning,
    Notification,
    Click,
    Hover,
}

/// Aurora sound categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundCategory {
    Confirmation,
    Attention,
    Notification,
    Interface,
}

/// Aurora sound themes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundTheme {
    Subtle,
    Standard,
    Rich,
}

impl Sound {
    /// Get sound category
    pub fn category(&self) -> SoundCategory {
        match self {
            Sound::Success => SoundCategory::Confirmation,
            Sound::Error => SoundCategory::Attention,
            Sound::Warning => SoundCategory::Attention,
            Sound::Notification => SoundCategory::Notification,
            Sound::Click => SoundCategory::Interface,
            Sound::Hover => SoundCategory::Interface,
        }
    }

    /// Get sound file name
    pub fn filename(&self) -> &'static str {
        match self {
            Sound::Success => "success.ogg",
            Sound::Error => "error.ogg",
            Sound::Warning => "warning.ogg",
            Sound::Notification => "notification.ogg",
            Sound::Click => "click.ogg",
            Sound::Hover => "hover.ogg",
        }
    }

    /// Get sound duration in milliseconds
    pub fn duration_ms(&self) -> u32 {
        match self {
            Sound::Success => 400,
            Sound::Error => 500,
            Sound::Warning => 450,
            Sound::Notification => 300,
            Sound::Click => 50,
            Sound::Hover => 30,
        }
    }

    /// Get volume for theme
    pub fn volume_for_theme(&self, theme: SoundTheme) -> f32 {
        match theme {
            SoundTheme::Subtle => 0.3,
            SoundTheme::Standard => 0.6,
            SoundTheme::Rich => 0.9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_category() {
        assert_eq!(Sound::Success.category(), SoundCategory::Confirmation);
        assert_eq!(Sound::Error.category(), SoundCategory::Attention);
        assert_eq!(Sound::Click.category(), SoundCategory::Interface);
    }

    #[test]
    fn test_sound_filename() {
        assert_eq!(Sound::Success.filename(), "success.ogg");
        assert_eq!(Sound::Error.filename(), "error.ogg");
    }

    #[test]
    fn test_sound_duration() {
        assert!(Sound::Success.duration_ms() > 0);
        assert!(Sound::Click.duration_ms() < Sound::Success.duration_ms());
    }

    #[test]
    fn test_volume_for_theme() {
        let success = Sound::Success;
        assert!(
            success.volume_for_theme(SoundTheme::Subtle)
                < success.volume_for_theme(SoundTheme::Standard)
        );
        assert!(
            success.volume_for_theme(SoundTheme::Standard)
                < success.volume_for_theme(SoundTheme::Rich)
        );
    }

    #[test]
    fn test_all_sounds() {
        let sounds = vec![
            Sound::Success,
            Sound::Error,
            Sound::Warning,
            Sound::Notification,
            Sound::Click,
            Sound::Hover,
        ];

        for sound in sounds {
            assert!(!sound.filename().is_empty());
            assert!(sound.duration_ms() > 0);
        }
    }
}
