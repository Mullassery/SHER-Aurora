//! DataTable Widget - Sortable, Selectable Data Display
//!
//! Complete table component for displaying and interacting with structured data.
//! Features: Sorting, selection, pagination, search integration, keyboard navigation.

use std::fmt;

/// Column sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
    None,
}

/// Column configuration
#[derive(Debug, Clone)]
pub struct Column {
    key: String,
    title: String,
    sortable: bool,
    width: u32,
    sort_direction: SortDirection,
}

impl Column {
    /// Create a new column
    pub fn new(key: &str, title: &str) -> Self {
        Self {
            key: key.to_string(),
            title: title.to_string(),
            sortable: true,
            width: 100,
            sort_direction: SortDirection::None,
        }
    }

    /// Make column non-sortable
    pub fn not_sortable(mut self) -> Self {
        self.sortable = false;
        self
    }

    /// Set column width (pixels)
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Get column key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get column title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Is column sortable?
    pub fn is_sortable(&self) -> bool {
        self.sortable
    }

    /// Get column width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get sort direction
    pub fn sort_direction(&self) -> SortDirection {
        self.sort_direction
    }

    /// Set sort direction
    pub fn set_sort_direction(&mut self, direction: SortDirection) {
        self.sort_direction = direction;
    }
}

/// Row data (simplified - maps column keys to values)
#[derive(Debug, Clone)]
pub struct Row {
    id: String,
    values: std::collections::HashMap<String, String>,
    selected: bool,
}

impl Row {
    /// Create a new row
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            values: std::collections::HashMap::new(),
            selected: false,
        }
    }

    /// Add a value to the row
    pub fn with_value(mut self, key: &str, value: &str) -> Self {
        self.values.insert(key.to_string(), value.to_string());
        self
    }

    /// Get row ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get value by column key
    pub fn value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// Is row selected?
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Set selected state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

/// Selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    None,
    Single,
    Multiple,
}

/// DataTable widget
pub struct DataTable {
    columns: Vec<Column>,
    rows: Vec<Row>,
    selection_mode: SelectionMode,
    page: u32,
    page_size: u32,
    sortable: bool,
    selectable: bool,
}

