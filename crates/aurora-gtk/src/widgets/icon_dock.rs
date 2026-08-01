//! Icon Dock Widget - Animated Icon Navigation Bar
//!
//! Icon-based navigation dock with spring animations, hover effects, and visual feedback.

use std::fmt;

/// Dock orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockOrientation {
    Horizontal,  // Bottom or top dock
    Vertical,    // Left or right dock
}

/// Dock position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockPosition {
    Top,
    Bottom,
    Left,
    Right,
}

/// Icon dock item
#[derive(Debug, Clone)]
pub struct DockItem {
    id: String,
    icon: String,
    label: String,
    active: bool,
    badge: std::option::Option<u32>,
}

impl DockItem {
    /// Create new dock item
    pub fn new(id: &str, icon: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            icon: icon.to_string(),
            label: label.to_string(),
            active: false,
            badge: std::option::Option::None,
        }
    }

    /// Mark as active
    pub fn active(mut self) -> Self {
        self.active = true;
        self
    }

    /// Add badge (notification count)
    pub fn with_badge(mut self, count: u32) -> Self {
        self.badge = std::option::Option::Some(count);
        self
    }

    /// Getters
    pub fn id(&self) -> &str { &self.id }
    pub fn icon(&self) -> &str { &self.icon }
    pub fn label(&self) -> &str { &self.label }
    pub fn is_active(&self) -> bool { self.active }
    pub fn badge(&self) -> std::option::Option<u32> { self.badge }
}

/// Animation configuration
#[derive(Debug, Clone)]
pub struct DockAnimation {
    scale_on_hover: f32,     // 1.0-1.5
    scale_spring_damping: f32, // 0.5-1.0
    scale_spring_stiffness: f32, // 100-300
    duration_ms: u32,        // 150-300ms
    active_indicator_height: f32, // px
}

impl DockAnimation {
    pub fn new() -> Self {
        Self {
            scale_on_hover: 1.2,
            scale_spring_damping: 0.8,
            scale_spring_stiffness: 200.0,
            duration_ms: 200,
            active_indicator_height: 3.0,
        }
    }

    /// Spring animation (natural, bouncy)
    pub fn spring(mut self) -> Self {
        self.scale_spring_damping = 0.6;
        self.scale_spring_stiffness = 300.0;
        self
    }

    /// Smooth animation (linear)
    pub fn smooth(mut self) -> Self {
        self.scale_spring_damping = 1.0;
        self.scale_spring_stiffness = 150.0;
        self
    }

    /// Fast animation
    pub fn fast(mut self) -> Self {
        self.duration_ms = 100;
        self
    }

    /// Getters
    pub fn scale_on_hover(&self) -> f32 { self.scale_on_hover }
    pub fn scale_spring_damping(&self) -> f32 { self.scale_spring_damping }
    pub fn scale_spring_stiffness(&self) -> f32 { self.scale_spring_stiffness }
    pub fn duration_ms(&self) -> u32 { self.duration_ms }
    pub fn active_indicator_height(&self) -> f32 { self.active_indicator_height }
}

impl Default for DockAnimation {
    fn default() -> Self {
        Self::new()
    }
}

/// Icon dock container
pub struct IconDock {
    items: Vec<DockItem>,
    active_index: std::option::Option<usize>,
    orientation: DockOrientation,
    position: DockPosition,
    animation: DockAnimation,
    icon_size: u32,  // px
    spacing: u32,    // px between icons
}

