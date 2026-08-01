//! Aurora Icon System - 1000+ Icon Management
//!
//! Unified icon system for GNOME applications with SVG templates, color utilities, and sizing.

pub mod core;
pub mod svg;
pub mod font;

use std::collections::HashMap;

/// Icon size categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconSize {
    ExtraSmall,  // 16px
    Small,       // 24px
    Medium,      // 32px
    Large,       // 48px
    ExtraLarge,  // 64px
}

impl IconSize {
    /// Get pixel size
    pub fn pixels(&self) -> u32 {
        match self {
            IconSize::ExtraSmall => 16,
            IconSize::Small => 24,
            IconSize::Medium => 32,
            IconSize::Large => 48,
            IconSize::ExtraLarge => 64,
        }
    }

    /// Get stroke weight for this size
    pub fn stroke_width(&self) -> f32 {
        match self {
            IconSize::ExtraSmall => 1.25,
            IconSize::Small => 1.5,
            IconSize::Medium => 1.75,
            IconSize::Large => 2.0,
            IconSize::ExtraLarge => 2.5,
        }
    }

    /// Get corner radius for this size
    pub fn corner_radius(&self) -> f32 {
        match self {
            IconSize::ExtraSmall => 2.0,
            IconSize::Small => 3.0,
            IconSize::Medium => 4.0,
            IconSize::Large => 6.0,
            IconSize::ExtraLarge => 8.0,
        }
    }
}

/// Icon category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconCategory {
    Navigation,   // 50+ icons: back, forward, home, menu, search
    Actions,      // 80+ icons: save, delete, edit, copy, paste
    Status,       // 60+ icons: success, error, warning, info, loading
    Media,        // 40+ icons: play, pause, volume, brightness
    System,       // 30+ icons: settings, user, battery, network
    Application, // 100+ icons: mail, calendar, contacts, files
}

impl IconCategory {
    /// Get category name
    pub fn name(&self) -> &str {
        match self {
            IconCategory::Navigation => "navigation",
            IconCategory::Actions => "actions",
            IconCategory::Status => "status",
            IconCategory::Media => "media",
            IconCategory::System => "system",
            IconCategory::Application => "application",
        }
    }

    /// Get target icon count for this category
    pub fn target_count(&self) -> usize {
        match self {
            IconCategory::Navigation => 50,
            IconCategory::Actions => 80,
            IconCategory::Status => 60,
            IconCategory::Media => 40,
            IconCategory::System => 30,
            IconCategory::Application => 100,
        }
    }
}

/// Icon semantic color context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconContext {
    Primary,      // Actions, primary buttons
    Success,      // Completion, confirmation
    Warning,      // Caution, attention needed
    Error,        // Destructive, failure
    Info,         // Information, help
    Neutral,      // Disabled, secondary
}

impl IconContext {
    /// Get hex color for this context
    pub fn color(&self) -> &str {
        match self {
            IconContext::Primary => "#003D99",    // Blue
            IconContext::Success => "#004400",    // Green
            IconContext::Warning => "#994400",    // Orange
            IconContext::Error => "#990000",      // Red
            IconContext::Info => "#0066CC",       // Light Blue
            IconContext::Neutral => "#666666",    // Gray
        }
    }

    /// Get light background for this context
    pub fn background_light(&self) -> &str {
        match self {
            IconContext::Primary => "#E3F2FD",
            IconContext::Success => "#E8F5E9",
            IconContext::Warning => "#FFF3E0",
            IconContext::Error => "#FFEBEE",
            IconContext::Info => "#E1F5FE",
            IconContext::Neutral => "#F5F5F5",
        }
    }
}

/// Icon metadata
#[derive(Debug, Clone)]
pub struct IconMetadata {
    id: String,
    name: String,
    category: IconCategory,
    tags: Vec<String>,
    context: IconContext,
    aliases: Vec<String>,  // Material Design "home", "house", etc.
}

