//! Breadcrumb Widget - File Path and Hierarchical Navigation
//!
//! Navigation component showing current location in a hierarchy with clickable segments.

use std::fmt;

/// Breadcrumb segment (path component)
#[derive(Debug, Clone)]
pub struct Segment {
    id: String,
    label: String,
    path: String,
    active: bool,
}

impl Segment {
    /// Create a new segment
    pub fn new(id: &str, label: &str, path: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            path: path.to_string(),
            active: false,
        }
    }

    /// Mark as active (current page)
    pub fn active(mut self) -> Self {
        self.active = true;
        self
    }

    /// Get segment ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get segment label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get segment path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Is this segment active?
    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// Breadcrumb overflow behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowBehavior {
    ShowAll,  // Show all segments
    Truncate, // Show first and last, hide middle
    Collapse, // Show only last segment (+ overflow menu)
}

/// Breadcrumb navigation component
pub struct Breadcrumb {
    segments: Vec<Segment>,
    overflow_behavior: OverflowBehavior,
    separator: String,
    max_visible: usize,
}

impl Breadcrumb {
    /// Create a new breadcrumb
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            overflow_behavior: OverflowBehavior::ShowAll,
            separator: "/".to_string(),
            max_visible: 10,
        }
    }

    /// Set overflow behavior
    pub fn with_overflow(mut self, behavior: OverflowBehavior) -> Self {
        self.overflow_behavior = behavior;
        self
    }

    /// Set separator (/, >, •, etc.)
    pub fn with_separator(mut self, separator: &str) -> Self {
        self.separator = separator.to_string();
        self
    }

    /// Set max visible segments
    pub fn with_max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Add a segment
    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    /// Add multiple segments
    pub fn add_segments(&mut self, segments: Vec<Segment>) {
        self.segments.extend(segments);
    }

    /// Get all segments
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// Get visible segments based on overflow behavior
    pub fn visible_segments(&self) -> Vec<&Segment> {
        match self.overflow_behavior {
            OverflowBehavior::ShowAll => self.segments.iter().collect(),
            OverflowBehavior::Truncate => {
                if self.segments.len() <= self.max_visible {
                    self.segments.iter().collect()
                } else {
                    let mut visible = vec![self.segments.first().unwrap()];
                    // Add ellipsis (conceptually - just track indices)
                    visible.extend(self.segments[self.segments.len() - 2..].iter());
                    visible
                }
            }
            OverflowBehavior::Collapse => {
                if let Some(last) = self.segments.last() {
                    vec![last]
                } else {
                    Vec::new()
                }
            }
        }
    }

    /// Get hidden segments (for overflow menu)
    pub fn hidden_segments(&self) -> Vec<&Segment> {
        match self.overflow_behavior {
            OverflowBehavior::Truncate => {
                if self.segments.len() > self.max_visible {
                    self.segments[1..self.segments.len() - 2].iter().collect()
                } else {
                    Vec::new()
                }
            }
            OverflowBehavior::Collapse => {
                if self.segments.len() > 1 {
                    self.segments[..self.segments.len() - 1].iter().collect()
                } else {
                    Vec::new()
                }
            }
            OverflowBehavior::ShowAll => Vec::new(),
        }
    }

    /// Find segment by ID
    pub fn find_segment(&self, id: &str) -> std::option::Option<&Segment> {
        self.segments.iter().find(|s| s.id == id)
    }

    /// Get segment at index
    pub fn segment_at(&self, index: usize) -> std::option::Option<&Segment> {
        self.segments.get(index)
    }

    /// Get current segment (last/active)
    pub fn current_segment(&self) -> std::option::Option<&Segment> {
        self.segments
            .iter()
            .find(|s| s.is_active())
            .or_else(|| self.segments.last())
    }

    /// Get root segment (first)
    pub fn root_segment(&self) -> std::option::Option<&Segment> {
        self.segments.first()
    }

    /// Get parent segment
    pub fn parent_segment(&self) -> std::option::Option<&Segment> {
        if self.segments.len() > 1 {
            Some(&self.segments[self.segments.len() - 2])
        } else {
            None
        }
    }

    /// Get breadcrumb path string
    pub fn path_string(&self) -> String {
        self.segments
            .iter()
            .map(|s| s.label.clone())
            .collect::<Vec<_>>()
            .join(&format!(" {} ", self.separator))
    }

    /// Get full file system path
    pub fn full_path(&self) -> String {
        if self.segments.is_empty() {
            "/".to_string()
        } else {
            self.segments.last().unwrap().path.clone()
        }
    }

    /// Get segment count
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Get visible segment count
    pub fn visible_count(&self) -> usize {
        self.visible_segments().len()
    }

    /// Has overflow?
    pub fn has_overflow(&self) -> bool {
        !self.hidden_segments().is_empty()
    }

    /// Get overflow behavior
    pub fn overflow_behavior(&self) -> OverflowBehavior {
        self.overflow_behavior
    }

    /// Get separator
    pub fn separator(&self) -> &str {
        &self.separator
    }

    /// Get max visible
    pub fn max_visible(&self) -> usize {
        self.max_visible
    }

    /// Get CSS class
    pub fn css_class(&self) -> String {
        let overflow_str = match self.overflow_behavior {
            OverflowBehavior::ShowAll => "show-all",
            OverflowBehavior::Truncate => "truncate",
            OverflowBehavior::Collapse => "collapse",
        };
        format!("aurora-breadcrumb aurora-breadcrumb-{}", overflow_str)
    }

    /// Clear all segments
    pub fn clear(&mut self) {
        self.segments.clear();
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Breadcrumb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Breadcrumb")
            .field("segment_count", &self.segment_count())
            .field("overflow_behavior", &self.overflow_behavior)
            .field("has_overflow", &self.has_overflow())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_creation() {
        let breadcrumb = Breadcrumb::new();
        assert_eq!(breadcrumb.segment_count(), 0);
        assert_eq!(breadcrumb.overflow_behavior(), OverflowBehavior::ShowAll);
    }

    #[test]
    fn test_add_segment() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.add_segment(Segment::new("home", "Home", "/home"));
        breadcrumb.add_segment(Segment::new("docs", "Documents", "/home/Documents"));
        assert_eq!(breadcrumb.segment_count(), 2);
    }

    #[test]
    fn test_add_segments() {
        let mut breadcrumb = Breadcrumb::new();
        let segments = vec![
            Segment::new("home", "Home", "/home"),
            Segment::new("docs", "Documents", "/home/Documents"),
            Segment::new("file", "file.txt", "/home/Documents/file.txt"),
        ];
        breadcrumb.add_segments(segments);
        assert_eq!(breadcrumb.segment_count(), 3);
    }

    #[test]
    fn test_find_segment() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.add_segment(Segment::new("docs", "Documents", "/home/Documents"));

        assert!(breadcrumb.find_segment("docs").is_some());
        assert_eq!(
            breadcrumb.find_segment("docs").unwrap().label(),
            "Documents"
        );
        assert!(breadcrumb.find_segment("unknown").is_none());
    }

    #[test]
    fn test_current_segment() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.add_segment(Segment::new("home", "Home", "/home"));
        breadcrumb.add_segment(Segment::new("docs", "Documents", "/home/Documents").active());

        let current = breadcrumb.current_segment();
        assert!(current.is_some());
        assert_eq!(current.unwrap().label(), "Documents");
    }

    #[test]
    fn test_root_segment() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.add_segment(Segment::new("home", "Home", "/home"));
        breadcrumb.add_segment(Segment::new("docs", "Documents", "/home/Documents"));

        let root = breadcrumb.root_segment();
        assert!(root.is_some());
        assert_eq!(root.unwrap().label(), "Home");
    }

    #[test]
    fn test_parent_segment() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.add_segment(Segment::new("home", "Home", "/home"));
        breadcrumb.add_segment(Segment::new("docs", "Documents", "/home/Documents"));
        breadcrumb.add_segment(Segment::new("file", "file.txt", "/home/Documents/file.txt"));

        let parent = breadcrumb.parent_segment();
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().label(), "Documents");
    }

    #[test]
    fn test_path_string() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.add_segment(Segment::new("home", "Home", "/home"));
        breadcrumb.add_segment(Segment::new("docs", "Documents", "/home/Documents"));

        assert_eq!(breadcrumb.path_string(), "Home / Documents");
    }

    #[test]
    fn test_full_path() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.add_segment(Segment::new("home", "Home", "/home"));
        breadcrumb.add_segment(Segment::new("docs", "Documents", "/home/Documents"));
        breadcrumb.add_segment(Segment::new("file", "file.txt", "/home/Documents/file.txt"));

        assert_eq!(breadcrumb.full_path(), "/home/Documents/file.txt");
    }

    #[test]
    fn test_separator() {
        let breadcrumb = Breadcrumb::new().with_separator(">");
        assert_eq!(breadcrumb.separator(), ">");

        let mut breadcrumb2 = Breadcrumb::new().with_separator("•");
        breadcrumb2.add_segment(Segment::new("home", "Home", "/home"));
        breadcrumb2.add_segment(Segment::new("docs", "Documents", "/home/Documents"));

        assert_eq!(breadcrumb2.path_string(), "Home • Documents");
    }

    #[test]
    fn test_overflow_show_all() {
        let mut breadcrumb = Breadcrumb::new().with_overflow(OverflowBehavior::ShowAll);

        for i in 0..5 {
            breadcrumb.add_segment(Segment::new(
                &i.to_string(),
                &format!("Dir {}", i),
                &format!("/{}", i),
            ));
        }

        assert_eq!(breadcrumb.visible_count(), 5);
        assert!(!breadcrumb.has_overflow());
    }

    #[test]
    fn test_overflow_truncate() {
        let mut breadcrumb = Breadcrumb::new()
            .with_overflow(OverflowBehavior::Truncate)
            .with_max_visible(3);

        for i in 0..5 {
            breadcrumb.add_segment(Segment::new(
                &i.to_string(),
                &format!("Dir {}", i),
                &format!("/{}", i),
            ));
        }

        assert!(breadcrumb.has_overflow());
        assert_eq!(breadcrumb.visible_count(), 3); // first, ellipsis, last
        assert_eq!(breadcrumb.hidden_segments().len(), 2);
    }

    #[test]
    fn test_overflow_collapse() {
        let mut breadcrumb = Breadcrumb::new().with_overflow(OverflowBehavior::Collapse);

        for i in 0..3 {
            breadcrumb.add_segment(Segment::new(
                &i.to_string(),
                &format!("Dir {}", i),
                &format!("/{}", i),
            ));
        }

        assert!(breadcrumb.has_overflow());
        assert_eq!(breadcrumb.visible_count(), 1);
        assert_eq!(breadcrumb.hidden_segments().len(), 2);
    }

    #[test]
    fn test_segment_at() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.add_segment(Segment::new("home", "Home", "/home"));
        breadcrumb.add_segment(Segment::new("docs", "Documents", "/home/Documents"));

        assert!(breadcrumb.segment_at(0).is_some());
        assert_eq!(breadcrumb.segment_at(0).unwrap().label(), "Home");
        assert!(breadcrumb.segment_at(1).is_some());
        assert_eq!(breadcrumb.segment_at(1).unwrap().label(), "Documents");
        assert!(breadcrumb.segment_at(2).is_none());
    }

    #[test]
    fn test_css_class() {
        let show_all = Breadcrumb::new().with_overflow(OverflowBehavior::ShowAll);
        assert!(show_all.css_class().contains("show-all"));

        let truncate = Breadcrumb::new().with_overflow(OverflowBehavior::Truncate);
        assert!(truncate.css_class().contains("truncate"));

        let collapse = Breadcrumb::new().with_overflow(OverflowBehavior::Collapse);
        assert!(collapse.css_class().contains("collapse"));
    }

    #[test]
    fn test_clear() {
        let mut breadcrumb = Breadcrumb::new();
        breadcrumb.add_segment(Segment::new("home", "Home", "/home"));
        breadcrumb.add_segment(Segment::new("docs", "Documents", "/home/Documents"));

        assert_eq!(breadcrumb.segment_count(), 2);
        breadcrumb.clear();
        assert_eq!(breadcrumb.segment_count(), 0);
    }

    #[test]
    fn test_default() {
        let breadcrumb = Breadcrumb::default();
        assert_eq!(breadcrumb.segment_count(), 0);
        assert_eq!(breadcrumb.overflow_behavior(), OverflowBehavior::ShowAll);
    }

    #[test]
    fn test_active_segment() {
        let segment = Segment::new("docs", "Documents", "/home/Documents").active();
        assert!(segment.is_active());
    }

    #[test]
    fn test_max_visible() {
        let breadcrumb = Breadcrumb::new().with_max_visible(5);
        assert_eq!(breadcrumb.max_visible(), 5);
    }
}