impl IconDock {
    /// Create new dock
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            active_index: std::option::Option::None,
            orientation: DockOrientation::Horizontal,
            position: DockPosition::Bottom,
            animation: DockAnimation::default(),
            icon_size: 48,
            spacing: 12,
        }
    }

    /// Set orientation
    pub fn with_orientation(mut self, orientation: DockOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set position
    pub fn with_position(mut self, position: DockPosition) -> Self {
        self.position = position;
        self
    }

    /// Set animation config
    pub fn with_animation(mut self, animation: DockAnimation) -> Self {
        self.animation = animation;
        self
    }

    /// Set icon size
    pub fn with_icon_size(mut self, size: u32) -> Self {
        self.icon_size = size;
        self
    }

    /// Set spacing
    pub fn with_spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Add item
    pub fn add_item(&mut self, item: DockItem) {
        if item.is_active() && self.active_index.is_none() {
            self.active_index = std::option::Option::Some(self.items.len());
        }
        self.items.push(item);
    }

    /// Add multiple items
    pub fn add_items(&mut self, items: Vec<DockItem>) {
        for item in items {
            self.add_item(item);
        }
    }

    /// Get all items
    pub fn items(&self) -> &[DockItem] {
        &self.items
    }

    /// Get active item
    pub fn active_item(&self) -> std::option::Option<&DockItem> {
        self.active_index.and_then(|idx| self.items.get(idx))
    }

    /// Get active index
    pub fn active_index(&self) -> std::option::Option<usize> {
        self.active_index
    }

    /// Set active by index
    pub fn set_active(&mut self, index: usize) -> bool {
        if index < self.items.len() {
            self.active_index = std::option::Option::Some(index);
            true
        } else {
            false
        }
    }

    /// Set active by ID
    pub fn set_active_by_id(&mut self, id: &str) -> bool {
        if let std::option::Option::Some(idx) = self.items.iter().position(|item| item.id == id) {
            self.active_index = std::option::Option::Some(idx);
            true
        } else {
            false
        }
    }

    /// Find item by ID
    pub fn find_item(&self, id: &str) -> std::option::Option<&DockItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get total width (horizontal) or height (vertical)
    pub fn dimension(&self) -> u32 {
        if self.items.is_empty() {
            0
        } else {
            let count = self.items.len() as u32;
            self.icon_size * count + self.spacing * (count - 1)
        }
    }

    /// Get orientation
    pub fn orientation(&self) -> DockOrientation {
        self.orientation
    }

    /// Get position
    pub fn position(&self) -> DockPosition {
        self.position
    }

    /// Get animation config
    pub fn animation(&self) -> &DockAnimation {
        &self.animation
    }

    /// Get icon size
    pub fn icon_size(&self) -> u32 {
        self.icon_size
    }

    /// Get spacing
    pub fn spacing(&self) -> u32 {
        self.spacing
    }

    /// Get CSS class
    pub fn css_class(&self) -> String {
        let orientation_str = match self.orientation {
            DockOrientation::Horizontal => "horizontal",
            DockOrientation::Vertical => "vertical",
        };
        let position_str = match self.position {
            DockPosition::Top => "top",
            DockPosition::Bottom => "bottom",
            DockPosition::Left => "left",
            DockPosition::Right => "right",
        };
        format!("aurora-icon-dock aurora-dock-{} aurora-dock-{}", orientation_str, position_str)
    }

    /// Get animation CSS class
    pub fn animation_css(&self) -> String {
        format!(
            "aurora-dock-animation {{ scale-on-hover: {}; duration: {}ms; }}",
            self.animation.scale_on_hover, self.animation.duration_ms
        )
    }
}

impl Default for IconDock {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for IconDock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IconDock")
            .field("item_count", &self.item_count())
            .field("active_index", &self.active_index)
            .field("orientation", &self.orientation)
            .field("position", &self.position)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dock_item_creation() {
        let item = DockItem::new("home", "home-icon", "Home");
        assert_eq!(item.id(), "home");
        assert_eq!(item.icon(), "home-icon");
        assert_eq!(item.label(), "Home");
        assert!(!item.is_active());
    }

    #[test]
    fn test_dock_item_active() {
        let item = DockItem::new("home", "home-icon", "Home").active();
        assert!(item.is_active());
    }

    #[test]
    fn test_dock_item_badge() {
        let item = DockItem::new("mail", "mail-icon", "Mail").with_badge(5);
        assert_eq!(item.badge(), Some(5));
    }

    #[test]
    fn test_animation_spring() {
        let anim = DockAnimation::new().spring();
        assert_eq!(anim.scale_spring_stiffness(), 300.0);
    }

    #[test]
    fn test_animation_smooth() {
        let anim = DockAnimation::new().smooth();
        assert_eq!(anim.scale_spring_damping(), 1.0);
    }

    #[test]
    fn test_animation_fast() {
        let anim = DockAnimation::new().fast();
        assert_eq!(anim.duration_ms(), 100);
    }

