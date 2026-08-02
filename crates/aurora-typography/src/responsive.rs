use crate::errors::{TypographyError, TypographyResult};
use crate::scale::TypographyStyle;
use serde::{Deserialize, Serialize};

/// Screen size breakpoint
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Breakpoint {
    Mobile,    // 11" laptop (1366×768)
    Tablet,    // 14" laptop (1920×1080)
    Desktop,   // 24"–27" monitor (2560×1440)
    Ultrawide, // >30" ultrawide (3440×1440)
}

impl Breakpoint {
    pub fn min_width(&self) -> u16 {
        match self {
            Breakpoint::Mobile => 1024,
            Breakpoint::Tablet => 1366,
            Breakpoint::Desktop => 1920,
            Breakpoint::Ultrawide => 2560,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Breakpoint::Mobile => "mobile",
            Breakpoint::Tablet => "tablet",
            Breakpoint::Desktop => "desktop",
            Breakpoint::Ultrawide => "ultrawide",
        }
    }
}

/// Viewport size
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ViewportSize {
    pub width: u16,
    pub height: u16,
}

impl ViewportSize {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn breakpoint(&self) -> Breakpoint {
        if self.width >= 2560 {
            Breakpoint::Ultrawide
        } else if self.width >= 1920 {
            Breakpoint::Desktop
        } else if self.width >= 1366 {
            Breakpoint::Tablet
        } else {
            Breakpoint::Mobile
        }
    }

    pub fn diagonal_inches(&self) -> f32 {
        let width_in = self.width as f32 / 96.0; // 96 DPI
        let height_in = self.height as f32 / 96.0;
        (width_in.powi(2) + height_in.powi(2)).sqrt()
    }
}

/// Responsive scaling for a text level
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResponsiveScale {
    pub mobile: u16,    // 11" laptop
    pub tablet: u16,    // 14" laptop
    pub desktop: u16,   // 24"–27" monitor
    pub ultrawide: u16, // >30" ultrawide
}

impl ResponsiveScale {
    pub fn new(mobile: u16, tablet: u16, desktop: u16, ultrawide: u16) -> Self {
        Self {
            mobile,
            tablet,
            desktop,
            ultrawide,
        }
    }

    pub fn for_breakpoint(&self, breakpoint: Breakpoint) -> u16 {
        match breakpoint {
            Breakpoint::Mobile => self.mobile,
            Breakpoint::Tablet => self.tablet,
            Breakpoint::Desktop => self.desktop,
            Breakpoint::Ultrawide => self.ultrawide,
        }
    }

    pub fn for_viewport(&self, viewport: ViewportSize) -> u16 {
        self.for_breakpoint(viewport.breakpoint())
    }
}

/// Responsive typography system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveTypography {
    pub display: ResponsiveScale,
    pub headline: ResponsiveScale,
    pub title: ResponsiveScale,
    pub body: ResponsiveScale,
    pub caption: ResponsiveScale,
    pub micro: ResponsiveScale,
    pub use_fluid_typography: bool,
}

impl Default for ResponsiveTypography {
    fn default() -> Self {
        Self {
            // Display: 36px (mobile) → 60px (ultrawide)
            display: ResponsiveScale::new(36, 44, 60, 72),
            // Headline: 24px (mobile) → 40px (ultrawide)
            headline: ResponsiveScale::new(24, 32, 40, 48),
            // Title: 18px (mobile) → 24px (ultrawide)
            title: ResponsiveScale::new(18, 22, 24, 28),
            // Body: 14px (mobile) → 16px (ultrawide)
            body: ResponsiveScale::new(14, 15, 16, 18),
            // Caption: 12px (mobile) → 13px (ultrawide)
            caption: ResponsiveScale::new(12, 12, 13, 14),
            // Micro: 11px (fixed, no scaling)
            micro: ResponsiveScale::new(11, 11, 12, 12),
            use_fluid_typography: true,
        }
    }
}

impl ResponsiveTypography {
    /// Apply responsive adjustments to a typography style
    pub fn apply_responsive_adjustments(
        &self,
        style: &mut TypographyStyle,
        viewport: ViewportSize,
    ) {
        // Get the responsive scale for this style
        // This is simplified; in practice we'd need to identify which style we're adjusting
        let scale = match style.font_size {
            48 => self.display,  // Display base
            32 => self.headline, // Headline base
            20 => self.title,    // Title base
            14 => self.body,     // Body base
            12 => self.caption,  // Caption base
            11 => self.micro,    // Micro base
            _ => return,         // Unknown style
        };

        // Apply responsive size
        style.font_size = scale.for_viewport(viewport);

        // Adjust line height slightly for smaller screens (tighter leading on mobile)
        if viewport.breakpoint() == Breakpoint::Mobile {
            style.line_height *= 0.95;
        }
    }

