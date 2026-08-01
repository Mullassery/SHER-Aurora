/// Aurora Card styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardStyle {
    /// Solid fill with surface color
    Filled,
    /// Border only, transparent background
    Outlined,
    /// Elevated with shadow
    Elevated,
}

impl Default for CardStyle {
    fn default() -> Self {
        Self::Filled
    }
}

/// Aurora Card component
///
/// A versatile container for grouped content with three style variants:
/// - Filled: Solid surface color (default)
/// - Outlined: Border only, minimal
/// - Elevated: Shadow elevation (emphasis)
#[derive(Debug, Clone)]
pub struct Card {
    style: CardStyle,
    spacing: i32,
    margin: i32,
    css_classes: Vec<String>,
}

impl Card {
    /// Create a new card
    pub fn new() -> Self {
        Self {
            style: CardStyle::default(),
            spacing: 0,
            margin: 0,
            css_classes: vec!["aurora-card".to_string()],
        }
    }

    /// Set card style
    pub fn with_style(mut self, style: CardStyle) -> Self {
        self.style = style;
        self
    }

    /// Set card spacing
    pub fn with_spacing(mut self, spacing: i32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set card margin
    pub fn with_margin(mut self, margin: i32) -> Self {
        self.margin = margin;
        self
    }

    /// Add CSS class
    pub fn add_css_class(mut self, class: &str) -> Self {
        self.css_classes.push(class.to_string());
        self
    }

    /// Get card style
    pub fn style(&self) -> CardStyle {
        self.style
    }

    /// Get card spacing
    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    /// Get card margin
    pub fn margin(&self) -> i32 {
        self.margin
    }

    /// Get CSS classes
    pub fn css_classes(&self) -> &[String] {
        &self.css_classes
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_new() {
        let card = Card::new();
        assert_eq!(card.style(), CardStyle::Filled);
    }

    #[test]
    fn test_card_style_filled() {
        let card = Card::new().with_style(CardStyle::Filled);
        assert_eq!(card.style(), CardStyle::Filled);
    }

    #[test]
    fn test_card_style_outlined() {
        let card = Card::new().with_style(CardStyle::Outlined);
        assert_eq!(card.style(), CardStyle::Outlined);
    }

    #[test]
    fn test_card_style_elevated() {
        let card = Card::new().with_style(CardStyle::Elevated);
        assert_eq!(card.style(), CardStyle::Elevated);
    }

    #[test]
    fn test_card_spacing() {
        let card = Card::new().with_spacing(16);
        assert_eq!(card.spacing(), 16);
    }

    #[test]
    fn test_card_margin() {
        let card = Card::new().with_margin(8);
        assert_eq!(card.margin(), 8);
    }

    #[test]
    fn test_card_css_class() {
        let card = Card::new().add_css_class("test-class");
        assert!(card.css_classes().contains(&"test-class".to_string()));
    }

    #[test]
    fn test_card_chaining() {
        let _card = Card::new()
            .with_style(CardStyle::Elevated)
            .with_spacing(12)
            .with_margin(8)
            .add_css_class("custom");
        // If this compiles, chaining works
    }
}
