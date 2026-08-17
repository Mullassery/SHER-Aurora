use crate::errors::{TypographyError, TypographyResult};
use crate::font::{FontFamily, FontVariant, FontWeight};
use serde::{Deserialize, Serialize};

/// Text level in the hierarchy
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TextLevel {
    Display,
    Headline,
    Title,
    Body,
    Caption,
    Micro,
}

impl TextLevel {
    pub fn name(&self) -> &'static str {
        match self {
            TextLevel::Display => "Display",
            TextLevel::Headline => "Headline",
            TextLevel::Title => "Title",
            TextLevel::Body => "Body",
            TextLevel::Caption => "Caption",
            TextLevel::Micro => "Micro",
        }
    }
}

/// Complete typography style
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TypographyStyle {
    pub font_size: u16, // Base font size in pixels
    pub font_weight: FontWeight,
    pub font_variant: FontVariant,
    pub line_height: f32,    // Multiplier (e.g., 1.5 = 150% of font size)
    pub letter_spacing: f32, // In pixels or em
    pub contrast_ratio: f32, // Minimum WCAG AAA compliance
}

impl TypographyStyle {
    pub fn new(
        font_size: u16,
        font_weight: FontWeight,
        line_height: f32,
        letter_spacing: f32,
    ) -> TypographyResult<Self> {
        if font_size == 0 {
            return Err(TypographyError::InvalidFontSize(
                "Font size must be > 0".to_string(),
            ));
        }
        if line_height <= 0.0 {
            return Err(TypographyError::InvalidLineHeight(
                "Line height must be > 0".to_string(),
            ));
        }

        Ok(Self {
            font_size,
            font_weight,
            font_variant: FontVariant::Normal,
            line_height,
            letter_spacing,
            contrast_ratio: 4.5, // Default WCAG AA
        })
    }

    pub fn line_height_pixels(&self) -> f32 {
        self.font_size as f32 * self.line_height
    }

    pub fn letter_spacing_em(&self) -> f32 {
        self.letter_spacing / self.font_size as f32
    }

    pub fn to_css(&self, font_family: &str) -> String {
        format!(
            "font-family: {}; font-size: {}px; font-weight: {}; line-height: {}; letter-spacing: {}px;",
            font_family,
            self.font_size,
            self.font_weight.value(),
            self.line_height,
            self.letter_spacing,
        )
    }
}

/// Complete type scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScale {
    pub display: TypographyStyle,
    pub headline: TypographyStyle,
    pub title: TypographyStyle,
    pub body: TypographyStyle,
    pub caption: TypographyStyle,
    pub micro: TypographyStyle,
    pub font_family: FontFamily,
}

impl Default for TypeScale {
    fn default() -> Self {
        Self::new(FontFamily::Inter)
    }
}

impl TypeScale {
    /// Create a new type scale for a given font family
    pub fn new(font_family: FontFamily) -> Self {
        Self {
            // Display: Large, attention-grabbing headlines
            display: TypographyStyle {
                font_size: 48,
                font_weight: FontWeight::Bold,
                font_variant: FontVariant::Normal,
                line_height: 1.2,
                letter_spacing: -0.02 * 48.0, // -2% tracking
                contrast_ratio: 7.0,
            },
            // Headline: Section headings
            headline: TypographyStyle {
                font_size: 32,
                font_weight: FontWeight::SemiBold,
                font_variant: FontVariant::Normal,
                line_height: 1.25,
                letter_spacing: -0.01 * 32.0, // -1% tracking
                contrast_ratio: 7.0,
            },
            // Title: Card titles, dialog titles
            title: TypographyStyle {
                font_size: 20,
                font_weight: FontWeight::SemiBold,
                font_variant: FontVariant::Normal,
                line_height: 1.3,
                letter_spacing: 0.0,
                contrast_ratio: 7.0,
            },
            // Body: Primary reading content
            body: TypographyStyle {
                font_size: 14,
                font_weight: FontWeight::Regular,
                font_variant: FontVariant::Normal,
                line_height: 1.5,
                letter_spacing: 0.01 * 14.0, // +1% tracking
                contrast_ratio: 7.0,
            },
            // Caption: Secondary text, metadata
            caption: TypographyStyle {
                font_size: 12,
                font_weight: FontWeight::Medium,
                font_variant: FontVariant::Normal,
                line_height: 1.4,
                letter_spacing: 0.02 * 12.0, // +2% tracking
                contrast_ratio: 4.5,
            },
            // Micro: Badges, tags, timestamps
            micro: TypographyStyle {
                font_size: 11,
                font_weight: FontWeight::Medium,
                font_variant: FontVariant::Normal,
                line_height: 1.3,
                letter_spacing: 0.03 * 11.0, // +3% tracking
                contrast_ratio: 4.5,
            },
            font_family,
        }
    }

