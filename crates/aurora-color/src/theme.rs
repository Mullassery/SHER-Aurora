use crate::color::Color;

/// Aurora theme variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeName {
    Light,
    Dark,
    OLED,
    HDR,
}

/// Aurora color theme
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: ThemeName,
}

impl Theme {
    /// Create a new theme
    pub fn new(name: ThemeName) -> Self {
        Self { name }
    }

    /// Get theme name
    pub fn name(&self) -> ThemeName {
        self.name
    }
}

/// Aurora color system palette
#[derive(Debug, Clone)]
pub struct ColorSystem {
    pub primary: Color,
    pub primary_container: Color,
    pub secondary: Color,
    pub secondary_container: Color,
    pub accent: Color,
    pub error: Color,
    pub error_container: Color,
    pub warning: Color,
    pub warning_container: Color,
    pub success: Color,
    pub success_container: Color,
    pub info: Color,
    pub info_container: Color,
    pub surface: Color,
    pub surface_variant: Color,
    pub surface_dim: Color,
    pub background: Color,
    pub foreground: Color,
    pub foreground_secondary: Color,
    pub foreground_tertiary: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub theme: ThemeName,
}

impl ColorSystem {
    /// Create color system from theme
    pub fn from_theme(theme: ThemeName) -> Self {
        match theme {
            ThemeName::Light => Self::light(),
            ThemeName::Dark => Self::dark(),
            ThemeName::OLED => Self::oled(),
            ThemeName::HDR => Self::hdr(),
        }
    }

    /// Light theme palette
    fn light() -> Self {
        Self {
            primary: Color::from_hex("#003D99").expect("valid built-in hex constant"),
            primary_container: Color::from_hex("#E3F2FD").expect("valid built-in hex constant"),
            secondary: Color::from_hex("#440099").expect("valid built-in hex constant"),
            secondary_container: Color::from_hex("#F3E5F5").expect("valid built-in hex constant"),
            accent: Color::from_hex("#AA0044").expect("valid built-in hex constant"),
            error: Color::from_hex("#990000").expect("valid built-in hex constant"),
            error_container: Color::from_hex("#FFEBEE").expect("valid built-in hex constant"),
            warning: Color::from_hex("#994400").expect("valid built-in hex constant"),
            warning_container: Color::from_hex("#FFF8E1").expect("valid built-in hex constant"),
            success: Color::from_hex("#004400").expect("valid built-in hex constant"),
            success_container: Color::from_hex("#E8F5E9").expect("valid built-in hex constant"),
            info: Color::from_hex("#0066CC").expect("valid built-in hex constant"),
            info_container: Color::from_hex("#E3F2FD").expect("valid built-in hex constant"),
            surface: Color::from_hex("#FEFEFE").expect("valid built-in hex constant"),
            surface_variant: Color::from_hex("#F5F5F5").expect("valid built-in hex constant"),
            surface_dim: Color::from_hex("#F0F0F0").expect("valid built-in hex constant"),
            background: Color::from_hex("#FFFFFF").expect("valid built-in hex constant"),
            foreground: Color::from_hex("#1A1A1A").expect("valid built-in hex constant"),
            foreground_secondary: Color::from_hex("#333333").expect("valid built-in hex constant"),
            foreground_tertiary: Color::from_hex("#666666").expect("valid built-in hex constant"),
            // #949494 clears the WCAG 1.4.11 non-text contrast minimum (3:1) against both
            // surface and background; the previous #CCCCCC only reached ~1.6:1.
            outline: Color::from_hex("#949494").expect("valid built-in hex constant"),
            outline_variant: Color::from_hex("#DDDDDD").expect("valid built-in hex constant"),
            theme: ThemeName::Light,
        }
    }

