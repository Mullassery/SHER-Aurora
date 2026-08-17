/// Aurora Card styles
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CardStyle {
    /// Solid fill with surface color
    #[default]
    Filled,
    /// Border only, transparent background
    Outlined,
    /// Elevated with shadow
    Elevated,
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

    /// Build a real `gtk4::Box` container widget from this descriptor.
    ///
    /// Constructs an actual GTK4 vertical box acting as the card surface:
    /// spacing, margins, and Aurora CSS classes (including the per-style
    /// class used to select filled/outlined/elevated appearance in the
    /// generated stylesheet) are applied through the real `gtk4` widget
    /// API. Callers must have already initialized GTK before calling this.
    /// Any real GTK4 widgets to be shown inside the card can be packed in
    /// with `gtk4::prelude::BoxExt::append` on the returned box.
    pub fn build(&self) -> gtk4::Box {
        use gtk4::prelude::*;

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(self.spacing)
            .margin_top(self.margin)
            .margin_bottom(self.margin)
            .margin_start(self.margin)
            .margin_end(self.margin)
            .build();

        for class in &self.css_classes {
            container.add_css_class(class);
        }

        match self.style {
            CardStyle::Filled => container.add_css_class("aurora-card-filled"),
            CardStyle::Outlined => {
                container.add_css_class("aurora-card-outlined");
                container.add_css_class("frame");
            }
            CardStyle::Elevated => container.add_css_class("aurora-card-elevated"),
        }

        container
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

    // Real GTK4 widget-construction test — see the comment in
    // `widgets::switch::tests` for why this is gated off macOS and how to
    // verify real GTK4 rendering locally on macOS instead.
    #[cfg(not(target_os = "macos"))]
    mod gtk_real {
        use super::*;

        #[gtk4::test]
        fn test_card_build_is_real_gtk4_widget() {
            use gtk4::prelude::*;
            let card = Card::new()
                .with_style(CardStyle::Elevated)
                .with_spacing(12)
                .build();
            assert_eq!(card.spacing(), 12);
            assert_eq!(card.orientation(), gtk4::Orientation::Vertical);
            assert!(card.css_classes().iter().any(|c| c == "aurora-card"));
            assert!(card
                .css_classes()
                .iter()
                .any(|c| c == "aurora-card-elevated"));

            // Pack a real child widget in, proving this is a genuine GTK4
            // container that real widgets can be appended to.
            let label = gtk4::Label::new(Some("Inside the card"));
            card.append(&label);
            assert!(card.first_child().is_some());
        }
    }
}
