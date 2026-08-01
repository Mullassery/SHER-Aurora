/// Aurora Sidebar component
///
/// A vertical navigation sidebar with collapsible sections.
#[derive(Debug, Clone)]
pub struct Sidebar {
    width: i32,
    spacing: i32,
    collapsed: bool,
    css_classes: Vec<String>,
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new() -> Self {
        Self {
            width: 280,
            spacing: 0,
            collapsed: false,
            css_classes: vec!["aurora-sidebar".to_string()],
        }
    }

    /// Set sidebar width
    pub fn with_width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    /// Set sidebar spacing
    pub fn with_spacing(mut self, spacing: i32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Collapse/expand sidebar
    pub fn set_collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Check if sidebar is collapsed
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    /// Toggle collapse state
    pub fn toggle_collapse(mut self) -> Self {
        self.collapsed = !self.collapsed;
        self
    }

    /// Add CSS class
    pub fn add_css_class(mut self, class: &str) -> Self {
        self.css_classes.push(class.to_string());
        self
    }

    /// Get sidebar width
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get sidebar spacing
    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    /// Get CSS classes
    pub fn css_classes(&self) -> &[String] {
        &self.css_classes
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_new() {
        let sidebar = Sidebar::new();
        assert!(!sidebar.is_collapsed());
    }

    #[test]
    fn test_sidebar_width() {
        let sidebar = Sidebar::new().with_width(250);
        assert_eq!(sidebar.width(), 250);
    }

    #[test]
    fn test_sidebar_spacing() {
        let sidebar = Sidebar::new().with_spacing(8);
        assert_eq!(sidebar.spacing(), 8);
    }

    #[test]
    fn test_sidebar_collapsed() {
        let sidebar = Sidebar::new().set_collapsed(true);
        assert!(sidebar.is_collapsed());
    }

    #[test]
    fn test_sidebar_toggle() {
        let sidebar = Sidebar::new();
        assert!(!sidebar.is_collapsed());

        let sidebar = sidebar.toggle_collapse();
        assert!(sidebar.is_collapsed());

        let sidebar = sidebar.toggle_collapse();
        assert!(!sidebar.is_collapsed());
    }

    #[test]
    fn test_sidebar_css_class() {
        let sidebar = Sidebar::new().add_css_class("custom");
        assert!(sidebar.css_classes().contains(&"custom".to_string()));
    }

    #[test]
    fn test_sidebar_chaining() {
        let _sidebar = Sidebar::new()
            .with_width(280)
            .with_spacing(12)
            .set_collapsed(false)
            .add_css_class("test");
    }
}
