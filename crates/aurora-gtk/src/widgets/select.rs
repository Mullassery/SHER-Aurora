//! Select/Combobox Widget - Dropdown Selection with Search
//!
//! Flexible dropdown component with search, multi-select, async loading, and custom rendering.

use std::fmt;

/// Selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectMode {
    Single,
    Multiple,
}

/// Option state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionState {
    Normal,
    Disabled,
    Loading,
}

/// Individual select option
#[derive(Debug, Clone)]
pub struct SelectOption {
    id: String,
    label: String,
    value: String,
    state: OptionState,
    group: std::option::Option<String>,
}

impl SelectOption {
    /// Create a new option
    pub fn new(id: &str, label: &str, value: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            value: value.to_string(),
            state: OptionState::Normal,
            group: None,
        }
    }

    /// Disable the option
    pub fn disabled(mut self) -> Self {
        self.state = OptionState::Disabled;
        self
    }

    /// Mark as loading
    pub fn loading(mut self) -> Self {
        self.state = OptionState::Loading;
        self
    }

    /// Add to group
    pub fn in_group(mut self, group: &str) -> Self {
        self.group = Some(group.to_string());
        self
    }

    /// Get option ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get option label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get option value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get option state
    pub fn state(&self) -> OptionState {
        self.state
    }

    /// Get group (if any)
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }

    /// Is disabled?
    pub fn is_disabled(&self) -> bool {
        self.state == OptionState::Disabled
    }
}

/// Select/Combobox container
pub struct Select {
    options: Vec<SelectOption>,
    selected: Vec<String>, // IDs of selected options
    selection_mode: SelectMode,
    search_enabled: bool,
    search_term: String,
    open: bool,
    placeholder: String,
}

