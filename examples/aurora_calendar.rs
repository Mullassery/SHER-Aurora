//! Aurora Calendar - Event Management Example
//!
//! Demonstrates Aurora components in a calendar application.

/// Calendar event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    id: u32,
    title: String,
    date: String,
    time: String,
    description: String,
}

impl Event {
    /// Create a new event
    pub fn new(id: u32, title: &str, date: &str, time: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            date: date.to_string(),
            time: time.to_string(),
            description: String::new(),
        }
    }

    /// Get event ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get event title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get event date
    pub fn date(&self) -> &str {
        &self.date
    }

    /// Get event time
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Set event description
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    /// Get event description
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Aurora calendar application
pub struct AuroraCalendar {
    title: String,
    events: Vec<Event>,
    current_month: u32,
    current_year: u32,
}

impl AuroraCalendar {
    /// Create a new calendar
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            events: Vec::new(),
            current_month: 1,
            current_year: 2027,
        }
    }

    /// Get calendar title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Add event
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Get events
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Get event by ID
    pub fn get_event(&self, id: u32) -> Option<&Event> {
        self.events.iter().find(|e| e.id == id)
    }

    /// Delete event
    pub fn delete_event(&mut self, id: u32) -> bool {
        if let Some(pos) = self.events.iter().position(|e| e.id == id) {
            self.events.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get current month
    pub fn current_month(&self) -> u32 {
        self.current_month
    }

    /// Get current year
    pub fn current_year(&self) -> u32 {
        self.current_year
    }

    /// Move to next month
    pub fn next_month(&mut self) {
        self.current_month += 1;
        if self.current_month > 12 {
            self.current_month = 1;
            self.current_year += 1;
        }
    }

    /// Move to previous month
    pub fn prev_month(&mut self) {
        if self.current_month > 1 {
            self.current_month -= 1;
        } else {
            self.current_month = 12;
            self.current_year -= 1;
        }
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

impl Default for AuroraCalendar {
    fn default() -> Self {
        Self::new("Aurora Calendar")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_creation() {
        let calendar = AuroraCalendar::new("My Calendar");
        assert_eq!(calendar.title(), "My Calendar");
        assert_eq!(calendar.event_count(), 0);
    }

    #[test]
    fn test_add_event() {
        let mut calendar = AuroraCalendar::new("Calendar");
        let event = Event::new(1, "Meeting", "2027-01-15", "10:00");
        calendar.add_event(event);
        assert_eq!(calendar.event_count(), 1);
    }

    #[test]
    fn test_get_event() {
        let mut calendar = AuroraCalendar::new("Calendar");
        let event = Event::new(1, "Meeting", "2027-01-15", "10:00");
        calendar.add_event(event);

        let found = calendar.get_event(1);
        assert!(found.is_some());
        assert_eq!(found.unwrap().title(), "Meeting");
    }

    #[test]
    fn test_delete_event() {
        let mut calendar = AuroraCalendar::new("Calendar");
        let event = Event::new(1, "Meeting", "2027-01-15", "10:00");
        calendar.add_event(event);

        assert!(calendar.delete_event(1));
        assert_eq!(calendar.event_count(), 0);
    }

    #[test]
    fn test_month_navigation() {
        let mut calendar = AuroraCalendar::new("Calendar");
        assert_eq!(calendar.current_month(), 1);

        calendar.next_month();
        assert_eq!(calendar.current_month(), 2);

        calendar.prev_month();
        assert_eq!(calendar.current_month(), 1);
    }

    #[test]
    fn test_year_wraparound() {
        let mut calendar = AuroraCalendar::new("Calendar");
        for _ in 0..12 {
            calendar.next_month();
        }
        assert_eq!(calendar.current_month(), 1);
        assert_eq!(calendar.current_year(), 2028);
    }

    #[test]
    fn test_event_description() {
        let mut event = Event::new(1, "Meeting", "2027-01-15", "10:00");
        event.set_description("Team standup");
        assert_eq!(event.description(), "Team standup");
    }

    #[test]
    fn test_default() {
        let calendar = AuroraCalendar::default();
        assert_eq!(calendar.title(), "Aurora Calendar");
    }
}
