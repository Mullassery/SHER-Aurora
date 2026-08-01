//! Storybook - Interactive Component Showcase
//!
//! Documentation and showcase system for Aurora components.
//! Generates interactive HTML documentation with examples, props, and code snippets.

use std::collections::HashMap;

/// Component story (example)
#[derive(Debug, Clone)]
pub struct Story {
    id: String,
    title: String,
    description: String,
    code_example: String,
    props: HashMap<String, String>,
    accessibility_notes: String,
}

impl Story {
    /// Create a new story
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            description: String::new(),
            code_example: String::new(),
            props: HashMap::new(),
            accessibility_notes: String::new(),
        }
    }

    /// Add description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Add code example
    pub fn with_code(mut self, code: &str) -> Self {
        self.code_example = code.to_string();
        self
    }

    /// Add prop documentation
    pub fn add_prop(mut self, name: &str, doc: &str) -> Self {
        self.props.insert(name.to_string(), doc.to_string());
        self
    }

    /// Add accessibility notes
    pub fn with_a11y_notes(mut self, notes: &str) -> Self {
        self.accessibility_notes = notes.to_string();
        self
    }

    /// Getters
    pub fn id(&self) -> &str { &self.id }
    pub fn title(&self) -> &str { &self.title }
    pub fn description(&self) -> &str { &self.description }
    pub fn code_example(&self) -> &str { &self.code_example }
    pub fn props(&self) -> &HashMap<String, String> { &self.props }
    pub fn accessibility_notes(&self) -> &str { &self.accessibility_notes }
}

/// Component documentation
#[derive(Debug, Clone)]
pub struct ComponentDoc {
    id: String,
    name: String,
    category: String,
    stories: Vec<Story>,
    overview: String,
    imports: String,
}

impl ComponentDoc {
    /// Create new component documentation
    pub fn new(id: &str, name: &str, category: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            stories: Vec::new(),
            overview: String::new(),
            imports: String::new(),
        }
    }

    /// Set overview
    pub fn with_overview(mut self, overview: &str) -> Self {
        self.overview = overview.to_string();
        self
    }

    /// Set import statement
    pub fn with_imports(mut self, imports: &str) -> Self {
        self.imports = imports.to_string();
        self
    }

    /// Add story
    pub fn add_story(&mut self, story: Story) {
        self.stories.push(story);
    }

    /// Add multiple stories
    pub fn add_stories(&mut self, stories: Vec<Story>) {
        self.stories.extend(stories);
    }

    /// Getters
    pub fn id(&self) -> &str { &self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn category(&self) -> &str { &self.category }
    pub fn stories(&self) -> &[Story] { &self.stories }
    pub fn overview(&self) -> &str { &self.overview }
    pub fn imports(&self) -> &str { &self.imports }
    pub fn story_count(&self) -> usize { self.stories.len() }
}

/// Storybook catalog
pub struct Storybook {
    components: HashMap<String, ComponentDoc>,
    categories: HashMap<String, Vec<String>>,  // category -> component ids
}

impl Storybook {
    /// Create new storybook
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    /// Register component
    pub fn register(&mut self, component: ComponentDoc) {
        let category = component.category.clone();
        let id = component.id.clone();

        self.categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id.clone());

        self.components.insert(id, component);
    }

    /// Get component by ID
    pub fn get(&self, id: &str) -> std::option::Option<&ComponentDoc> {
        self.components.get(id)
    }

    /// Get all components in category
    pub fn by_category(&self, category: &str) -> Vec<&ComponentDoc> {
        self.categories
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.components.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all components
    pub fn all(&self) -> Vec<&ComponentDoc> {
        self.components.values().collect()
    }

    /// Get component count
    pub fn total_count(&self) -> usize {
        self.components.len()
    }

    /// Get categories
    pub fn categories(&self) -> Vec<&str> {
        self.categories.keys().map(|s| s.as_str()).collect()
    }

    /// Generate HTML for component
    pub fn generate_html(&self, component_id: &str) -> std::option::Option<String> {
        self.get(component_id).map(|component| {
            let mut html = String::new();
            html.push_str(&format!("<h1>{}</h1>\n", component.name));
            html.push_str(&format!("<p>{}</p>\n", component.overview));
            html.push_str(&format!("<pre><code>{}</code></pre>\n", component.imports));

            for story in &component.stories {
                html.push_str(&format!("<h2>{}</h2>\n", story.title));
                html.push_str(&format!("<p>{}</p>\n", story.description));
                html.push_str(&format!("<pre><code>{}</code></pre>\n", story.code_example));

                if !story.accessibility_notes.is_empty() {
                    html.push_str(&format!(
                        "<div class='a11y-notes'><p><strong>Accessibility:</strong> {}</p></div>\n",
                        story.accessibility_notes
                    ));
                }

                if !story.props.is_empty() {
                    html.push_str("<h3>Props</h3>\n<ul>\n");
                    for (prop, doc) in &story.props {
                        html.push_str(&format!("<li><code>{}</code>: {}</li>\n", prop, doc));
                    }
                    html.push_str("</ul>\n");
                }
            }

            html
        })
    }

    /// Generate catalog index
    pub fn generate_catalog_index(&self) -> String {
        let mut html = String::from("<h1>Aurora Component Catalog</h1>\n");
        html.push_str(&format!("<p>Total Components: {}</p>\n", self.total_count()));

        for category in self.categories() {
            html.push_str(&format!("<h2>{}</h2>\n", category));
            html.push_str("<ul>\n");
            for component in self.by_category(category) {
                html.push_str(&format!("<li><a href='#{}' onclick='loadComponent(\"{}\")'>{}</a></li>\n",
                    component.id, component.id, component.name));
            }
            html.push_str("</ul>\n");
        }

        html
    }
}

