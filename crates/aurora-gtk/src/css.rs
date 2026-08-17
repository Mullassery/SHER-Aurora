use crate::errors::{GtkError, GtkResult};
use crate::theme::Theme;
use aurora_tokens::DesignTokens;

/// GTK CSS Provider — converts Aurora tokens to GTK CSS
#[derive(Debug)]
pub struct CssProvider {
    theme: Theme,
    tokens: DesignTokens,
}

impl CssProvider {
    /// Create a new CSS provider
    pub fn new(theme: Theme) -> GtkResult<Self> {
        let mut tokens = DesignTokens::new();

        // Map our Theme to Aurora theme
        let aurora_theme = match theme {
            Theme::Light => aurora_tokens::Theme::Light,
            Theme::Dark => aurora_tokens::Theme::Dark,
            Theme::OLED => aurora_tokens::Theme::OLED,
            Theme::HDR => aurora_tokens::Theme::HDR,
        };

        tokens.set_theme(aurora_theme);

        // Validate tokens
        tokens
            .validate()
            .map_err(|e| GtkError::CssError(e.to_string()))?;

        Ok(Self { theme, tokens })
    }

    /// Generate complete CSS stylesheet
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // CSS Reset
        css.push_str("/* Aurora Design System - GTK4 Stylesheet */\n\n");
        css.push_str("/* Theme Class */\n");
        css.push_str(&format!(
            ".{} {{\n  --aurora-theme: {};\n}}\n\n",
            self.theme.css_class(),
            self.theme.name()
        ));

        // Design Tokens as CSS Custom Properties
        css.push_str("/* Design Tokens */\n");
        css.push_str(":root {\n");

        // Spacing tokens
        css.push_str("  /* Spacing (8px baseline) */\n");
        css.push_str(&format!(
            "  --spacing-xxs: {}px;\n",
            self.tokens.spacing.xxs
        ));
        css.push_str(&format!("  --spacing-xs: {}px;\n", self.tokens.spacing.xs));
        css.push_str(&format!("  --spacing-sm: {}px;\n", self.tokens.spacing.sm));
        css.push_str(&format!("  --spacing-md: {}px;\n", self.tokens.spacing.md));
        css.push_str(&format!("  --spacing-lg: {}px;\n", self.tokens.spacing.lg));
        css.push_str(&format!("  --spacing-xl: {}px;\n", self.tokens.spacing.xl));
        css.push_str(&format!(
            "  --spacing-xxl: {}px;\n",
            self.tokens.spacing.xxl
        ));
        css.push_str(&format!(
            "  --spacing-xxxl: {}px;\n",
            self.tokens.spacing.xxxl
        ));

        // Radius tokens
        css.push_str("\n  /* Border Radius */\n");
        css.push_str(&format!("  --radius-xs: {}px;\n", self.tokens.radius.xs));
        css.push_str(&format!("  --radius-sm: {}px;\n", self.tokens.radius.sm));
        css.push_str(&format!("  --radius-md: {}px;\n", self.tokens.radius.md));
        css.push_str(&format!("  --radius-lg: {}px;\n", self.tokens.radius.lg));
        css.push_str(&format!("  --radius-xl: {}px;\n", self.tokens.radius.xl));

        // Motion tokens
        css.push_str("\n  /* Motion (Durations) */\n");
        css.push_str("  --motion-instant: 80ms;\n");
        css.push_str("  --motion-fast: 120ms;\n");
        css.push_str("  --motion-normal: 220ms;\n");
        css.push_str("  --motion-slow: 350ms;\n");
        css.push_str("  --motion-dramatic: 500ms;\n");

        // Elevation tokens
        css.push_str("\n  /* Elevation (Shadows) */\n");
        css.push_str(&format!(
            "  --elevation-1: {};\n",
            self.tokens.elevation.level1.to_css()
        ));
        css.push_str(&format!(
            "  --elevation-2: {};\n",
            self.tokens.elevation.level2.to_css()
        ));
        css.push_str(&format!(
            "  --elevation-3: {};\n",
            self.tokens.elevation.level3.to_css()
        ));
        css.push_str(&format!(
            "  --elevation-4: {};\n",
            self.tokens.elevation.level4.to_css()
        ));
        css.push_str(&format!(
            "  --elevation-5: {};\n",
            self.tokens.elevation.level5.to_css()
        ));

        // Color tokens (from current theme)
        let colors = self.tokens.color_system.current();
        css.push_str("\n  /* Semantic Colors */\n");
        css.push_str(&format!(
            "  --color-surface: {};\n",
            colors.surface.to_rgba()
        ));
        css.push_str(&format!(
            "  --color-surface-variant: {};\n",
            colors.surface_variant.to_rgba()
        ));
        css.push_str(&format!(
            "  --color-background: {};\n",
            colors.background.to_rgba()
        ));
        css.push_str(&format!(
            "  --color-foreground: {};\n",
            colors.foreground.to_rgba()
        ));
        css.push_str(&format!(
            "  --color-foreground-secondary: {};\n",
            colors.foreground_secondary.to_rgba()
        ));
        css.push_str(&format!(
            "  --color-foreground-tertiary: {};\n",
            colors.foreground_tertiary.to_rgba()
        ));
        css.push_str(&format!(
            "  --color-primary: {};\n",
            colors.primary.to_rgba()
        ));
        css.push_str(&format!(
            "  --color-secondary: {};\n",
            colors.secondary.to_rgba()
        ));
        css.push_str(&format!("  --color-accent: {};\n", colors.accent.to_rgba()));
        css.push_str(&format!(
            "  --color-success: {};\n",
            colors.success.to_rgba()
        ));
        css.push_str(&format!(
            "  --color-warning: {};\n",
            colors.warning.to_rgba()
        ));
        css.push_str(&format!("  --color-error: {};\n", colors.error.to_rgba()));
        css.push_str(&format!("  --color-info: {};\n", colors.info.to_rgba()));
        css.push_str(&format!(
            "  --color-outline: {};\n",
            colors.outline.to_rgba()
        ));