    /// Dark theme palette
    fn dark() -> Self {
        Self {
            primary: Color::from_hex("#6EB7FF").expect("valid built-in hex constant"),
            // Container tones below are darkened from their original values so the
            // semantic color rendered on top (e.g. `primary` on `primary_container`)
            // clears real WCAG AAA text contrast (7:1) instead of falling to ~2-4:1.
            primary_container: Color::from_hex("#052456").expect("valid built-in hex constant"),
            secondary: Color::from_hex("#C5B3FF").expect("valid built-in hex constant"),
            secondary_container: Color::from_hex("#470D7A").expect("valid built-in hex constant"),
            accent: Color::from_hex("#FF80AB").expect("valid built-in hex constant"),
            error: Color::from_hex("#F8A29A").expect("valid built-in hex constant"),
            error_container: Color::from_hex("#58130F").expect("valid built-in hex constant"),
            warning: Color::from_hex("#FFD54F").expect("valid built-in hex constant"),
            warning_container: Color::from_hex("#693500").expect("valid built-in hex constant"),
            success: Color::from_hex("#81C784").expect("valid built-in hex constant"),
            success_container: Color::from_hex("#0E3211").expect("valid built-in hex constant"),
            info: Color::from_hex("#64B5F6").expect("valid built-in hex constant"),
            info_container: Color::from_hex("#06224D").expect("valid built-in hex constant"),
            surface: Color::from_hex("#1E1E1E").expect("valid built-in hex constant"),
            surface_variant: Color::from_hex("#2D2D2D").expect("valid built-in hex constant"),
            surface_dim: Color::from_hex("#121212").expect("valid built-in hex constant"),
            background: Color::from_hex("#121212").expect("valid built-in hex constant"),
            foreground: Color::from_hex("#F5F5F5").expect("valid built-in hex constant"),
            foreground_secondary: Color::from_hex("#BDBDBD").expect("valid built-in hex constant"),
            foreground_tertiary: Color::from_hex("#9E9E9E").expect("valid built-in hex constant"),
            // #696969 clears the WCAG 1.4.11 non-text contrast minimum (3:1) against both
            // surface and background; the previous #424242 only reached ~1.6-1.9:1.
            outline: Color::from_hex("#696969").expect("valid built-in hex constant"),
            outline_variant: Color::from_hex("#333333").expect("valid built-in hex constant"),
            theme: ThemeName::Dark,
        }
    }

    /// OLED theme palette (true black)
    fn oled() -> Self {
        Self {
            primary: Color::from_hex("#6EB7FF").expect("valid built-in hex constant"),
            primary_container: Color::from_hex("#052456").expect("valid built-in hex constant"),
            secondary: Color::from_hex("#C5B3FF").expect("valid built-in hex constant"),
            secondary_container: Color::from_hex("#470D7A").expect("valid built-in hex constant"),
            accent: Color::from_hex("#FF80AB").expect("valid built-in hex constant"),
            error: Color::from_hex("#F8A29A").expect("valid built-in hex constant"),
            error_container: Color::from_hex("#58130F").expect("valid built-in hex constant"),
            warning: Color::from_hex("#FFD54F").expect("valid built-in hex constant"),
            warning_container: Color::from_hex("#693500").expect("valid built-in hex constant"),
            success: Color::from_hex("#81C784").expect("valid built-in hex constant"),
            success_container: Color::from_hex("#0E3211").expect("valid built-in hex constant"),
            info: Color::from_hex("#64B5F6").expect("valid built-in hex constant"),
            info_container: Color::from_hex("#06224D").expect("valid built-in hex constant"),
            surface: Color::from_hex("#0D0D0D").expect("valid built-in hex constant"),
            surface_variant: Color::from_hex("#1A1A1A").expect("valid built-in hex constant"),
            surface_dim: Color::from_hex("#000000").expect("valid built-in hex constant"),
            background: Color::from_hex("#000000").expect("valid built-in hex constant"),
            foreground: Color::from_hex("#F5F5F5").expect("valid built-in hex constant"),
            foreground_secondary: Color::from_hex("#BDBDBD").expect("valid built-in hex constant"),
            foreground_tertiary: Color::from_hex("#9E9E9E").expect("valid built-in hex constant"),
            // #5F5F5F clears the WCAG 1.4.11 non-text contrast minimum (3:1) against both
            // surface and background; the previous #2D2D2D only reached ~1.4-1.5:1.
            outline: Color::from_hex("#5F5F5F").expect("valid built-in hex constant"),
            outline_variant: Color::from_hex("#1A1A1A").expect("valid built-in hex constant"),
            theme: ThemeName::OLED,
        }
    }