impl Default for Storybook {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_creation() {
        let story = Story::new("button-primary", "Primary Button");
        assert_eq!(story.id(), "button-primary");
        assert_eq!(story.title(), "Primary Button");
    }

    #[test]
    fn test_story_with_metadata() {
        let story = Story::new("button-primary", "Primary Button")
            .with_description("A button with primary style")
            .with_code("Button::new(\"Click me\").with_style(ButtonStyle::Filled)")
            .add_prop("style", "The button style (Filled, Tinted, Outlined, Ghost)")
            .with_a11y_notes("Ensure sufficient color contrast (7:1)");

        assert_eq!(story.description(), "A button with primary style");
        assert_eq!(story.props().len(), 1);
        assert!(!story.accessibility_notes().is_empty());
    }

    #[test]
    fn test_component_doc_creation() {
        let component = ComponentDoc::new("button", "Button", "Components");
        assert_eq!(component.name(), "Button");
        assert_eq!(component.category(), "Components");
    }

    #[test]
    fn test_component_doc_with_stories() {
        let mut component = ComponentDoc::new("button", "Button", "Components");
        component.add_story(Story::new("button-primary", "Primary Button"));
        component.add_story(Story::new("button-secondary", "Secondary Button"));

        assert_eq!(component.story_count(), 2);
    }

    #[test]
    fn test_storybook_registration() {
        let mut storybook = Storybook::new();
        let component = ComponentDoc::new("button", "Button", "Components");
        storybook.register(component);

        assert_eq!(storybook.total_count(), 1);
        assert!(storybook.get("button").is_some());
    }

    #[test]
    fn test_storybook_by_category() {
        let mut storybook = Storybook::new();
        storybook.register(ComponentDoc::new("button", "Button", "Forms"));
        storybook.register(ComponentDoc::new("input", "Input", "Forms"));
        storybook.register(ComponentDoc::new("card", "Card", "Containers"));

        let form_components = storybook.by_category("Forms");
        assert_eq!(form_components.len(), 2);

        let container_components = storybook.by_category("Containers");
        assert_eq!(container_components.len(), 1);
    }

    #[test]
    fn test_storybook_categories() {
        let mut storybook = Storybook::new();
        storybook.register(ComponentDoc::new("button", "Button", "Forms"));
        storybook.register(ComponentDoc::new("card", "Card", "Containers"));

        let categories = storybook.categories();
        assert_eq!(categories.len(), 2);
    }

    #[test]
    fn test_storybook_generate_html() {
        let mut storybook = Storybook::new();
        let mut component = ComponentDoc::new("button", "Button", "Components")
            .with_overview("A flexible button component");
        component.add_story(Story::new("primary", "Primary Button")
            .with_description("Default primary button")
            .with_code("Button::new(\"Click\")"));

        storybook.register(component);

        let html = storybook.generate_html("button");
        assert!(html.is_some());
        assert!(html.unwrap().contains("Button"));
    }

    #[test]
    fn test_storybook_generate_catalog_index() {
        let mut storybook = Storybook::new();
        storybook.register(ComponentDoc::new("button", "Button", "Forms"));
        storybook.register(ComponentDoc::new("card", "Card", "Containers"));

        let index = storybook.generate_catalog_index();
        assert!(index.contains("Aurora Component Catalog"));
        assert!(index.contains("Total Components: 2"));
    }

    #[test]
    fn test_storybook_all() {
        let mut storybook = Storybook::new();
        storybook.register(ComponentDoc::new("button", "Button", "Forms"));
        storybook.register(ComponentDoc::new("card", "Card", "Containers"));

        let all = storybook.all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_story_props() {
        let story = Story::new("button", "Button")
            .add_prop("text", "Button label text")
            .add_prop("style", "Button style variant");

        assert_eq!(story.props().len(), 2);
    }

    #[test]
    fn test_default_storybook() {
        let storybook = Storybook::default();
        assert_eq!(storybook.total_count(), 0);
    }
}
