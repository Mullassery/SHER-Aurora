//! Aurora Files - File Browser Example
//!
//! Demonstrates Aurora components in a real GNOME application.

/// File browser application
pub struct AuroraFiles {
    title: String,
    files: Vec<String>,
    selected: Option<usize>,
}

impl AuroraFiles {
    /// Create a new file browser
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            files: vec![
                "Documents".to_string(),
                "Downloads".to_string(),
                "Pictures".to_string(),
                "Videos".to_string(),
                "Music".to_string(),
            ],
            selected: None,
        }
    }

    /// Get title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get files
    pub fn files(&self) -> &[String] {
        &self.files
    }

    /// Get selected file
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Select a file
    pub fn select(&mut self, index: usize) {
        if index < self.files.len() {
            self.selected = Some(index);
        }
    }

    /// Add a file
    pub fn add_file(&mut self, filename: &str) {
        self.files.push(filename.to_string());
    }

    /// Delete selected file
    pub fn delete_selected(&mut self) {
        if let Some(index) = self.selected {
            if index < self.files.len() {
                self.files.remove(index);
                self.selected = None;
            }
        }
    }

    /// Get file count
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

impl Default for AuroraFiles {
    fn default() -> Self {
        Self::new("Aurora Files")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aurora_files_creation() {
        let app = AuroraFiles::new("My Files");
        assert_eq!(app.title(), "My Files");
        assert!(app.file_count() > 0);
    }

    #[test]
    fn test_file_selection() {
        let mut app = AuroraFiles::new("Files");
        app.select(0);
        assert_eq!(app.selected(), Some(0));
    }

    #[test]
    fn test_add_file() {
        let mut app = AuroraFiles::new("Files");
        let initial_count = app.file_count();
        app.add_file("NewFile.txt");
        assert_eq!(app.file_count(), initial_count + 1);
    }

    #[test]
    fn test_delete_file() {
        let mut app = AuroraFiles::new("Files");
        let initial_count = app.file_count();
        app.select(0);
        app.delete_selected();
        assert_eq!(app.file_count(), initial_count - 1);
    }

    #[test]
    fn test_default() {
        let app = AuroraFiles::default();
        assert_eq!(app.title(), "Aurora Files");
    }
}