    #[test]
    fn test_icon_dock_creation() {
        let dock = IconDock::new();
        assert_eq!(dock.item_count(), 0);
        assert_eq!(dock.orientation(), DockOrientation::Horizontal);
    }

    #[test]
    fn test_icon_dock_add_item() {
        let mut dock = IconDock::new();
        dock.add_item(DockItem::new("home", "home-icon", "Home"));
        assert_eq!(dock.item_count(), 1);
    }

    #[test]
    fn test_icon_dock_active_item() {
        let mut dock = IconDock::new();
        dock.add_item(DockItem::new("home", "home-icon", "Home").active());
        assert!(dock.active_item().is_some());
        assert_eq!(dock.active_item().unwrap().id(), "home");
    }

    #[test]
    fn test_icon_dock_set_active() {
        let mut dock = IconDock::new();
        dock.add_item(DockItem::new("home", "home-icon", "Home"));
        dock.add_item(DockItem::new("settings", "settings-icon", "Settings"));

        dock.set_active(1);
        assert_eq!(dock.active_index(), Some(1));
    }

    #[test]
    fn test_icon_dock_set_active_by_id() {
        let mut dock = IconDock::new();
        dock.add_item(DockItem::new("home", "home-icon", "Home"));
        dock.add_item(DockItem::new("settings", "settings-icon", "Settings"));

        dock.set_active_by_id("settings");
        assert_eq!(dock.active_item().unwrap().id(), "settings");
    }

    #[test]
    fn test_icon_dock_find_item() {
        let mut dock = IconDock::new();
        dock.add_item(DockItem::new("home", "home-icon", "Home"));

        let item = dock.find_item("home");
        assert!(item.is_some());
        assert_eq!(item.unwrap().label(), "Home");
    }

    #[test]
    fn test_icon_dock_dimension() {
        let mut dock = IconDock::new();
        dock.add_item(DockItem::new("home", "home-icon", "Home"));
        dock.add_item(DockItem::new("settings", "settings-icon", "Settings"));

        // 48px * 2 + 12px spacing = 108px
        assert_eq!(dock.dimension(), 108);
    }

    #[test]
    fn test_icon_dock_orientation() {
        let dock = IconDock::new().with_orientation(DockOrientation::Vertical);
        assert_eq!(dock.orientation(), DockOrientation::Vertical);
    }

    #[test]
    fn test_icon_dock_position() {
        let dock = IconDock::new().with_position(DockPosition::Left);
        assert_eq!(dock.position(), DockPosition::Left);
    }

    #[test]
    fn test_icon_dock_css_class() {
        let dock = IconDock::new()
            .with_orientation(DockOrientation::Vertical)
            .with_position(DockPosition::Left);

        let css_class = dock.css_class();
        assert!(css_class.contains("aurora-icon-dock"));
        assert!(css_class.contains("vertical"));
        assert!(css_class.contains("left"));
    }

    #[test]
    fn test_icon_dock_animation_css() {
        let dock = IconDock::new();
        let css = dock.animation_css();
        assert!(css.contains("scale-on-hover"));
        assert!(css.contains("duration"));
    }

    #[test]
    fn test_icon_dock_icon_size() {
        let dock = IconDock::new().with_icon_size(64);
        assert_eq!(dock.icon_size(), 64);
    }

    #[test]
    fn test_icon_dock_spacing() {
        let dock = IconDock::new().with_spacing(16);
        assert_eq!(dock.spacing(), 16);
    }

    #[test]
    fn test_icon_dock_default() {
        let dock = IconDock::default();
        assert_eq!(dock.item_count(), 0);
    }

    #[test]
    fn test_icon_dock_add_items_batch() {
        let mut dock = IconDock::new();
        let items = vec![
            DockItem::new("home", "home-icon", "Home"),
            DockItem::new("files", "files-icon", "Files"),
            DockItem::new("settings", "settings-icon", "Settings"),
        ];
        dock.add_items(items);
        assert_eq!(dock.item_count(), 3);
    }

    #[test]
    fn test_icon_dock_with_animation() {
        let anim = DockAnimation::new().spring();
        let dock = IconDock::new().with_animation(anim);
        assert_eq!(dock.animation().scale_spring_stiffness(), 300.0);
    }
}
