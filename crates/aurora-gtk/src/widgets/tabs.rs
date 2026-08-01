//! Tabs Widget - Multi-View Navigation
//!
//! Tab component for switching between multiple content views.
//! Features: Multiple styles, keyboard navigation, animations, responsive.

use std::fmt;

/// Tab style variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStyle {
    Underline,  // Underline active tab
    Button,     // Button-like tabs
    Pill,       // Pill-shaped with background
}

/// Tab orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabOrientation {
    Horizontal,
    Vertical,
}

/// Individual tab
#[derive(Debug, Clone)]
pub struct Tab {
    id: String,
    label: String,
    enabled: bool,
    badge: Option<u32>,
}

impl Tab {
    /// Create a new tab
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            enabled: true,
            badge: None,
        }
    }

    /// Disable the tab
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Add a badge (e.g., for unread count)
    pub fn with_badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }

    /// Get tab ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get tab label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Is tab enabled?
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get badge count
    pub fn badge(&self) -> Option<u32> {
        self.badge
    }
}

/// Tab panel content
#[derive(Debug, Clone)]
pub struct TabPanel {
    tab_id: String,
    title: String,
    content: String,
    lazy_load: bool,
}

impl TabPanel {
    /// Create a new tab panel
    pub fn new(tab_id: &str, title: &str) -> Self {
        Self {
            tab_id: tab_id.to_string(),
            title: title.to_string(),
            content: String::new(),
            lazy_load: false,
        }
    }

    /// Set panel content
    pub fn with_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Enable lazy loading
    pub fn lazy_load(mut self) -> Self {
        self.lazy_load = true;
        self
    }

    /// Get tab ID
    pub fn tab_id(&self) -> &str {
        &self.tab_id
    }

    /// Get title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Is lazy loading enabled?
    pub fn is_lazy_load(&self) -> bool {
        self.lazy_load
    }
}

/// Tabs container
pub struct Tabs {
    tabs: Vec<Tab>,
    panels: Vec<TabPanel>,
    active_index: usize,
    style: TabStyle,
    orientation: TabOrientation,
}