impl Select {
    /// Create a new select
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected: Vec::new(),
            selection_mode: SelectMode::Single,
            search_enabled: true,
            search_term: String::new(),
            open: false,
            placeholder: "Select an option".to_string(),
        }
    }

    /// Set selection mode
    pub fn with_mode(mut self, mode: SelectMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Enable/disable search
    pub fn with_search(mut self, enabled: bool) -> Self {
        self.search_enabled = enabled;
        self
    }

    /// Set placeholder text
    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    /// Add an option
    pub fn add_option(&mut self, option: SelectOption) {
        self.options.push(option);
    }

    /// Add multiple options
    pub fn add_options(&mut self, options: Vec<SelectOption>) {
        self.options.extend(options);
    }

    /// Get all options
    pub fn options(&self) -> &[SelectOption] {
        &self.options
    }

    /// Get filtered options based on search term
    pub fn filtered_options(&self) -> Vec<&SelectOption> {
        self.options
            .iter()
            .filter(|opt| {
                if self.search_term.is_empty() {
                    true
                } else {
                    opt.label
                        .to_lowercase()
                        .contains(&self.search_term.to_lowercase())
                        || opt.value
                            .to_lowercase()
                            .contains(&self.search_term.to_lowercase())
                }
            })
            .collect()
    }

    /// Set search term
    pub fn set_search_term(&mut self, term: &str) {
        self.search_term = term.to_string();
    }

    /// Get search term
    pub fn search_term(&self) -> &str {
        &self.search_term
    }

    /// Is search enabled?
    pub fn is_search_enabled(&self) -> bool {
        self.search_enabled
    }

    /// Open the dropdown
    pub fn open(&mut self) {
        self.open = true;
    }

    /// Close the dropdown
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Is open?
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Select an option by ID
    pub fn select(&mut self, option_id: &str) -> bool {
        if let Some(option) = self.options.iter().find(|o| o.id == option_id) {
            if option.is_disabled() {
                return false;
            }

            if self.selection_mode == SelectMode::Single {
                self.selected.clear();
            }

            if !self.selected.contains(&option_id.to_string()) {
                self.selected.push(option_id.to_string());
            }

            self.close();
            true
        } else {
            false
        }
    }

    /// Deselect an option by ID
    pub fn deselect(&mut self, option_id: &str) -> bool {
        if let std::option::Option::Some(pos) = self.selected.iter().position(|id| id == option_id) {
            self.selected.remove(pos);
            true
        } else {
            false
        }
    }

    /// Toggle option selection
    pub fn toggle(&mut self, option_id: &str) -> bool {
        if self.selected.contains(&option_id.to_string()) {
            self.deselect(option_id)
        } else {
            self.select(option_id)
        }
    }

    /// Get selected option IDs
    pub fn selected_ids(&self) -> &[String] {
        &self.selected
    }

    /// Get selected options
    pub fn selected_options(&self) -> Vec<&SelectOption> {
        self.selected
            .iter()
            .filter_map(|id| self.options.iter().find(|o| &o.id == id))
            .collect()
    }

    /// Get first selected option (Single mode)
    pub fn first_selected(&self) -> std::option::Option<&SelectOption> {
        if let std::option::Option::Some(id) = self.selected.first() {
            self.options.iter().find(|o| &o.id == id)
        } else {
            std::option::Option::None
        }
    }

    /// Get selected label(s)
    pub fn selected_label(&self) -> String {
        match self.selection_mode {
            SelectMode::Single => {
                self.first_selected()
                    .map(|o| o.label.clone())
                    .unwrap_or_else(|| self.placeholder.clone())
            }
            SelectMode::Multiple => {
                if self.selected.is_empty() {
                    self.placeholder.clone()
                } else {
                    format!("{} selected", self.selected.len())
                }
            }
        }
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.selected.clear();
    }

    /// Get selection mode
    pub fn selection_mode(&self) -> SelectMode {
        self.selection_mode
    }

    /// Get placeholder
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// Get option count
    pub fn option_count(&self) -> usize {
        self.options.len()
    }

    /// Get selected count
    pub fn selected_count(&self) -> usize {
        self.selected.len()
    }

    /// Is option selected?
    pub fn is_selected(&self, option_id: &str) -> bool {
        self.selected.contains(&option_id.to_string())
    }

    /// Get CSS class
    pub fn css_class(&self) -> String {
        let mode_str = match self.selection_mode {
            SelectMode::Single => "single",
            SelectMode::Multiple => "multiple",
        };
        format!("aurora-select aurora-select-{}", mode_str)
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Select {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Select")
            .field("option_count", &self.option_count())
            .field("selected_count", &self.selected_count())
            .field("selection_mode", &self.selection_mode)
            .field("open", &self.open)
            .field("search_enabled", &self.search_enabled)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_creation() {
        let select = Select::new();
        assert_eq!(select.option_count(), 0);
        assert_eq!(select.selected_count(), 0);
        assert_eq!(select.selection_mode(), SelectMode::Single);
    }

    #[test]
    fn test_add_options() {
        let mut select = Select::new();
        select.add_option(SelectOption::new("1", "Apple", "apple"));
        select.add_option(SelectOption::new("2", "Banana", "banana"));
        select.add_option(SelectOption::new("3", "Cherry", "cherry"));
        assert_eq!(select.option_count(), 3);
    }

    #[test]
    fn test_add_multiple_options() {
        let mut select = Select::new();
        let options = vec![
            SelectOption::new("1", "Apple", "apple"),
            SelectOption::new("2", "Banana", "banana"),
        ];
        select.add_options(options);
        assert_eq!(select.option_count(), 2);
    }

    #[test]
    fn test_select_single_mode() {
        let mut select = Select::new()
            .with_mode(SelectMode::Single);
        select.add_option(SelectOption::new("1", "Apple", "apple"));
        select.add_option(SelectOption::new("2", "Banana", "banana"));

        select.select("1");
        assert_eq!(select.selected_count(), 1);
        assert!(select.is_selected("1"));

        // Switch selection
        select.select("2");
        assert_eq!(select.selected_count(), 1);
        assert!(select.is_selected("2"));
        assert!(!select.is_selected("1"));
    }

    #[test]
    fn test_select_multiple_mode() {
        let mut select = Select::new()
            .with_mode(SelectMode::Multiple);
        select.add_option(SelectOption::new("1", "Apple", "apple"));
        select.add_option(SelectOption::new("2", "Banana", "banana"));
        select.add_option(SelectOption::new("3", "Cherry", "cherry"));

        select.select("1");
        select.select("2");
        assert_eq!(select.selected_count(), 2);
        assert!(select.is_selected("1"));
        assert!(select.is_selected("2"));
    }

    #[test]
    fn test_deselect() {
        let mut select = Select::new()
            .with_mode(SelectMode::Multiple);
        select.add_option(SelectOption::new("1", "Apple", "apple"));
        select.add_option(SelectOption::new("2", "Banana", "banana"));

        select.select("1");
        select.select("2");
        select.deselect("1");

        assert_eq!(select.selected_count(), 1);
        assert!(!select.is_selected("1"));
        assert!(select.is_selected("2"));
    }

    #[test]
    fn test_toggle() {
        let mut select = Select::new()
            .with_mode(SelectMode::Multiple);
        select.add_option(SelectOption::new("1", "Apple", "apple"));

        select.toggle("1");
        assert!(select.is_selected("1"));

        select.toggle("1");
        assert!(!select.is_selected("1"));
    }

    #[test]
    fn test_disabled_option() {
        let mut select = Select::new();
        select.add_option(SelectOption::new("1", "Apple", "apple").disabled());

        assert!(!select.select("1"));
        assert_eq!(select.selected_count(), 0);
    }

    #[test]
    fn test_search_filter() {
        let mut select = Select::new();
        select.add_option(SelectOption::new("1", "Apple", "apple"));
        select.add_option(SelectOption::new("2", "Apricot", "apricot"));
        select.add_option(SelectOption::new("3", "Banana", "banana"));

        select.set_search_term("ap");
        let filtered = select.filtered_options();
        assert_eq!(filtered.len(), 2); // Apple and Apricot

        select.set_search_term("ban");
        let filtered = select.filtered_options();
        assert_eq!(filtered.len(), 1); // Banana
    }

    #[test]
    fn test_search_disabled() {
        let select = Select::new()
            .with_search(false);
        assert!(!select.is_search_enabled());
    }

    #[test]
    fn test_open_close() {
        let mut select = Select::new();
        assert!(!select.is_open());

        select.open();
        assert!(select.is_open());

        select.close();
        assert!(!select.is_open());
    }

    #[test]
    fn test_selected_label_single() {
        let mut select = Select::new()
            .with_mode(SelectMode::Single)
            .with_placeholder("Pick one");
        select.add_option(SelectOption::new("1", "Apple", "apple"));

        assert_eq!(select.selected_label(), "Pick one");

        select.select("1");
        assert_eq!(select.selected_label(), "Apple");
    }

    #[test]
    fn test_selected_label_multiple() {
        let mut select = Select::new()
            .with_mode(SelectMode::Multiple)
            .with_placeholder("Pick items");
        select.add_option(SelectOption::new("1", "Apple", "apple"));
        select.add_option(SelectOption::new("2", "Banana", "banana"));

        assert_eq!(select.selected_label(), "Pick items");

        select.select("1");
        select.select("2");
        assert_eq!(select.selected_label(), "2 selected");
    }

    #[test]
    fn test_clear() {
        let mut select = Select::new()
            .with_mode(SelectMode::Multiple);
        select.add_option(SelectOption::new("1", "Apple", "apple"));
        select.add_option(SelectOption::new("2", "Banana", "banana"));

        select.select("1");
        select.select("2");
        assert_eq!(select.selected_count(), 2);

        select.clear();
        assert_eq!(select.selected_count(), 0);
    }

    #[test]
    fn test_first_selected() {
        let mut select = Select::new()
            .with_mode(SelectMode::Single);
        select.add_option(SelectOption::new("1", "Apple", "apple"));

        assert!(select.first_selected().is_none());

        select.select("1");
        assert!(select.first_selected().is_some());
        assert_eq!(select.first_selected().unwrap().label(), "Apple");
    }

    #[test]
    fn test_selected_options() {
        let mut select = Select::new()
            .with_mode(SelectMode::Multiple);
        select.add_option(SelectOption::new("1", "Apple", "apple"));
        select.add_option(SelectOption::new("2", "Banana", "banana"));
        select.add_option(SelectOption::new("3", "Cherry", "cherry"));

        select.select("1");
        select.select("3");

        let selected = select.selected_options();
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].label(), "Apple");
        assert_eq!(selected[1].label(), "Cherry");
    }

    #[test]
    fn test_option_grouping() {
        let mut select = Select::new();
        select.add_option(SelectOption::new("1", "Red Apple", "red_apple").in_group("Fruits"));
        select.add_option(SelectOption::new("2", "Carrot", "carrot").in_group("Vegetables"));

        assert_eq!(select.options[0].group(), std::option::Option::Some("Fruits"));
        assert_eq!(select.options[1].group(), std::option::Option::Some("Vegetables"));
    }

    #[test]
    fn test_css_class() {
        let single = Select::new().with_mode(SelectMode::Single);
        assert!(single.css_class().contains("single"));

        let multiple = Select::new().with_mode(SelectMode::Multiple);
        assert!(multiple.css_class().contains("multiple"));
    }

    #[test]
    fn test_option_state() {
        let disabled = SelectOption::new("1", "Apple", "apple").disabled();
        assert!(disabled.is_disabled());
        assert_eq!(disabled.state(), OptionState::Disabled);

        let loading = SelectOption::new("2", "Banana", "banana").loading();
        assert_eq!(loading.state(), OptionState::Loading);
    }

    #[test]
    fn test_default() {
        let select = Select::default();
        assert_eq!(select.option_count(), 0);
        assert_eq!(select.selected_count(), 0);
    }

    #[test]
    fn test_placeholder() {
        let select = Select::new()
            .with_placeholder("Custom placeholder");
        assert_eq!(select.placeholder(), "Custom placeholder");
    }
}