    /// Get style for a text level
    pub fn get_style(&self, level: TextLevel) -> TypographyStyle {
        match level {
            TextLevel::Display => self.display,
            TextLevel::Headline => self.headline,
            TextLevel::Title => self.title,
            TextLevel::Body => self.body,
            TextLevel::Caption => self.caption,
            TextLevel::Micro => self.micro,
        }
    }

    /// Validate scale consistency
    pub fn validate(&self) -> TypographyResult<()> {
        // Ensure sizes are in descending order
        if self.display.font_size < self.headline.font_size {
            return Err(TypographyError::ValidationError(
                "Display must be larger than Headline".to_string(),
            ));
        }
        if self.headline.font_size < self.title.font_size {
            return Err(TypographyError::ValidationError(
                "Headline must be larger than Title".to_string(),
            ));
        }
        if self.title.font_size < self.body.font_size {
            return Err(TypographyError::ValidationError(
                "Title must be larger than Body".to_string(),
            ));
        }
        if self.body.font_size < self.caption.font_size {
            return Err(TypographyError::ValidationError(
                "Body must be larger than Caption".to_string(),
            ));
        }
        if self.caption.font_size < self.micro.font_size {
            return Err(TypographyError::ValidationError(
                "Caption must be larger than Micro".to_string(),
            ));
        }

        // Ensure line heights are valid
        for (name, style) in [
            ("Display", self.display),
            ("Headline", self.headline),
            ("Title", self.title),
            ("Body", self.body),
            ("Caption", self.caption),
            ("Micro", self.micro),
        ] {
            if style.line_height <= 0.0 {
                return Err(TypographyError::ValidationError(format!(
                    "{} has invalid line height",
                    name
                )));
            }
        }

        Ok(())
    }

    /// Generate CSS for all type scales
    pub fn to_css(&self) -> String {
        let family = self.font_family.name();
        let mut css = String::new();

        let scales = [
            ("display", self.display),
            ("headline", self.headline),
            ("title", self.title),
            ("body", self.body),
            ("caption", self.caption),
            ("micro", self.micro),
        ];

        for (class, style) in scales.iter() {
            css.push_str(&format!(
                ".text-{} {{\n  {}\n}}\n\n",
                class,
                style.to_css(family)
            ));
        }

        css
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typography_style_creation() {
        let style = TypographyStyle::new(14, FontWeight::Regular, 1.5, 0.14).unwrap();
        assert_eq!(style.font_size, 14);
        assert_eq!(style.line_height, 1.5);
    }

    #[test]
    fn test_typography_style_invalid_size() {
        let result = TypographyStyle::new(0, FontWeight::Regular, 1.5, 0.14);
        assert!(result.is_err());
    }

    #[test]
    fn test_line_height_pixels() {
        let style = TypographyStyle::new(14, FontWeight::Regular, 1.5, 0.14).unwrap();
        assert_eq!(style.line_height_pixels(), 21.0);
    }

    #[test]
    fn test_type_scale_default() {
        let scale = TypeScale::default();
        assert_eq!(scale.body.font_size, 14);
        assert_eq!(scale.caption.font_size, 12);
    }

    #[test]
    fn test_type_scale_validation() {
        let scale = TypeScale::default();
        assert!(scale.validate().is_ok());
    }

    #[test]
    fn test_get_style() {
        let scale = TypeScale::default();
        let body = scale.get_style(TextLevel::Body);
        assert_eq!(body.font_size, 14);
    }

    #[test]
    fn test_to_css() {
        let scale = TypeScale::default();
        let css = scale.to_css();
        assert!(css.contains("text-body"));
        assert!(css.contains("text-headline"));
    }

    #[test]
    fn test_size_ordering() {
        let scale = TypeScale::default();
        assert!(scale.display.font_size > scale.headline.font_size);
        assert!(scale.headline.font_size > scale.title.font_size);
        assert!(scale.title.font_size > scale.body.font_size);
        assert!(scale.body.font_size > scale.caption.font_size);
        assert!(scale.caption.font_size > scale.micro.font_size);
    }
}