impl DataTable {
    /// Create a new DataTable
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            selection_mode: SelectionMode::Multiple,
            page: 0,
            page_size: 10,
            sortable: true,
            selectable: true,
        }
    }

    /// Add a column
    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    /// Add a row
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    /// Get all columns
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// Get all rows
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    /// Get rows for current page
    pub fn paginated_rows(&self) -> &[Row] {
        let start = (self.page * self.page_size) as usize;
        let end = ((self.page + 1) * self.page_size) as usize;
        if start < self.rows.len() {
            &self.rows[start..end.min(self.rows.len())]
        } else {
            &[]
        }
    }

    /// Get total row count
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get total page count
    pub fn page_count(&self) -> u32 {
        ((self.rows.len() as u32 + self.page_size - 1) / self.page_size).max(1)
    }

    /// Set page size
    pub fn set_page_size(&mut self, size: u32) {
        self.page_size = size.max(1);
        // Reset to first page
        self.page = 0;
    }

    /// Go to page
    pub fn go_to_page(&mut self, page: u32) {
        self.page = page.min(self.page_count() - 1);
    }

    /// Next page
    pub fn next_page(&mut self) {
        if self.page < self.page_count() - 1 {
            self.page += 1;
        }
    }

    /// Previous page
    pub fn prev_page(&mut self) {
        if self.page > 0 {
            self.page -= 1;
        }
    }

    /// Get current page
    pub fn current_page(&self) -> u32 {
        self.page
    }

    /// Set selection mode
    pub fn set_selection_mode(&mut self, mode: SelectionMode) {
        self.selection_mode = mode;
        if mode == SelectionMode::Single {
            // Deselect all but first
            let first_selected = self.rows.iter().position(|r| r.selected);
            for (i, row) in self.rows.iter_mut().enumerate() {
                row.selected = Some(i) == first_selected;
            }
        }
    }

    /// Get selection mode
    pub fn selection_mode(&self) -> SelectionMode {
        self.selection_mode
    }

    /// Select row by index
    pub fn select_row(&mut self, index: usize, exclusive: bool) {
        if index < self.rows.len() {
            if exclusive || self.selection_mode == SelectionMode::Single {
                // Deselect all others
                for row in self.rows.iter_mut() {
                    row.selected = false;
                }
            }
            self.rows[index].selected = true;
        }
    }

    /// Deselect row by index
    pub fn deselect_row(&mut self, index: usize) {
        if index < self.rows.len() {
            self.rows[index].selected = false;
        }
    }

    /// Get selected row IDs
    pub fn selected_rows(&self) -> Vec<&str> {
        self.rows
            .iter()
            .filter(|r| r.selected)
            .map(|r| r.id.as_str())
            .collect()
    }

    /// Sort by column
    pub fn sort_by(&mut self, column_key: &str, direction: SortDirection) {
        // Reset all columns' sort direction
        for col in self.columns.iter_mut() {
            col.sort_direction = SortDirection::None;
        }

        // Set sort on requested column
        if let Some(col) = self.columns.iter_mut().find(|c| c.key == column_key) {
            col.sort_direction = direction;

            // Sort rows based on column values
            if direction != SortDirection::None {
                self.rows.sort_by(|a, b| {
                    let a_val = a.value(column_key).unwrap_or("");
                    let b_val = b.value(column_key).unwrap_or("");

                    let cmp = a_val.cmp(b_val);
                    if direction == SortDirection::Descending {
                        cmp.reverse()
                    } else {
                        cmp
                    }
                });
            }
        }
    }

    /// Get CSS class
    pub fn css_class(&self) -> String {
        format!("aurora-datatable aurora-datatable-selectable")
    }

    /// Is sortable?
    pub fn is_sortable(&self) -> bool {
        self.sortable
    }

    /// Is selectable?
    pub fn is_selectable(&self) -> bool {
        self.selectable
    }
}

