use serde::{Deserialize, Serialize};

/// Aurora theme for GTK
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    OLED,
    HDR,
}

impl Theme {
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::OLED => "OLED",
            Theme::HDR => "HDR",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Theme::Light => "aurora-light",
            Theme::Dark => "aurora-dark",
            Theme::OLED => "aurora-oled",
            Theme::HDR => "aurora-hdr",
        }
    }
}

/// The OS-reported theme preference, separated from any live GTK object so
/// the Light/Dark selection is pure and unit-testable without a display.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ThemePreference {
    /// The OS/GTK "prefer dark" setting.
    pub dark: bool,
    /// A high-contrast GTK theme is active.
    ///
    /// Aurora's token set has no dedicated high-contrast palette yet, so this
    /// is tracked (and its notify signal triggers a refresh) but does not by
    /// itself change the selected palette. See `ROADMAP_HONEST.md`.
    pub high_contrast: bool,
}

impl ThemePreference {
    pub fn new(dark: bool, high_contrast: bool) -> Self {
        Self {
            dark,
            high_contrast,
        }
    }

    /// Derives the preference from GTK's real settings values: the
    /// `gtk-application-prefer-dark-theme` flag and the `gtk-theme-name`
    /// string (used to detect GNOME's `HighContrast`/`HighContrastInverse`
    /// themes).
    pub fn from_gtk_settings(prefer_dark: bool, theme_name: Option<&str>) -> Self {
        Self {
            dark: prefer_dark,
            high_contrast: theme_name.is_some_and(is_high_contrast_theme_name),
        }
    }

    /// Selects an existing Aurora theme from the preference. High contrast is
    /// recorded but has no separate token palette yet.
    pub fn theme(self) -> Theme {
        if self.dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

/// Matches theme names like `HighContrast`, `HighContrastInverse`, and
/// `high-contrast` while ignoring case and separators.
fn is_high_contrast_theme_name(name: &str) -> bool {
    let normalized: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    normalized.contains("highcontrast")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_preference_selects_light() {
        let preference = ThemePreference::from_gtk_settings(false, Some("Adwaita"));
        assert_eq!(preference.theme(), Theme::Light);
        assert!(!preference.high_contrast);
    }

    #[test]
    fn dark_preference_selects_dark() {
        let preference = ThemePreference::from_gtk_settings(true, Some("Adwaita-dark"));
        assert_eq!(preference.theme(), Theme::Dark);
    }

    #[test]
    fn high_contrast_theme_names_are_detected() {
        for name in [
            "HighContrast",
            "HighContrastInverse",
            "high-contrast",
            "HIGH_CONTRAST_DARK",
        ] {
            assert!(
                ThemePreference::from_gtk_settings(false, Some(name)).high_contrast,
                "{name} should be detected as high contrast"
            );
        }
        assert!(!ThemePreference::from_gtk_settings(false, Some("Adwaita")).high_contrast);
        assert!(!ThemePreference::from_gtk_settings(false, None).high_contrast);
    }

    #[test]
    fn preference_changes_select_a_different_theme() {
        let light = ThemePreference::new(false, false);
        let dark = ThemePreference::new(true, false);
        assert_eq!(light.theme(), Theme::Light);
        assert_eq!(dark.theme(), Theme::Dark);
        assert_ne!(light.theme(), dark.theme());
    }
}
