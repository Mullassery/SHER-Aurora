/// Aurora List component
///
/// A scrollable list container for displaying collections of items.
#[derive(Debug, Clone)]
pub struct List {
    spacing: i32,
    margin: i32,
    css_classes: Vec<String>,
    items: Vec<String>,
}

impl List {
    /// Create a new list
    pub fn new() -> Self {
        Self {
            spacing: 0,
            margin: 0,
            css_classes: vec!["aurora-list".to_string()],
            items: Vec::new(),
        }
    }

    /// Add item to list
    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }

    /// Set list spacing
    pub fn with_spacing(mut self, spacing: i32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set list margin
    pub fn with_margin(mut self, margin: i32) -> Self {
        self.margin = margin;
        self
    }

    /// Add CSS class
    pub fn add_css_class(mut self, class: &str) -> Self {
        self.css_classes.push(class.to_string());
        self
    }

    /// Get list items
    pub fn items(&self) -> &[String] {
        &self.items
    }

    /// Get list spacing
    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    /// Get list margin
    pub fn margin(&self) -> i32 {
        self.margin
    }

    /// Get CSS classes
    pub fn css_classes(&self) -> &[String] {
        &self.css_classes
    }

    /// Get item count
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if list is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_new() {
        let list = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_add_item() {
        let list = List::new().add_item("Item 1").add_item("Item 2");
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0], "Item 1");
    }

    #[test]
    fn test_list_spacing() {
        let list = List::new().with_spacing(12);
        assert_eq!(list.spacing(), 12);
    }

    #[test]
    fn test_list_margin() {
        let list = List::new().with_margin(8);
        assert_eq!(list.margin(), 8);
    }

    #[test]
    fn test_list_css_class() {
        let list = List::new().add_css_class("custom");
        assert!(list.css_classes().contains(&"custom".to_string()));
    }

    #[test]
    fn test_list_is_empty() {
        let list = List::new();
        assert!(list.is_empty());

        let list = list.add_item("Item");
        assert!(!list.is_empty());
    }

    #[test]
    fn test_list_chaining() {
        let _list = List::new()
            .with_spacing(8)
            .with_margin(4)
            .add_item("Item 1")
            .add_item("Item 2")
            .add_css_class("test");
    }
}