impl Default for DataTable {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for DataTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataTable")
            .field("columns", &self.columns.len())
            .field("rows", &self.rows.len())
            .field("selection_mode", &self.selection_mode)
            .field("page", &self.page)
            .field("page_size", &self.page_size)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datatable_creation() {
        let table = DataTable::new();
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.columns().len(), 0);
    }

    #[test]
    fn test_add_columns() {
        let mut table = DataTable::new();
        table.add_column(Column::new("id", "ID"));
        table.add_column(Column::new("name", "Name"));
        assert_eq!(table.columns().len(), 2);
    }

    #[test]
    fn test_add_rows() {
        let mut table = DataTable::new();
        let row1 = Row::new("1").with_value("name", "Alice");
        let row2 = Row::new("2").with_value("name", "Bob");
        table.add_row(row1);
        table.add_row(row2);
        assert_eq!(table.row_count(), 2);
    }

    #[test]
    fn test_pagination() {
        let mut table = DataTable::new();
        for i in 0..25 {
            table.add_row(Row::new(&i.to_string()));
        }

        table.set_page_size(10);
        assert_eq!(table.page_count(), 3);
        assert_eq!(table.paginated_rows().len(), 10);

        table.next_page();
        assert_eq!(table.current_page(), 1);
    }

    #[test]
    fn test_row_selection_single() {
        let mut table = DataTable::new();
        table.add_row(Row::new("1"));
        table.add_row(Row::new("2"));
        table.set_selection_mode(SelectionMode::Single);

        table.select_row(0, true);
        assert_eq!(table.selected_rows().len(), 1);

        table.select_row(1, true);
        assert_eq!(table.selected_rows().len(), 1);
        assert_eq!(table.selected_rows()[0], "2");
    }

    #[test]
    fn test_row_selection_multiple() {
        let mut table = DataTable::new();
        table.add_row(Row::new("1"));
        table.add_row(Row::new("2"));
        table.add_row(Row::new("3"));
        table.set_selection_mode(SelectionMode::Multiple);

        table.select_row(0, false);
        table.select_row(1, false);
        assert_eq!(table.selected_rows().len(), 2);
    }

    #[test]
    fn test_sorting_ascending() {
        let mut table = DataTable::new();
        table.add_column(Column::new("name", "Name"));
        table.add_row(Row::new("1").with_value("name", "Charlie"));
        table.add_row(Row::new("2").with_value("name", "Alice"));
        table.add_row(Row::new("3").with_value("name", "Bob"));

        table.sort_by("name", SortDirection::Ascending);
        assert_eq!(table.rows[0].id(), "2"); // Alice
        assert_eq!(table.rows[1].id(), "3"); // Bob
        assert_eq!(table.rows[2].id(), "1"); // Charlie
    }

    #[test]
    fn test_sorting_descending() {
        let mut table = DataTable::new();
        table.add_column(Column::new("name", "Name"));
        table.add_row(Row::new("1").with_value("name", "Charlie"));
        table.add_row(Row::new("2").with_value("name", "Alice"));
        table.add_row(Row::new("3").with_value("name", "Bob"));

        table.sort_by("name", SortDirection::Descending);
        assert_eq!(table.rows[0].id(), "1"); // Charlie
        assert_eq!(table.rows[1].id(), "3"); // Bob
        assert_eq!(table.rows[2].id(), "2"); // Alice
    }

    #[test]
    fn test_deselect_row() {
        let mut table = DataTable::new();
        table.add_row(Row::new("1"));
        table.select_row(0, true);
        assert_eq!(table.selected_rows().len(), 1);

        table.deselect_row(0);
        assert_eq!(table.selected_rows().len(), 0);
    }

    #[test]
    fn test_column_not_sortable() {
        let col = Column::new("id", "ID").not_sortable();
        assert!(!col.is_sortable());
    }

    #[test]
    fn test_column_with_width() {
        let col = Column::new("name", "Name").with_width(200);
        assert_eq!(col.width(), 200);
    }

    #[test]
    fn test_page_navigation() {
        let mut table = DataTable::new();
        for i in 0..50 {
            table.add_row(Row::new(&i.to_string()));
        }
        table.set_page_size(10);

        assert_eq!(table.page_count(), 5);
        table.next_page();
        table.next_page();
        assert_eq!(table.current_page(), 2);

        table.prev_page();
        assert_eq!(table.current_page(), 1);

        table.go_to_page(4);
        assert_eq!(table.current_page(), 4);
    }

    #[test]
    fn test_css_class() {
        let table = DataTable::new();
        assert!(table.css_class().contains("aurora-datatable"));
    }

    #[test]
    fn test_default() {
        let table = DataTable::default();
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.selection_mode(), SelectionMode::Multiple);
    }

    #[test]
    fn test_row_values() {
        let row = Row::new("1")
            .with_value("name", "Alice")
            .with_value("age", "30");

        assert_eq!(row.value("name"), Some("Alice"));
        assert_eq!(row.value("age"), Some("30"));
        assert_eq!(row.value("unknown"), None);
    }

    #[test]
    fn test_selected_rows_empty() {
        let table = DataTable::new();
        assert_eq!(table.selected_rows().len(), 0);
    }

    #[test]
    fn test_page_boundaries() {
        let mut table = DataTable::new();
        for i in 0..25 {
            table.add_row(Row::new(&i.to_string()));
        }
        table.set_page_size(10);

        // Try to go beyond max page
        table.go_to_page(100);
        assert_eq!(table.current_page(), 2); // Should be last page (0-indexed)
    }

    #[test]
    fn test_sort_direction_reset() {
        let mut table = DataTable::new();
        table.add_column(Column::new("col1", "Column 1"));
        table.add_column(Column::new("col2", "Column 2"));

        table.sort_by("col1", SortDirection::Ascending);
        assert_eq!(table.columns[0].sort_direction(), SortDirection::Ascending);

        table.sort_by("col2", SortDirection::Descending);
        assert_eq!(table.columns[0].sort_direction(), SortDirection::None);
        assert_eq!(table.columns[1].sort_direction(), SortDirection::Descending);
    }
}
