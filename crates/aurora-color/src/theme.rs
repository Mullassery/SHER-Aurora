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
            primary: Color::from_hex("#003D99").unwrap(),
            primary_container: Color::from_hex("#E3F2FD").unwrap(),
            secondary: Color::from_hex("#440099").unwrap(),
            secondary_container: Color::from_hex("#F3E5F5").unwrap(),
            accent: Color::from_hex("#AA0044").unwrap(),
            error: Color::from_hex("#990000").unwrap(),
            error_container: Color::from_hex("#FFEBEE").unwrap(),
            warning: Color::from_hex("#994400").unwrap(),
            warning_container: Color::from_hex("#FFF8E1").unwrap(),
            success: Color::from_hex("#004400").unwrap(),
            success_container: Color::from_hex("#E8F5E9").unwrap(),
            info: Color::from_hex("#0066CC").unwrap(),
            info_container: Color::from_hex("#E3F2FD").unwrap(),
            surface: Color::from_hex("#FEFEFE").unwrap(),
            surface_variant: Color::from_hex("#F5F5F5").unwrap(),
            surface_dim: Color::from_hex("#F0F0F0").unwrap(),
            background: Color::from_hex("#FFFFFF").unwrap(),
            foreground: Color::from_hex("#1A1A1A").unwrap(),
            foreground_secondary: Color::from_hex("#333333").unwrap(),
            foreground_tertiary: Color::from_hex("#666666").unwrap(),
            // #949494 clears the WCAG 1.4.11 non-text contrast minimum (3:1) against both
            // surface and background; the previous #CCCCCC only reached ~1.6:1.
            outline: Color::from_hex("#949494").unwrap(),
            outline_variant: Color::from_hex("#DDDDDD").unwrap(),
            theme: ThemeName::Light,
        }
    }

    /// Dark theme palette
    fn dark() -> Self {
        Self {
            primary: Color::from_hex("#6EB7FF").unwrap(),
            // Container tones below are darkened from their original values so the
            // semantic color rendered on top (e.g. `primary` on `primary_container`)
            // clears real WCAG AAA text contrast (7:1) instead of falling to ~2-4:1.
            primary_container: Color::from_hex("#052456").unwrap(),
            secondary: Color::from_hex("#C5B3FF").unwrap(),
            secondary_container: Color::from_hex("#470D7A").unwrap(),
            accent: Color::from_hex("#FF80AB").unwrap(),
            error: Color::from_hex("#F8A29A").unwrap(),
            error_container: Color::from_hex("#58130F").unwrap(),
            warning: Color::from_hex("#FFD54F").unwrap(),
            warning_container: Color::from_hex("#693500").unwrap(),
            success: Color::from_hex("#81C784").unwrap(),
            success_container: Color::from_hex("#0E3211").unwrap(),
            info: Color::from_hex("#64B5F6").unwrap(),
            info_container: Color::from_hex("#06224D").unwrap(),
            surface: Color::from_hex("#1E1E1E").unwrap(),
            surface_variant: Color::from_hex("#2D2D2D").unwrap(),
            surface_dim: Color::from_hex("#121212").unwrap(),
            background: Color::from_hex("#121212").unwrap(),
            foreground: Color::from_hex("#F5F5F5").unwrap(),
            foreground_secondary: Color::from_hex("#BDBDBD").unwrap(),
            foreground_tertiary: Color::from_hex("#9E9E9E").unwrap(),
            // #696969 clears the WCAG 1.4.11 non-text contrast minimum (3:1) against both
            // surface and background; the previous #424242 only reached ~1.6-1.9:1.
            outline: Color::from_hex("#696969").unwrap(),
            outline_variant: Color::from_hex("#333333").unwrap(),
            theme: ThemeName::Dark,
        }
    }

    /// OLED theme palette (true black)
    fn oled() -> Self {
        Self {
            primary: Color::from_hex("#6EB7FF").unwrap(),
            primary_container: Color::from_hex("#052456").unwrap(),
            secondary: Color::from_hex("#C5B3FF").unwrap(),
            secondary_container: Color::from_hex("#470D7A").unwrap(),
            accent: Color::from_hex("#FF80AB").unwrap(),
            error: Color::from_hex("#F8A29A").unwrap(),
            error_container: Color::from_hex("#58130F").unwrap(),
            warning: Color::from_hex("#FFD54F").unwrap(),
            warning_container: Color::from_hex("#693500").unwrap(),
            success: Color::from_hex("#81C784").unwrap(),
            success_container: Color::from_hex("#0E3211").unwrap(),
            info: Color::from_hex("#64B5F6").unwrap(),
            info_container: Color::from_hex("#06224D").unwrap(),
            surface: Color::from_hex("#0D0D0D").unwrap(),
            surface_variant: Color::from_hex("#1A1A1A").unwrap(),
            surface_dim: Color::from_hex("#000000").unwrap(),
            background: Color::from_hex("#000000").unwrap(),
            foreground: Color::from_hex("#F5F5F5").unwrap(),
            foreground_secondary: Color::from_hex("#BDBDBD").unwrap(),
            foreground_tertiary: Color::from_hex("#9E9E9E").unwrap(),
            // #5F5F5F clears the WCAG 1.4.11 non-text contrast minimum (3:1) against both
            // surface and background; the previous #2D2D2D only reached ~1.4-1.5:1.
            outline: Color::from_hex("#5F5F5F").unwrap(),
            outline_variant: Color::from_hex("#1A1A1A").unwrap(),
            theme: ThemeName::OLED,
        }
    }

    /// HDR theme palette (extended color gamut)
    fn hdr() -> Self {
        Self {
            primary: Color::from_hex("#6EB7FF").unwrap(),
            primary_container: Color::from_hex("#052456").unwrap(),
            secondary: Color::from_hex("#C5B3FF").unwrap(),
            secondary_container: Color::from_hex("#470D7A").unwrap(),
            accent: Color::from_hex("#FF80AB").unwrap(),
            error: Color::from_hex("#F8A29A").unwrap(),
            error_container: Color::from_hex("#58130F").unwrap(),
            warning: Color::from_hex("#FFD54F").unwrap(),
            warning_container: Color::from_hex("#693500").unwrap(),
            success: Color::from_hex("#81C784").unwrap(),
            success_container: Color::from_hex("#0E3211").unwrap(),
            info: Color::from_hex("#64B5F6").unwrap(),
            info_container: Color::from_hex("#06224D").unwrap(),
            surface: Color::from_hex("#1E1E1E").unwrap(),
            surface_variant: Color::from_hex("#2D2D2D").unwrap(),
            surface_dim: Color::from_hex("#121212").unwrap(),
            background: Color::from_hex("#121212").unwrap(),
            foreground: Color::from_hex("#F5F5F5").unwrap(),
            foreground_secondary: Color::from_hex("#BDBDBD").unwrap(),
            foreground_tertiary: Color::from_hex("#9E9E9E").unwrap(),
            outline: Color::from_hex("#696969").unwrap(),
            outline_variant: Color::from_hex("#333333").unwrap(),
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
