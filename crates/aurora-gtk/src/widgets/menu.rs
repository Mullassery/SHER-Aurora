//! Menu Widget - Context and Navigation Menus
//!
//! Flexible menu system with keyboard shortcuts, submenus, dividers, and icons.

use std::fmt;

/// Menu item type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemType {
    Action,  // Regular menu item
    Toggle,  // Toggleable menu item (checkbox-like)
    Submenu, // Item with submenu
    Divider, // Separator
}

/// Menu item state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemState {
    Normal,
    Disabled,
    Checked, // For toggle items
}

/// Keyboard shortcut
#[derive(Debug, Clone)]
pub struct Shortcut {
    modifiers: Vec<String>, // ctrl, shift, alt, meta
    key: String,            // a, b, c, F1, etc.
}

impl Shortcut {
    /// Create a new shortcut
    pub fn new(key: &str) -> Self {
        Self {
            modifiers: Vec::new(),
            key: key.to_string(),
        }
    }

    /// Add modifier (ctrl, shift, alt, meta)
    pub fn with_modifier(mut self, modifier: &str) -> Self {
        self.modifiers.push(modifier.to_string());
        self
    }

    /// Get display string (e.g. "Ctrl+S")
    pub fn display(&self) -> String {
        let mut parts: Vec<String> = self.modifiers.clone();
        parts.push(self.key.clone());
        parts.join("+")
    }

    /// Get modifiers
    pub fn modifiers(&self) -> &[String] {
        &self.modifiers
    }

    /// Get key
    pub fn key(&self) -> &str {
        &self.key
    }
}

/// Individual menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    id: String,
    label: String,
    item_type: MenuItemType,
    state: MenuItemState,
    shortcut: std::option::Option<Shortcut>,
    icon: std::option::Option<String>,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            item_type: MenuItemType::Action,
            state: MenuItemState::Normal,
            shortcut: std::option::Option::None,
            icon: std::option::Option::None,
        }
    }

    /// Make item a toggle (checkbox)
    pub fn toggle(mut self) -> Self {
        self.item_type = MenuItemType::Toggle;
        self
    }

    /// Make item a submenu
    pub fn submenu(mut self) -> Self {
        self.item_type = MenuItemType::Submenu;
        self
    }

    /// Create a divider
    pub fn divider() -> Self {
        Self {
            id: "divider".to_string(),
            label: String::new(),
            item_type: MenuItemType::Divider,
            state: MenuItemState::Normal,
            shortcut: std::option::Option::None,
            icon: std::option::Option::None,
        }
    }

    /// Disable the item
    pub fn disabled(mut self) -> Self {
        self.state = MenuItemState::Disabled;
        self
    }

    /// Check the item (for toggle items)
    pub fn checked(mut self) -> Self {
        self.state = MenuItemState::Checked;
        self
    }

    /// Add keyboard shortcut
    pub fn with_shortcut(mut self, shortcut: Shortcut) -> Self {
        self.shortcut = std::option::Option::Some(shortcut);
        self
    }

    /// Add icon
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = std::option::Option::Some(icon.to_string());
        self
    }

    /// Get item ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get item label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get item type
    pub fn item_type(&self) -> MenuItemType {
        self.item_type
    }

    /// Get item state
    pub fn state(&self) -> MenuItemState {
        self.state
    }

    /// Get shortcut (if any)
    pub fn shortcut(&self) -> std::option::Option<&Shortcut> {
        self.shortcut.as_ref()
    }

    /// Get icon (if any)
    pub fn icon(&self) -> std::option::Option<&str> {
        self.icon.as_deref()
    }

    /// Is disabled?
    pub fn is_disabled(&self) -> bool {
        self.state == MenuItemState::Disabled
    }

    /// Is checked?
    pub fn is_checked(&self) -> bool {
        self.state == MenuItemState::Checked
    }

    /// Is divider?
    pub fn is_divider(&self) -> bool {
        self.item_type == MenuItemType::Divider
    }
}