impl Tabs {
    /// Create a new tabs container
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            panels: Vec::new(),
            active_index: 0,
            style: TabStyle::Underline,
            orientation: TabOrientation::Horizontal,
        }
    }

    /// Set tab style
    pub fn with_style(mut self, style: TabStyle) -> Self {
        self.style = style;
        self
    }

    /// Set orientation
    pub fn with_orientation(mut self, orientation: TabOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Add a tab
    pub fn add_tab(&mut self, tab: Tab) {
        self.tabs.push(tab);
    }

    /// Add a panel
    pub fn add_panel(&mut self, panel: TabPanel) {
        self.panels.push(panel);
    }

    /// Get all tabs
    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Get all panels
    pub fn panels(&self) -> &[TabPanel] {
        &self.panels
    }

    /// Get active tab
    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_index)
    }

    /// Get active panel
    pub fn active_panel(&self) -> Option<&TabPanel> {
        self.panels.get(self.active_index)
    }

    /// Get active tab index
    pub fn active_index(&self) -> usize {
        self.active_index
    }

    /// Set active tab by index
    pub fn set_active(&mut self, index: usize) -> bool {
        if index < self.tabs.len() && self.tabs[index].is_enabled() {
            self.active_index = index;
            true
        } else {
            false
        }
    }

    /// Set active tab by ID
    pub fn set_active_by_id(&mut self, id: &str) -> bool {
        if let Some(pos) = self.tabs.iter().position(|t| t.id == id && t.is_enabled()) {
            self.active_index = pos;
            true
        } else {
            false
        }
    }

    /// Next tab
    pub fn next(&mut self) -> bool {
        for i in (self.active_index + 1)..self.tabs.len() {
            if self.tabs[i].is_enabled() {
                self.active_index = i;
                return true;
            }
        }
        false
    }

    /// Previous tab
    pub fn prev(&mut self) -> bool {
        for i in (0..self.active_index).rev() {
            if self.tabs[i].is_enabled() {
                self.active_index = i;
                return true;
            }
        }
        false
    }

    /// Get tab style
    pub fn style(&self) -> TabStyle {
        self.style
    }

    /// Get orientation
    pub fn orientation(&self) -> TabOrientation {
        self.orientation
    }

    /// Get tab count
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Get CSS class
    pub fn css_class(&self) -> String {
        let style_str = match self.style {
            TabStyle::Underline => "underline",
            TabStyle::Button => "button",
            TabStyle::Pill => "pill",
        };
        let orientation_str = match self.orientation {
            TabOrientation::Horizontal => "horizontal",
            TabOrientation::Vertical => "vertical",
        };
        format!("aurora-tabs aurora-tabs-{} aurora-tabs-{}", style_str, orientation_str)
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Tabs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tabs")
            .field("tab_count", &self.tab_count())
            .field("active_index", &self.active_index)
            .field("style", &self.style)
            .field("orientation", &self.orientation)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabs_creation() {
        let tabs = Tabs::new();
        assert_eq!(tabs.tab_count(), 0);
        assert_eq!(tabs.style(), TabStyle::Underline);
        assert_eq!(tabs.orientation(), TabOrientation::Horizontal);
    }

    #[test]
    fn test_add_tabs() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("tab1", "Home"));
        tabs.add_tab(Tab::new("tab2", "Settings"));
        tabs.add_tab(Tab::new("tab3", "Help"));
        assert_eq!(tabs.tab_count(), 3);
    }

    #[test]
    fn test_add_panels() {
        let mut tabs = Tabs::new();
        tabs.add_panel(TabPanel::new("tab1", "Home"));
        tabs.add_panel(TabPanel::new("tab2", "Settings"));
        assert_eq!(tabs.panels().len(), 2);
    }

    #[test]
    fn test_set_active_by_index() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("tab1", "Tab 1"));
        tabs.add_tab(Tab::new("tab2", "Tab 2"));

        assert!(tabs.set_active(1));
        assert_eq!(tabs.active_index(), 1);
        assert_eq!(tabs.active_tab().unwrap().label(), "Tab 2");
    }

    #[test]
    fn test_set_active_by_id() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("home", "Home"));
        tabs.add_tab(Tab::new("settings", "Settings"));

        assert!(tabs.set_active_by_id("settings"));
        assert_eq!(tabs.active_tab().unwrap().id(), "settings");
    }

    #[test]
    fn test_next_tab() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("tab1", "Tab 1"));
        tabs.add_tab(Tab::new("tab2", "Tab 2"));
        tabs.add_tab(Tab::new("tab3", "Tab 3"));

        assert!(tabs.next());
        assert_eq!(tabs.active_index(), 1);
        assert!(tabs.next());
        assert_eq!(tabs.active_index(), 2);
        assert!(!tabs.next()); // No more tabs
    }

    #[test]
    fn test_prev_tab() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("tab1", "Tab 1"));
        tabs.add_tab(Tab::new("tab2", "Tab 2"));
        tabs.add_tab(Tab::new("tab3", "Tab 3"));

        tabs.set_active(2);
        assert!(tabs.prev());
        assert_eq!(tabs.active_index(), 1);
        assert!(tabs.prev());
        assert_eq!(tabs.active_index(), 0);
        assert!(!tabs.prev()); // No previous tabs
    }

    #[test]
    fn test_disabled_tabs_skipped() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("tab1", "Tab 1"));
        tabs.add_tab(Tab::new("tab2", "Tab 2").disabled());
        tabs.add_tab(Tab::new("tab3", "Tab 3"));

        // Should skip disabled tab2
        assert!(tabs.next());
        assert_eq!(tabs.active_index(), 2); // Jumps to tab3
    }

    #[test]
    fn test_disabled_tab_not_selectable() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("tab1", "Tab 1"));
        tabs.add_tab(Tab::new("tab2", "Tab 2").disabled());

        assert!(!tabs.set_active(1)); // Cannot select disabled tab
        assert_eq!(tabs.active_index(), 0); // Stays on tab1
    }

    #[test]
    fn test_tab_badge() {
        let tab = Tab::new("notifications", "Notifications").with_badge(5);
        assert_eq!(tab.badge(), Some(5));
    }

    #[test]
    fn test_tab_styles() {
        let underline = Tabs::new().with_style(TabStyle::Underline);
        assert_eq!(underline.style(), TabStyle::Underline);

        let button = Tabs::new().with_style(TabStyle::Button);
        assert_eq!(button.style(), TabStyle::Button);

        let pill = Tabs::new().with_style(TabStyle::Pill);
        assert_eq!(pill.style(), TabStyle::Pill);
    }

    #[test]
    fn test_tab_orientation() {
        let horizontal = Tabs::new().with_orientation(TabOrientation::Horizontal);
        assert_eq!(horizontal.orientation(), TabOrientation::Horizontal);

        let vertical = Tabs::new().with_orientation(TabOrientation::Vertical);
        assert_eq!(vertical.orientation(), TabOrientation::Vertical);
    }

    #[test]
    fn test_css_class() {
        let tabs = Tabs::new()
            .with_style(TabStyle::Button)
            .with_orientation(TabOrientation::Vertical);
        assert!(tabs.css_class().contains("aurora-tabs"));
        assert!(tabs.css_class().contains("button"));
        assert!(tabs.css_class().contains("vertical"));
    }

    #[test]
    fn test_active_panel() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("tab1", "Tab 1"));
        tabs.add_tab(Tab::new("tab2", "Tab 2"));
        tabs.add_panel(TabPanel::new("tab1", "Tab 1 Content"));
        tabs.add_panel(TabPanel::new("tab2", "Tab 2 Content"));

        assert_eq!(tabs.active_panel().unwrap().title(), "Tab 1 Content");

        tabs.set_active(1);
        assert_eq!(tabs.active_panel().unwrap().title(), "Tab 2 Content");
    }

    #[test]
    fn test_lazy_load_panel() {
        let panel = TabPanel::new("tab1", "Tab 1").lazy_load();
        assert!(panel.is_lazy_load());
    }

    #[test]
    fn test_panel_content() {
        let panel = TabPanel::new("tab1", "Tab 1").with_content("Panel content here");
        assert_eq!(panel.content(), "Panel content here");
    }

    #[test]
    fn test_default() {
        let tabs = Tabs::default();
        assert_eq!(tabs.tab_count(), 0);
        assert_eq!(tabs.active_index(), 0);
    }

    #[test]
    fn test_complex_navigation() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("tab1", "Tab 1"));
        tabs.add_tab(Tab::new("tab2", "Tab 2").disabled());
        tabs.add_tab(Tab::new("tab3", "Tab 3"));
        tabs.add_tab(Tab::new("tab4", "Tab 4"));

        // Start at tab1
        assert_eq!(tabs.active_index(), 0);

        // Navigate forward (skips disabled tab2)
        tabs.next();
        assert_eq!(tabs.active_index(), 2); // tab3

        // Navigate backward
        tabs.prev();
        assert_eq!(tabs.active_index(), 0); // tab1 (skips disabled tab2)

        // Direct set
        assert!(tabs.set_active_by_id("tab4"));
        assert_eq!(tabs.active_index(), 3);
    }

    #[test]
    fn test_invalid_operations() {
        let mut tabs = Tabs::new();
        tabs.add_tab(Tab::new("tab1", "Tab 1"));

        // Invalid index
        assert!(!tabs.set_active(10));

        // Empty next/prev
        for _ in 0..5 {
            tabs.next(); // Eventually reaches end
        }
        assert!(!tabs.next());
    }
}