        css.push_str("}\n\n");

        // Component Styles
        css.push_str("/* Component Styles */\n\n");

        // Button
        css.push_str(".aurora-button {\n");
        css.push_str("  padding: var(--spacing-md) var(--spacing-lg);\n");
        css.push_str("  border-radius: var(--radius-sm);\n");
        css.push_str("  background-color: var(--color-primary);\n");
        css.push_str("  color: white;\n");
        css.push_str("  border: none;\n");
        css.push_str("  font-weight: 500;\n");
        css.push_str("  transition: all var(--motion-fast);\n");
        css.push_str("}\n\n");

        css.push_str(".aurora-button:hover {\n");
        css.push_str("  box-shadow: var(--elevation-1);\n");
        css.push_str("  transform: translateY(-1px);\n");
        css.push_str("}\n\n");

        css.push_str(".aurora-button:active {\n");
        css.push_str("  transform: translateY(0);\n");
        css.push_str("  box-shadow: none;\n");
        css.push_str("}\n\n");

        // Card
        css.push_str(".aurora-card {\n");
        css.push_str("  padding: var(--spacing-md);\n");
        css.push_str("  border-radius: var(--radius-md);\n");
        css.push_str("  background-color: var(--color-surface);\n");
        css.push_str("  box-shadow: var(--elevation-2);\n");
        css.push_str("  transition: box-shadow var(--motion-normal);\n");
        css.push_str("}\n\n");

        css.push_str(".aurora-card:hover {\n");
        css.push_str("  box-shadow: var(--elevation-3);\n");
        css.push_str("}\n\n");

        // Input
        css.push_str(".aurora-input {\n");
        css.push_str("  padding: var(--spacing-md);\n");
        css.push_str("  border-radius: var(--radius-sm);\n");
        css.push_str("  border: 1px solid var(--color-outline);\n");
        css.push_str("  background-color: var(--color-surface);\n");
        css.push_str("  color: var(--color-foreground);\n");
        css.push_str("  transition: border-color var(--motion-fast);\n");
        css.push_str("}\n\n");

        css.push_str(".aurora-input:focus {\n");
        css.push_str("  border-color: var(--color-primary);\n");
        css.push_str("  outline: 2px solid var(--color-accent);\n");
        css.push_str("  outline-offset: 2px;\n");
        css.push_str("}\n\n");

        css
    }

    /// Get the current theme
    pub fn theme(&self) -> Theme {
        self.theme
    }

    /// Get the design tokens
    pub fn tokens(&self) -> &DesignTokens {
        &self.tokens
    }

    /// Load this provider's generated stylesheet into a real GTK4
    /// `gtk4::CssProvider` and install it on `display` at application
    /// priority, so it actually affects how every real widget on that
    /// display is rendered.
    ///
    /// This performs genuine GTK4 CSS-engine work (`gtk_css_provider_load_from_string`
    /// together with `gtk_style_context_add_provider_for_display`), not a mock.
    /// Note that the generated stylesheet borrows some web-CSS conventions that
    /// GTK4's CSS engine may not fully support in every case; any unsupported
    /// declaration is ignored by GTK's permissive CSS parser (it logs to
    /// stderr, it does not fail), the same way a browser ignores unknown
    /// CSS. Bringing the token-derived stylesheet to full GTK4 CSS parity is
    /// tracked as follow-up work.
    pub fn install(&self, display: &gtk4::gdk::Display) -> gtk4::CssProvider {
        let provider = gtk4::CssProvider::new();
        provider.load_from_string(&self.generate_css());
        gtk4::style_context_add_provider_for_display(
            display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        provider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_provider_light() {
        let provider = CssProvider::new(Theme::Light);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_css_provider_dark() {
        let provider = CssProvider::new(Theme::Dark);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_css_generation() {
        let provider = CssProvider::new(Theme::Light).unwrap();
        let css = provider.generate_css();
        assert!(css.contains("--spacing-md"));
        assert!(css.contains("--color-primary"));
        assert!(css.contains(".aurora-button"));
        assert!(css.contains(".aurora-card"));
        assert!(css.contains(".aurora-input"));
    }

    #[test]
    fn test_css_contains_theme_class() {
        let provider = CssProvider::new(Theme::Light).unwrap();
        let css = provider.generate_css();
        assert!(css.contains("aurora-light"));
    }

    #[test]
    fn test_css_contains_motion_tokens() {
        let provider = CssProvider::new(Theme::Light).unwrap();
        let css = provider.generate_css();
        assert!(css.contains("--motion-instant"));
        assert!(css.contains("--motion-fast"));
        assert!(css.contains("--motion-normal"));
    }
}