    /// HDR theme palette (extended color gamut)
    fn hdr() -> Self {
        Self {
            primary: Color::from_hex("#6EB7FF").expect("valid built-in hex constant"),
            primary_container: Color::from_hex("#052456").expect("valid built-in hex constant"),
            secondary: Color::from_hex("#C5B3FF").expect("valid built-in hex constant"),
            secondary_container: Color::from_hex("#470D7A").expect("valid built-in hex constant"),
            accent: Color::from_hex("#FF80AB").expect("valid built-in hex constant"),
            error: Color::from_hex("#F8A29A").expect("valid built-in hex constant"),
            error_container: Color::from_hex("#58130F").expect("valid built-in hex constant"),
            warning: Color::from_hex("#FFD54F").expect("valid built-in hex constant"),
            warning_container: Color::from_hex("#693500").expect("valid built-in hex constant"),
            success: Color::from_hex("#81C784").expect("valid built-in hex constant"),
            success_container: Color::from_hex("#0E3211").expect("valid built-in hex constant"),
            info: Color::from_hex("#64B5F6").expect("valid built-in hex constant"),
            info_container: Color::from_hex("#06224D").expect("valid built-in hex constant"),
            surface: Color::from_hex("#1E1E1E").expect("valid built-in hex constant"),
            surface_variant: Color::from_hex("#2D2D2D").expect("valid built-in hex constant"),
            surface_dim: Color::from_hex("#121212").expect("valid built-in hex constant"),
            background: Color::from_hex("#121212").expect("valid built-in hex constant"),
            foreground: Color::from_hex("#F5F5F5").expect("valid built-in hex constant"),
            foreground_secondary: Color::from_hex("#BDBDBD").expect("valid built-in hex constant"),
            foreground_tertiary: Color::from_hex("#9E9E9E").expect("valid built-in hex constant"),
            outline: Color::from_hex("#696969").expect("valid built-in hex constant"),
            outline_variant: Color::from_hex("#333333").expect("valid built-in hex constant"),
            theme: ThemeName::HDR,
        }
    }

    /// Generate CSS custom properties
    pub fn to_css(&self) -> String {
        format!(
            r#":root {{
  --color-primary: {};
  --color-primary-container: {};
  --color-secondary: {};
  --color-secondary-container: {};
  --color-accent: {};
  --color-error: {};
  --color-error-container: {};
  --color-warning: {};
  --color-warning-container: {};
  --color-success: {};
  --color-success-container: {};
  --color-info: {};
  --color-info-container: {};
  --color-surface: {};
  --color-surface-variant: {};
  --color-surface-dim: {};
  --color-background: {};
  --color-foreground: {};
  --color-foreground-secondary: {};
  --color-foreground-tertiary: {};
  --color-outline: {};
  --color-outline-variant: {};
}}"#,
            self.primary.to_hex(),
            self.primary_container.to_hex(),
            self.secondary.to_hex(),
            self.secondary_container.to_hex(),
            self.accent.to_hex(),
            self.error.to_hex(),
            self.error_container.to_hex(),
            self.warning.to_hex(),
            self.warning_container.to_hex(),
            self.success.to_hex(),
            self.success_container.to_hex(),
            self.info.to_hex(),
            self.info_container.to_hex(),
            self.surface.to_hex(),
            self.surface_variant.to_hex(),
            self.surface_dim.to_hex(),
            self.background.to_hex(),
            self.foreground.to_hex(),
            self.foreground_secondary.to_hex(),
            self.foreground_tertiary.to_hex(),
            self.outline.to_hex(),
            self.outline_variant.to_hex(),
        )
    }

    /// Get theme name
    pub fn theme(&self) -> ThemeName {
        self.theme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_light() {
        let theme = ColorSystem::from_theme(ThemeName::Light);
        assert_eq!(theme.theme(), ThemeName::Light);
    }

    #[test]
    fn test_theme_dark() {
        let theme = ColorSystem::from_theme(ThemeName::Dark);
        assert_eq!(theme.theme(), ThemeName::Dark);
    }

    #[test]
    fn test_theme_oled() {
        let theme = ColorSystem::from_theme(ThemeName::OLED);
        assert_eq!(theme.theme(), ThemeName::OLED);
    }

    #[test]
    fn test_theme_hdr() {
        let theme = ColorSystem::from_theme(ThemeName::HDR);
        assert_eq!(theme.theme(), ThemeName::HDR);
    }

    #[test]
    fn test_css_generation() {
        let theme = ColorSystem::from_theme(ThemeName::Light);
        let css = theme.to_css();
        assert!(css.contains("--color-primary"));
        assert!(css.contains("--color-error"));
    }

    #[test]
    fn test_all_colors_present() {
        let theme = ColorSystem::from_theme(ThemeName::Light);
        assert_ne!(theme.primary, Color::default());
        assert_ne!(theme.error, Color::default());
        assert_ne!(theme.success, Color::default());
    }

    #[test]
    fn test_theme_contrast() {
        let light = ColorSystem::from_theme(ThemeName::Light);
        // Light theme should have good contrast
        assert!(light.foreground.passes_wcag_aaa(&light.background));
    }
}
