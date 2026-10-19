/// Aurora Tooltip descriptor.
///
/// GTK tooltips are attached to a widget rather than being a widget themselves,
/// so this type provides a real `attach_to()` that calls the GTK4 API instead
/// of the old no-op string functions.
#[derive(Debug, Clone)]
pub struct Tooltip {
    text: String,
}

impl Tooltip {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// Attach this tooltip to a real GTK4 widget.
    pub fn attach_to<W: gtk4::prelude::IsA<gtk4::Widget>>(&self, widget: &W) {
        use gtk4::prelude::WidgetExt;
        widget.set_tooltip_text(Some(&self.text));
    }
}

/// Set a tooltip on a component by id is retained for API compatibility but
/// cannot work without a widget reference; use [`Tooltip::attach_to`].
pub fn set_tooltip(_component_id: &str, _text: &str) -> &'static str {
    "use Tooltip::attach_to(widget) to set a real GTK4 tooltip"
}

/// Remove a tooltip; use `WidgetExt::set_tooltip_text(None)` with a widget.
pub fn remove_tooltip(_component_id: &str) -> &'static str {
    "use WidgetExt::set_tooltip_text(None) to remove a real GTK4 tooltip"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_keeps_text() {
        assert_eq!(Tooltip::new("Save").text(), "Save");
    }

    #[cfg(not(target_os = "macos"))]
    #[gtk4::test]
    fn tooltip_attaches_to_real_widget() {
        use gtk4::prelude::*;
        let button = gtk4::Button::with_label("Save");
        Tooltip::new("Save changes").attach_to(&button);
        assert_eq!(button.tooltip_text().as_deref(), Some("Save changes"));
    }
}
