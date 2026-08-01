/// Aurora Dialog response type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogResponse {
    Accept,
    Cancel,
    Delete,
    Help,
    Custom(i32),
}

/// Aurora Dialog component
///
/// A modal or non-blocking dialog for user interactions.
#[derive(Debug, Clone)]
pub struct AuroraDialog {
    title: String,
    message: String,
    modal: bool,
    buttons: Vec<(String, i32)>,
}

impl AuroraDialog {
    /// Create a new dialog
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            message: String::new(),
            modal: false,
            buttons: Vec::new(),
        }
    }

    /// Set dialog message/content
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    /// Set as modal dialog
    pub fn modal(mut self, is_modal: bool) -> Self {
        self.modal = is_modal;
        self
    }

    /// Add a button with response
    pub fn add_button(mut self, label: &str, response: i32) -> Self {
        self.buttons.push((label.to_string(), response));
        self
    }

    /// Get dialog title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get dialog message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Check if modal
    pub fn is_modal(&self) -> bool {
        self.modal
    }

    /// Get buttons
    pub fn buttons(&self) -> &[(String, i32)] {
        &self.buttons
    }
}

impl Default for AuroraDialog {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_new() {
        let dialog = AuroraDialog::new("Test Dialog");
        assert_eq!(dialog.title(), "Test Dialog");
    }

    #[test]
    fn test_dialog_message() {
        let dialog = AuroraDialog::new("Test")
            .with_message("Hello World");
        assert_eq!(dialog.message(), "Hello World");
    }

    #[test]
    fn test_dialog_modal() {
        let dialog = AuroraDialog::new("Test")
            .modal(true);
        assert!(dialog.is_modal());
    }

    #[test]
    fn test_dialog_buttons() {
        let dialog = AuroraDialog::new("Confirm")
            .add_button("Yes", 1)
            .add_button("No", 0);
        assert_eq!(dialog.buttons().len(), 2);
    }

    #[test]
    fn test_dialog_chaining() {
        let _dialog = AuroraDialog::new("Confirm")
            .with_message("Are you sure?")
            .modal(true)
            .add_button("Yes", 1)
            .add_button("No", 0);
    }
}