impl IconMetadata {
    /// Create new icon metadata
    pub fn new(id: &str, name: &str, category: IconCategory) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category,
            tags: Vec::new(),
            context: IconContext::Primary,
            aliases: Vec::new(),
        }
    }

    /// Add tag
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Set context color
    pub fn with_context(mut self, context: IconContext) -> Self {
        self.context = context;
        self
    }

    /// Add alias
    pub fn with_alias(mut self, alias: &str) -> Self {
        self.aliases.push(alias.to_string());
        self
    }

    /// Getters
    pub fn id(&self) -> &str { &self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn category(&self) -> IconCategory { self.category }
    pub fn tags(&self) -> &[String] { &self.tags }
    pub fn context(&self) -> IconContext { self.context }
    pub fn aliases(&self) -> &[String] { &self.aliases }
}

/// Icon library
pub struct IconLibrary {
    icons: HashMap<String, IconMetadata>,
    by_category: HashMap<String, Vec<String>>,
    by_tag: HashMap<String, Vec<String>>,
}

impl IconLibrary {
    /// Create new icon library
    pub fn new() -> Self {
        Self {
            icons: HashMap::new(),
            by_category: HashMap::new(),
            by_tag: HashMap::new(),
        }
    }

    /// Register icon
    pub fn register(&mut self, icon: IconMetadata) {
        let category_name = icon.category.name().to_string();

        // Add to category index
        self.by_category
            .entry(category_name)
            .or_insert_with(Vec::new)
            .push(icon.id.clone());

        // Add to tag index
        for tag in &icon.tags {
            self.by_tag
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(icon.id.clone());
        }

        self.icons.insert(icon.id.clone(), icon);
    }

    /// Get icon metadata
    pub fn get(&self, id: &str) -> std::option::Option<&IconMetadata> {
        self.icons.get(id)
    }

    /// Find by name
    pub fn find_by_name(&self, name: &str) -> std::option::Option<&IconMetadata> {
        self.icons.values().find(|i| i.name == name)
    }