/// Menu container
pub struct Menu {
    items: Vec<MenuItem>,
    submenus: std::collections::HashMap<String, Menu>,
    open_submenu: std::option::Option<String>,
}

impl Menu {
    /// Create a new menu
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            submenus: std::collections::HashMap::new(),
            open_submenu: std::option::Option::None,
        }
    }

    /// Add an item
    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    /// Add a divider
    pub fn add_divider(&mut self) {
        self.items.push(MenuItem::divider());
    }

    /// Add multiple items
    pub fn add_items(&mut self, items: Vec<MenuItem>) {
        self.items.extend(items);
    }

    /// Get all items
    pub fn items(&self) -> &[MenuItem] {
        &self.items
    }

    /// Add submenu
    pub fn add_submenu(&mut self, item_id: &str, submenu: Menu) {
        self.submenus.insert(item_id.to_string(), submenu);
    }

    /// Get submenu (if exists)
    pub fn submenu(&self, item_id: &str) -> std::option::Option<&Menu> {
        self.submenus.get(item_id)
    }

    /// Get mutable submenu
    pub fn submenu_mut(&mut self, item_id: &str) -> std::option::Option<&mut Menu> {
        self.submenus.get_mut(item_id)
    }

    /// Open submenu
    pub fn open_submenu(&mut self, item_id: &str) -> bool {
        if self.submenus.contains_key(item_id) {
            self.open_submenu = std::option::Option::Some(item_id.to_string());
            true
        } else {
            false
        }
    }

    /// Close submenu
    pub fn close_submenu(&mut self) {
        self.open_submenu = std::option::Option::None;
    }

    /// Get open submenu ID (if any)
    pub fn open_submenu_id(&self) -> std::option::Option<&str> {
        self.open_submenu.as_deref()
    }

    /// Is submenu open?
    pub fn is_submenu_open(&self, item_id: &str) -> bool {
        self.open_submenu.as_ref().is_some_and(|id| id == item_id)
    }

    /// Toggle submenu
    pub fn toggle_submenu(&mut self, item_id: &str) -> bool {
        if self.is_submenu_open(item_id) {
            self.close_submenu();
            false
        } else {
            self.open_submenu(item_id)
        }
    }

    /// Find item by ID
    pub fn find_item(&self, id: &str) -> std::option::Option<&MenuItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get item with shortcuts
    pub fn items_with_shortcuts(&self) -> Vec<&MenuItem> {
        self.items
            .iter()
            .filter(|item| item.shortcut.is_some())
            .collect()
    }

    /// Get item with icons
    pub fn items_with_icons(&self) -> Vec<&MenuItem> {
        self.items
            .iter()
            .filter(|item| item.icon.is_some())
            .collect()
    }

    /// Get enabled items
    pub fn enabled_items(&self) -> Vec<&MenuItem> {
        self.items
            .iter()
            .filter(|item| !item.is_disabled() && !item.is_divider())
            .collect()
    }

    /// Get divider count
    pub fn divider_count(&self) -> usize {
        self.items.iter().filter(|item| item.is_divider()).count()
    }

    /// Get submenu count
    pub fn submenu_count(&self) -> usize {
        self.submenus.len()
    }

    /// Get CSS class
    pub fn css_class(&self) -> String {
        "aurora-menu".to_string()
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Menu")
            .field("item_count", &self.item_count())
            .field("submenu_count", &self.submenu_count())
            .field("open_submenu", &self.open_submenu)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_creation() {
        let menu = Menu::new();
        assert_eq!(menu.item_count(), 0);
        assert_eq!(menu.submenu_count(), 0);
    }

    #[test]
    fn test_add_items() {
        let mut menu = Menu::new();
        menu.add_item(MenuItem::new("save", "Save"));
        menu.add_item(MenuItem::new("undo", "Undo"));
        assert_eq!(menu.item_count(), 2);
    }

    #[test]
    fn test_add_divider() {
        let mut menu = Menu::new();
        menu.add_item(MenuItem::new("save", "Save"));
        menu.add_divider();
        menu.add_item(MenuItem::new("exit", "Exit"));

        assert_eq!(menu.item_count(), 3);
        assert_eq!(menu.divider_count(), 1);
    }

    #[test]
    fn test_menu_item_types() {
        let action = MenuItem::new("save", "Save");
        assert_eq!(action.item_type(), MenuItemType::Action);

        let toggle = MenuItem::new("bold", "Bold").toggle();
        assert_eq!(toggle.item_type(), MenuItemType::Toggle);

        let submenu = MenuItem::new("view", "View").submenu();
        assert_eq!(submenu.item_type(), MenuItemType::Submenu);

        let divider = MenuItem::divider();
        assert_eq!(divider.item_type(), MenuItemType::Divider);
    }

    #[test]
    fn test_menu_item_disabled() {
        let item = MenuItem::new("save", "Save").disabled();
        assert!(item.is_disabled());
        assert_eq!(item.state(), MenuItemState::Disabled);
    }

    #[test]
    fn test_menu_item_checked() {
        let item = MenuItem::new("bold", "Bold").toggle().checked();
        assert!(item.is_checked());
        assert_eq!(item.state(), MenuItemState::Checked);
    }

    #[test]
    fn test_shortcut() {
        let shortcut = Shortcut::new("S").with_modifier("Ctrl");
        assert_eq!(shortcut.key(), "S");
        assert_eq!(shortcut.display(), "Ctrl+S");
    }

    #[test]
    fn test_menu_item_with_shortcut() {
        let shortcut = Shortcut::new("S").with_modifier("Ctrl");
        let item = MenuItem::new("save", "Save").with_shortcut(shortcut);

        assert!(item.shortcut().is_some());
        assert_eq!(item.shortcut().unwrap().display(), "Ctrl+S");
    }

    #[test]
    fn test_menu_item_with_icon() {
        let item = MenuItem::new("save", "Save").with_icon("document-save");
        assert_eq!(item.icon(), std::option::Option::Some("document-save"));
    }

    #[test]
    fn test_add_submenu() {
        let mut menu = Menu::new();
        let mut view_menu = Menu::new();
        view_menu.add_item(MenuItem::new("zoom-in", "Zoom In"));

        menu.add_item(MenuItem::new("view", "View").submenu());
        menu.add_submenu("view", view_menu);

        assert_eq!(menu.submenu_count(), 1);
        assert!(menu.submenu("view").is_some());
    }

    #[test]
    fn test_open_submenu() {
        let mut menu = Menu::new();
        menu.add_item(MenuItem::new("view", "View").submenu());
        menu.add_submenu("view", Menu::new());

        assert!(!menu.is_submenu_open("view"));

        assert!(menu.open_submenu("view"));
        assert!(menu.is_submenu_open("view"));

        menu.close_submenu();
        assert!(!menu.is_submenu_open("view"));
    }

    #[test]
    fn test_toggle_submenu() {
        let mut menu = Menu::new();
        menu.add_item(MenuItem::new("view", "View").submenu());
        menu.add_submenu("view", Menu::new());

        menu.toggle_submenu("view");
        assert!(menu.is_submenu_open("view"));

        menu.toggle_submenu("view");
        assert!(!menu.is_submenu_open("view"));
    }

    #[test]
    fn test_find_item() {
        let mut menu = Menu::new();
        menu.add_item(MenuItem::new("save", "Save"));
        menu.add_item(MenuItem::new("exit", "Exit"));

        assert!(menu.find_item("save").is_some());
        assert_eq!(menu.find_item("save").unwrap().label(), "Save");
        assert!(menu.find_item("unknown").is_none());
    }

    #[test]
    fn test_items_with_shortcuts() {
        let mut menu = Menu::new();
        menu.add_item(
            MenuItem::new("save", "Save").with_shortcut(Shortcut::new("S").with_modifier("Ctrl")),
        );
        menu.add_item(MenuItem::new("undo", "Undo"));

        let with_shortcuts = menu.items_with_shortcuts();
        assert_eq!(with_shortcuts.len(), 1);
        assert_eq!(with_shortcuts[0].label(), "Save");
    }

    #[test]
    fn test_items_with_icons() {
        let mut menu = Menu::new();
        menu.add_item(MenuItem::new("save", "Save").with_icon("document-save"));
        menu.add_item(MenuItem::new("exit", "Exit"));

        let with_icons = menu.items_with_icons();
        assert_eq!(with_icons.len(), 1);
        assert_eq!(with_icons[0].label(), "Save");
    }

    #[test]
    fn test_enabled_items() {
        let mut menu = Menu::new();
        menu.add_item(MenuItem::new("save", "Save"));
        menu.add_item(MenuItem::new("readonly", "Read Only").disabled());
        menu.add_divider();
        menu.add_item(MenuItem::new("exit", "Exit"));

        let enabled = menu.enabled_items();
        assert_eq!(enabled.len(), 2); // save and exit
    }

    #[test]
    fn test_complex_menu() {
        let mut menu = Menu::new();

        // File menu items
        menu.add_item(
            MenuItem::new("new", "New").with_shortcut(Shortcut::new("N").with_modifier("Ctrl")),
        );
        menu.add_item(
            MenuItem::new("open", "Open").with_shortcut(Shortcut::new("O").with_modifier("Ctrl")),
        );
        menu.add_item(
            MenuItem::new("save", "Save").with_shortcut(Shortcut::new("S").with_modifier("Ctrl")),
        );
        menu.add_divider();
        menu.add_item(MenuItem::new("exit", "Exit"));

        // Create View submenu
        let mut view_menu = Menu::new();
        view_menu.add_item(
            MenuItem::new("zoom-in", "Zoom In")
                .with_shortcut(Shortcut::new("Plus").with_modifier("Ctrl")),
        );
        view_menu.add_item(
            MenuItem::new("zoom-out", "Zoom Out")
                .with_shortcut(Shortcut::new("Minus").with_modifier("Ctrl")),
        );
        view_menu.add_item(MenuItem::new("full-screen", "Full Screen").toggle());

        menu.add_item(MenuItem::new("view", "View").submenu());
        menu.add_submenu("view", view_menu);

        assert_eq!(menu.item_count(), 6); // new, open, save, divider, exit, view
        assert_eq!(menu.submenu_count(), 1);
        assert_eq!(menu.items_with_shortcuts().len(), 3); // new, open, save
    }

    #[test]
    fn test_default() {
        let menu = Menu::default();
        assert_eq!(menu.item_count(), 0);
    }

    #[test]
    fn test_css_class() {
        let menu = Menu::new();
        assert_eq!(menu.css_class(), "aurora-menu");
    }

    #[test]
    fn test_open_submenu_id() {
        let mut menu = Menu::new();
        menu.add_item(MenuItem::new("view", "View").submenu());
        menu.add_submenu("view", Menu::new());

        assert!(menu.open_submenu_id().is_none());

        menu.open_submenu("view");
        assert_eq!(menu.open_submenu_id(), std::option::Option::Some("view"));
    }

    #[test]
    fn test_divider() {
        let divider = MenuItem::divider();
        assert!(divider.is_divider());
        assert_eq!(divider.item_type(), MenuItemType::Divider);
    }

    #[test]
    fn test_shortcut_multiple_modifiers() {
        let shortcut = Shortcut::new("S")
            .with_modifier("Ctrl")
            .with_modifier("Shift");
        assert_eq!(shortcut.display(), "Ctrl+Shift+S");
    }

    #[test]
    fn test_add_multiple_items() {
        let mut menu = Menu::new();
        let items = vec![
            MenuItem::new("new", "New"),
            MenuItem::new("open", "Open"),
            MenuItem::new("save", "Save"),
        ];
        menu.add_items(items);
        assert_eq!(menu.item_count(), 3);
    }
}