    /// Generate CSS with responsive breakpoints
    pub fn to_css(&self) -> String {
        let mut css = String::new();

        css.push_str("/* Mobile (11\" laptop) */\n");
        css.push_str("@media (min-width: 1024px) {\n");
        css.push_str(&format!(
            "  .text-display {{ font-size: {}px; }}\n",
            self.display.mobile
        ));
        css.push_str(&format!(
            "  .text-headline {{ font-size: {}px; }}\n",
            self.headline.mobile
        ));
        css.push_str(&format!(
            "  .text-title {{ font-size: {}px; }}\n",
            self.title.mobile
        ));
        css.push_str("}\n\n");

        css.push_str("/* Tablet (14\" laptop) */\n");
        css.push_str("@media (min-width: 1366px) {\n");
        css.push_str(&format!(
            "  .text-display {{ font-size: {}px; }}\n",
            self.display.tablet
        ));
        css.push_str(&format!(
            "  .text-headline {{ font-size: {}px; }}\n",
            self.headline.tablet
        ));
        css.push_str(&format!(
            "  .text-title {{ font-size: {}px; }}\n",
            self.title.tablet
        ));
        css.push_str("}\n\n");

        css.push_str("/* Desktop (24–27\" monitor) */\n");
        css.push_str("@media (min-width: 1920px) {\n");
        css.push_str(&format!(
            "  .text-display {{ font-size: {}px; }}\n",
            self.display.desktop
        ));
        css.push_str(&format!(
            "  .text-headline {{ font-size: {}px; }}\n",
            self.headline.desktop
        ));
        css.push_str(&format!(
            "  .text-title {{ font-size: {}px; }}\n",
            self.title.desktop
        ));
        css.push_str("}\n\n");

        css.push_str("/* Ultrawide (>30\") */\n");
        css.push_str("@media (min-width: 2560px) {\n");
        css.push_str(&format!(
            "  .text-display {{ font-size: {}px; }}\n",
            self.display.ultrawide
        ));
        css.push_str(&format!(
            "  .text-headline {{ font-size: {}px; }}\n",
            self.headline.ultrawide
        ));
        css.push_str(&format!(
            "  .text-title {{ font-size: {}px; }}\n",
            self.title.ultrawide
        ));
        css.push_str("}\n");

        css
    }

    pub fn validate(&self) -> TypographyResult<()> {
        // Ensure all scales have mobile <= tablet <= desktop <= ultrawide
        let scales = [
            ("display", self.display),
            ("headline", self.headline),
            ("title", self.title),
            ("body", self.body),
            ("caption", self.caption),
            ("micro", self.micro),
        ];

        for (name, scale) in scales.iter() {
            if scale.mobile > scale.tablet
                || scale.tablet > scale.desktop
                || scale.desktop > scale.ultrawide
            {
                return Err(TypographyError::ValidationError(format!(
                    "{} sizes not in ascending order",
                    name
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_breakpoint() {
        assert_eq!(
            ViewportSize::new(1024, 768).breakpoint(),
            Breakpoint::Mobile
        );
        assert_eq!(
            ViewportSize::new(1920, 1080).breakpoint(),
            Breakpoint::Desktop
        );
        assert_eq!(
            ViewportSize::new(3440, 1440).breakpoint(),
            Breakpoint::Ultrawide
        );
    }

    #[test]
    fn test_responsive_scale() {
        let scale = ResponsiveScale::new(14, 15, 16, 18);
        assert_eq!(scale.for_breakpoint(Breakpoint::Mobile), 14);
        assert_eq!(scale.for_breakpoint(Breakpoint::Desktop), 16);
    }

    #[test]
    fn test_responsive_typography_default() {
        let responsive = ResponsiveTypography::default();
        assert!(responsive.use_fluid_typography);
    }

    #[test]
    fn test_responsive_typography_validation() {
        let responsive = ResponsiveTypography::default();
        assert!(responsive.validate().is_ok());
    }

    #[test]
    fn test_responsive_css() {
        let responsive = ResponsiveTypography::default();
        let css = responsive.to_css();
        assert!(css.contains("@media"));
        assert!(css.contains("text-display"));
    }

    #[test]
    fn test_viewport_diagonal() {
        let viewport = ViewportSize::new(1920, 1080);
        let diagonal = viewport.diagonal_inches();
        assert!(diagonal > 20.0 && diagonal < 25.0); // Should be ~21.5"
    }
}
