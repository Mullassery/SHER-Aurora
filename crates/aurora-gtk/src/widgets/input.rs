/// Aurora Input field type
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    /// Standard text input
    #[default]
    Text,
    /// Password (masked) input
    Password,
    /// Search input (with clear button)
    Search,
    /// Email input
    Email,
    /// Number input
    Number,
}

/// Aurora Input component
///
/// A text input field with support for multiple input types.
#[derive(Debug, Clone)]
pub struct Input {
    input_type: InputType,
    placeholder: String,
    text: String,
    error: bool,
    sensitive: bool,
    css_classes: Vec<String>,
}

impl Input {
    /// Create a new input field
    pub fn new(input_type: InputType) -> Self {
        Self {
            input_type,
            placeholder: String::new(),
            text: String::new(),
            error: false,
            sensitive: true,
            css_classes: vec!["aurora-input".to_string()],
        }
    }

    /// Set placeholder text
    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    /// Set input text
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Enable/disable the input
    pub fn set_sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = sensitive;
        self
    }

    /// Set error state
    pub fn set_error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Get current text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Clear input
    pub fn clear(&mut self) {
        self.text.clear();
    }

    /// Get input type
    pub fn input_type(&self) -> InputType {
        self.input_type
    }

    /// Get placeholder
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// Check if error state
    pub fn is_error(&self) -> bool {
        self.error
    }

    /// Check if sensitive
    pub fn is_sensitive(&self) -> bool {
        self.sensitive
    }

    /// Get CSS classes
    pub fn css_classes(&self) -> &[String] {
        &self.css_classes
    }

    /// Build a real `gtk4::Entry` widget from this descriptor.
    ///
    /// Constructs an actual GTK4 text-entry widget: placeholder, initial
    /// text, sensitivity, visibility (for password fields), and Aurora CSS
    /// classes are applied through the real `gtk4` widget API. Callers must
    /// have already initialized GTK before calling this.
    pub fn build(&self) -> gtk4::Entry {
        use gtk4::prelude::*;

        let entry = gtk4::Entry::builder()
            .placeholder_text(self.placeholder.as_str())
            .text(self.text.as_str())
            .sensitive(self.sensitive)
            .visibility(self.input_type != InputType::Password)
            .build();

        for class in &self.css_classes {
            entry.add_css_class(class);
        }

        if self.error {
            entry.add_css_class("error");
        }

        match self.input_type {
            InputType::Search => {
                entry.set_input_purpose(gtk4::InputPurpose::FreeForm);
                entry.add_css_class("aurora-search");
            }
            InputType::Email => entry.set_input_purpose(gtk4::InputPurpose::Email),
            InputType::Number => entry.set_input_purpose(gtk4::InputPurpose::Number),
            InputType::Password => entry.set_input_purpose(gtk4::InputPurpose::Password),
            InputType::Text => entry.set_input_purpose(gtk4::InputPurpose::FreeForm),
        }

        entry
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new(InputType::Text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_new() {
        let input = Input::new(InputType::Text);
        assert_eq!(input.input_type(), InputType::Text);
    }

    #[test]
    fn test_input_types() {
        assert_eq!(Input::new(InputType::Text).input_type(), InputType::Text);
        assert_eq!(
            Input::new(InputType::Password).input_type(),
            InputType::Password
        );
        assert_eq!(Input::new(InputType::Email).input_type(), InputType::Email);
    }

    #[test]
    fn test_input_placeholder() {
        let input = Input::new(InputType::Text).with_placeholder("Enter text");
        assert_eq!(input.placeholder(), "Enter text");
    }

    #[test]
    fn test_input_text() {
        let input = Input::new(InputType::Text).with_text("Hello");
        assert_eq!(input.text(), "Hello");
    }

    #[test]
    fn test_input_clear() {
        let mut input = Input::new(InputType::Text).with_text("Hello");
        input.clear();
        assert_eq!(input.text(), "");
    }

    #[test]
    fn test_input_error_state() {
        let input = Input::new(InputType::Text).set_error(true);
        assert!(input.is_error());
    }

    #[test]
    fn test_input_password_type() {
        let input = Input::new(InputType::Password);
        assert_eq!(input.input_type(), InputType::Password);
    }

    #[test]
    fn test_input_default() {
        let input = Input::default();
        assert_eq!(input.input_type(), InputType::Text);
    }

    #[test]
    fn test_input_chaining() {
        let _input = Input::new(InputType::Email)
            .with_placeholder("user@example.com")
            .with_text("test@example.com")
            .set_sensitive(true);
    }

    // Real GTK4 widget-construction test — see the comment in
    // `widgets::switch::tests` for why this is gated off macOS and how to
    // verify real GTK4 rendering locally on macOS instead.
    #[cfg(not(target_os = "macos"))]
    mod gtk_real {
        use super::*;

        #[gtk4::test]
        fn test_input_build_is_real_gtk4_widget() {
            use gtk4::prelude::*;
            let entry = Input::new(InputType::Text)
                .with_placeholder("Enter text")
                .with_text("Hello")
                .build();
            assert_eq!(entry.text(), "Hello");
            assert_eq!(entry.placeholder_text().unwrap(), "Enter text");
            assert!(entry.css_classes().iter().any(|c| c == "aurora-input"));
        }

        #[gtk4::test]
        fn test_password_input_hides_text_in_real_gtk4() {
            use gtk4::prelude::EntryExt;
            let entry = Input::new(InputType::Password).build();
            // GTK4's own visibility flag (masks characters), not our struct's
            assert!(!EntryExt::is_visible(&entry));
            let plain = Input::new(InputType::Text).build();
            assert!(EntryExt::is_visible(&plain));
        }
    }
}