    /// Get all icons in category
    pub fn by_category(&self, category: IconCategory) -> Vec<&IconMetadata> {
        let category_name = category.name();
        self.by_category
            .get(category_name)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.icons.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all icons with tag
    pub fn by_tag(&self, tag: &str) -> Vec<&IconMetadata> {
        self.by_tag
            .get(tag)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.icons.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Search by name or tag
    pub fn search(&self, query: &str) -> Vec<&IconMetadata> {
        let lower = query.to_lowercase();
        self.icons
            .values()
            .filter(|i| {
                i.name.to_lowercase().contains(&lower)
                    || i.tags.iter().any(|t| t.to_lowercase().contains(&lower))
            })
            .collect()
    }

    /// Get total icon count
    pub fn total_count(&self) -> usize {
        self.icons.len()
    }

    /// Get category count
    pub fn category_count(&self, category: IconCategory) -> usize {
        self.by_category(category).len()
    }

    /// Get all icons
    pub fn all(&self) -> Vec<&IconMetadata> {
        self.icons.values().collect()
    }
}

impl Default for IconLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_size_pixels() {
        assert_eq!(IconSize::ExtraSmall.pixels(), 16);
        assert_eq!(IconSize::Small.pixels(), 24);
        assert_eq!(IconSize::Medium.pixels(), 32);
        assert_eq!(IconSize::Large.pixels(), 48);
        assert_eq!(IconSize::ExtraLarge.pixels(), 64);
    }

    #[test]
    fn test_icon_size_stroke_width() {
        assert_eq!(IconSize::ExtraSmall.stroke_width(), 1.25);
        assert_eq!(IconSize::Small.stroke_width(), 1.5);
        assert_eq!(IconSize::Medium.stroke_width(), 1.75);
        assert_eq!(IconSize::Large.stroke_width(), 2.0);
        assert_eq!(IconSize::ExtraLarge.stroke_width(), 2.5);
    }

    #[test]
    fn test_icon_context_colors() {
        assert_eq!(IconContext::Primary.color(), "#003D99");
        assert_eq!(IconContext::Success.color(), "#004400");
        assert_eq!(IconContext::Error.color(), "#990000");
    }

    #[test]
    fn test_icon_metadata_creation() {
        let icon = IconMetadata::new("home", "Home", IconCategory::Navigation);
        assert_eq!(icon.id(), "home");
        assert_eq!(icon.name(), "Home");
        assert_eq!(icon.category(), IconCategory::Navigation);
    }

    #[test]
    fn test_icon_metadata_tags() {
        let icon = IconMetadata::new("save", "Save", IconCategory::Actions)
            .with_tag("file")
            .with_tag("action");

        assert_eq!(icon.tags().len(), 2);
        assert!(icon.tags().contains(&"file".to_string()));
    }

    #[test]
    fn test_icon_library_register() {
        let mut library = IconLibrary::new();
        let icon = IconMetadata::new("home", "Home", IconCategory::Navigation);
        library.register(icon);

        assert_eq!(library.total_count(), 1);
        assert!(library.get("home").is_some());
    }

    #[test]
    fn test_icon_library_by_category() {
        let mut library = IconLibrary::new();
        library.register(IconMetadata::new("home", "Home", IconCategory::Navigation));
        library.register(IconMetadata::new("save", "Save", IconCategory::Actions));

        let nav_icons = library.by_category(IconCategory::Navigation);
        assert_eq!(nav_icons.len(), 1);

        let action_icons = library.by_category(IconCategory::Actions);
        assert_eq!(action_icons.len(), 1);
    }

    #[test]
    fn test_icon_library_by_tag() {
        let mut library = IconLibrary::new();
        library.register(
            IconMetadata::new("save", "Save", IconCategory::Actions)
                .with_tag("file"),
        );
        library.register(
            IconMetadata::new("new", "New", IconCategory::Actions)
                .with_tag("file"),
        );

        let file_icons = library.by_tag("file");
        assert_eq!(file_icons.len(), 2);
    }

    #[test]
    fn test_icon_library_search() {
        let mut library = IconLibrary::new();
        library.register(IconMetadata::new("home", "Home", IconCategory::Navigation)
            .with_tag("navigation"));
        library.register(IconMetadata::new("house", "House", IconCategory::Navigation)
            .with_tag("home"));

        let results = library.search("home");
        assert_eq!(results.len(), 2); // "home" in name + "home" in tags
    }

    #[test]
    fn test_icon_library_find_by_name() {
        let mut library = IconLibrary::new();
        library.register(IconMetadata::new("home", "Home", IconCategory::Navigation));

        let icon = library.find_by_name("Home");
        assert!(icon.is_some());
        assert_eq!(icon.unwrap().id(), "home");
    }

    #[test]
    fn test_icon_category_target_count() {
        assert_eq!(IconCategory::Navigation.target_count(), 50);
        assert_eq!(IconCategory::Actions.target_count(), 80);
        assert_eq!(IconCategory::Status.target_count(), 60);
        assert_eq!(IconCategory::Media.target_count(), 40);
        assert_eq!(IconCategory::System.target_count(), 30);
        assert_eq!(IconCategory::Application.target_count(), 100);
    }

    #[test]
    fn test_icon_metadata_with_context() {
        let icon = IconMetadata::new("delete", "Delete", IconCategory::Actions)
            .with_context(IconContext::Error);
        assert_eq!(icon.context(), IconContext::Error);
        assert_eq!(icon.context().color(), "#990000");
    }

    #[test]
    fn test_icon_size_corner_radius() {
        assert_eq!(IconSize::Small.corner_radius(), 3.0);
        assert_eq!(IconSize::Medium.corner_radius(), 4.0);
        assert_eq!(IconSize::Large.corner_radius(), 6.0);
    }

    #[test]
    fn test_default_library() {
        let library = IconLibrary::default();
        assert_eq!(library.total_count(), 0);
    }

    #[test]
    fn test_icon_category_names() {
        assert_eq!(IconCategory::Navigation.name(), "navigation");
        assert_eq!(IconCategory::Actions.name(), "actions");
        assert_eq!(IconCategory::Status.name(), "status");
    }
}
