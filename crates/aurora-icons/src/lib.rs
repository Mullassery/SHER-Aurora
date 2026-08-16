//! aurora-icons — Aurora's real SVG icon set
//!
//! # Honest scope
//!
//! This crate ships **24 real, hand-authored, renderable SVG icons** across
//! five categories (navigation, actions, status, media, system). Earlier
//! project documentation claimed "2000+ organized system and application
//! icons" — that claim was aspirational and, at the time it was written,
//! not backed by a single actual SVG asset anywhere in the repository.
//! This crate corrects that: every icon returned by [`icon_svg`] is a
//! complete, valid, 24x24 `viewBox` SVG document with real path/shape
//! geometry — not a placeholder, not lorem-ipsum, not a stub.
//!
//! Growing this set toward a much larger, comprehensive icon library
//! (including application/brand icons) is real, legitimate future work —
//! it just isn't claimed as already-done here.
//!
//! # Design
//!
//! All icons share a consistent visual language: 24x24 grid, 1.5px stroke,
//! round line caps/joins, `currentColor` for stroke (and, where a small
//! filled accent is used, fill) so consumers can recolor icons via CSS/GTK
//! styling without regenerating markup.
//!
//! # Example
//!
//! ```
//! use aurora_icons::{icon_svg, IconId};
//!
//! let svg = icon_svg(IconId::Home).unwrap();
//! assert!(svg.starts_with("<svg"));
//! assert!(svg.contains("viewBox"));
//! ```

use std::fmt;

/// Identifiers for every icon this crate actually ships real artwork for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconId {
    // Navigation
    ArrowLeft,
    ArrowRight,
    Home,
    Menu,
    Search,
    // Actions
    Save,
    Delete,
    Edit,
    Copy,
    Refresh,
    // Status
    Check,
    CloseCircle,
    Alert,
    Info,
    Clock,
    // Media
    Play,
    Pause,
    Volume,
    Image,
    // System
    Settings,
    User,
    Wifi,
    Battery,
    Power,
}

/// Icon category, matching the organization scheme used across Aurora.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconCategory {
    Navigation,
    Actions,
    Status,
    Media,
    System,
}

impl fmt::Display for IconCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            IconCategory::Navigation => "navigation",
            IconCategory::Actions => "actions",
            IconCategory::Status => "status",
            IconCategory::Media => "media",
            IconCategory::System => "system",
        };
        write!(f, "{name}")
    }
}

impl IconId {
    /// Every icon this crate ships real artwork for.
    pub const ALL: [IconId; 24] = [
        IconId::ArrowLeft,
        IconId::ArrowRight,
        IconId::Home,
        IconId::Menu,
        IconId::Search,
        IconId::Save,
        IconId::Delete,
        IconId::Edit,
        IconId::Copy,
        IconId::Refresh,
        IconId::Check,
        IconId::CloseCircle,
        IconId::Alert,
        IconId::Info,
        IconId::Clock,
        IconId::Play,
        IconId::Pause,
        IconId::Volume,
        IconId::Image,
        IconId::Settings,
        IconId::User,
        IconId::Wifi,
        IconId::Battery,
        IconId::Power,
    ];

    /// Stable, kebab-case string id (matches common icon-naming convention).
    pub fn slug(&self) -> &'static str {
        match self {
            IconId::ArrowLeft => "arrow-left",
            IconId::ArrowRight => "arrow-right",
            IconId::Home => "home",
            IconId::Menu => "menu",
            IconId::Search => "search",
            IconId::Save => "save",
            IconId::Delete => "delete",
            IconId::Edit => "edit",
            IconId::Copy => "copy",
            IconId::Refresh => "refresh",
            IconId::Check => "check",
            IconId::CloseCircle => "close-circle",
            IconId::Alert => "alert",
            IconId::Info => "info",
            IconId::Clock => "clock",
            IconId::Play => "play",
            IconId::Pause => "pause",
            IconId::Volume => "volume",
            IconId::Image => "image",
            IconId::Settings => "settings",
            IconId::User => "user",
            IconId::Wifi => "wifi",
            IconId::Battery => "battery",
            IconId::Power => "power",
        }
    }

    /// Category this icon belongs to.
    pub fn category(&self) -> IconCategory {
        match self {
            IconId::ArrowLeft
            | IconId::ArrowRight
            | IconId::Home
            | IconId::Menu
            | IconId::Search => IconCategory::Navigation,
            IconId::Save | IconId::Delete | IconId::Edit | IconId::Copy | IconId::Refresh => {
                IconCategory::Actions
            }
            IconId::Check | IconId::CloseCircle | IconId::Alert | IconId::Info | IconId::Clock => {
                IconCategory::Status
            }
            IconId::Play | IconId::Pause | IconId::Volume | IconId::Image => IconCategory::Media,
            IconId::Settings | IconId::User | IconId::Wifi | IconId::Battery | IconId::Power => {
                IconCategory::System
            }
        }
    }

    /// Look up an icon by its slug (e.g. `"arrow-left"`).
    pub fn from_slug(slug: &str) -> Option<IconId> {
        IconId::ALL.into_iter().find(|id| id.slug() == slug)
    }
}

/// Shared SVG open/close wrapper: 24x24 grid, stroke-based, `currentColor`.
fn wrap(body: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" \
         stroke=\"currentColor\" stroke-width=\"1.5\" stroke-linecap=\"round\" \
         stroke-linejoin=\"round\">{body}</svg>"
    )
}

/// Real SVG body geometry (everything inside the `<svg>...</svg>` wrapper)
/// for each icon. This is the actual hand-authored artwork.
fn icon_body(id: IconId) -> &'static str {
    match id {
        IconId::ArrowLeft => r#"<path d="M19 12H5M12 19l-7-7 7-7"/>"#,
        IconId::ArrowRight => r#"<path d="M5 12h14M12 5l7 7-7 7"/>"#,
        IconId::Home => {
            r#"<path d="M3 11.5 12 4l9 7.5"/><path d="M5 10v10h14V10"/><path d="M9 20v-6h6v6"/>"#
        }
        IconId::Menu => r#"<path d="M4 7h16M4 12h16M4 17h16"/>"#,
        IconId::Search => r#"<circle cx="11" cy="11" r="7"/><path d="M21 21l-4.35-4.35"/>"#,
        IconId::Save => {
            r#"<path d="M5 4h11l3 3v13H5z"/><path d="M8 4v6h8V4"/><path d="M8 14h8v6H8z"/>"#
        }
        IconId::Delete => {
            r#"<path d="M4 7h16"/><path d="M9 7V4h6v3"/><path d="M6 7l1 13h10l1-13"/>"#
        }
        IconId::Edit => r#"<path d="M4 20l4-1 11-11-3-3L5 16z"/><path d="M14 5l3 3"/>"#,
        IconId::Copy => {
            r#"<rect x="9" y="9" width="11" height="11" rx="1"/><path d="M5 15V5a1 1 0 011-1h10"/>"#
        }
        IconId::Refresh => {
            r#"<path d="M4 12a8 8 0 0 1 14-5.3M20 4v5h-5"/><path d="M20 12a8 8 0 0 1-14 5.3M4 20v-5h5"/>"#
        }
        IconId::Check => r#"<path d="M5 13l4 4 10-10"/>"#,
        IconId::CloseCircle => r#"<circle cx="12" cy="12" r="9"/><path d="M9 9l6 6M15 9l-6 6"/>"#,
        IconId::Alert => r#"<path d="M12 3 22 20H2z"/><path d="M12 9v5"/><path d="M12 17h.01"/>"#,
        IconId::Info => {
            r#"<circle cx="12" cy="12" r="9"/><path d="M12 8h.01"/><path d="M11 12h1v5h1"/>"#
        }
        IconId::Clock => r#"<circle cx="12" cy="12" r="9"/><path d="M12 7v5l4 2"/>"#,
        IconId::Play => r#"<path d="M7 4l13 8-13 8z"/>"#,
        IconId::Pause => {
            r#"<rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/>"#
        }
        IconId::Volume => r#"<path d="M4 9v6h4l6 5V4l-6 5z"/><path d="M17 9a5 5 0 0 1 0 6"/>"#,
        IconId::Image => {
            r#"<rect x="3" y="4" width="18" height="16" rx="2"/><circle cx="8.5" cy="9.5" r="1.5"/><path d="M21 16l-5-5-4 4-3-3-6 6"/>"#
        }
        IconId::Settings => {
            r#"<circle cx="12" cy="12" r="3"/><path d="M12 2v3M12 19v3M4.2 4.2l2.1 2.1M17.7 17.7l2.1 2.1M2 12h3M19 12h3M4.2 19.8l2.1-2.1M17.7 6.3l2.1-2.1"/>"#
        }
        IconId::User => {
            r#"<circle cx="12" cy="8" r="4"/><path d="M4 21c0-4.4 3.6-8 8-8s8 3.6 8 8"/>"#
        }
        IconId::Wifi => {
            r#"<path d="M2 8.5a16 16 0 0 1 20 0"/><path d="M5.5 12.5a11 11 0 0 1 13 0"/><path d="M9 16.3a6 6 0 0 1 6 0"/><circle cx="12" cy="20" r="1" fill="currentColor" stroke="none"/>"#
        }
        IconId::Battery => {
            r#"<rect x="2" y="7" width="18" height="10" rx="2"/><path d="M22 10v4"/><rect x="4" y="9" width="10" height="6" fill="currentColor" stroke="none"/>"#
        }
        IconId::Power => r#"<path d="M12 3v9"/><path d="M6.3 6.3a9 9 0 1 0 11.4 0"/>"#,
    }
}

/// Get the real, complete SVG document for an icon (wrapped with a
/// consistent 24x24 `<svg>` shell via [`wrap`]).
///
/// Every `IconId` variant has real artwork (this function never returns
/// `None` for a value produced by this crate's own `IconId::ALL`); the
/// signature returns `Option<String>` so callers resolving icons dynamically
/// from user-provided ids (e.g. via [`IconId::from_slug`]) have a natural
/// "not found" path instead of needing to panic.
pub fn icon_svg(id: IconId) -> Option<String> {
    Some(wrap(icon_body(id)))
}

/// Total number of icons this crate actually ships (source of truth for
/// documentation — never hand-write this number in a README/doc comment,
/// derive it from here or from `IconId::ALL.len()`).
pub const ICON_COUNT: usize = IconId::ALL.len();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_id_has_real_svg() {
        for id in IconId::ALL {
            let svg = icon_svg(id).unwrap_or_else(|| panic!("missing SVG for {:?}", id));
            let svg = svg.as_str();
            assert!(svg.starts_with("<svg"), "{:?} did not start with <svg", id);
            assert!(svg.ends_with("</svg>"), "{:?} did not end with </svg>", id);
            assert!(
                svg.contains("viewBox=\"0 0 24 24\""),
                "{:?} missing viewBox",
                id
            );
            // Every icon must have real drawable geometry, not an empty shell.
            assert!(
                svg.contains("<path") || svg.contains("<circle") || svg.contains("<rect"),
                "{:?} has no drawable elements",
                id
            );
        }
    }

    #[test]
    fn icon_count_is_accurate_not_inflated() {
        // This is the actual, real number of hand-authored icons. If this
        // grows, ICON_COUNT (and any docs referencing it) grow with it —
        // there is no separate "target" number pretending to be current.
        assert_eq!(ICON_COUNT, 24);
        assert_eq!(IconId::ALL.len(), ICON_COUNT);
    }

    #[test]
    fn slugs_are_unique() {
        let mut slugs: Vec<&str> = IconId::ALL.iter().map(|i| i.slug()).collect();
        slugs.sort_unstable();
        let mut deduped = slugs.clone();
        deduped.dedup();
        assert_eq!(slugs.len(), deduped.len(), "duplicate icon slugs found");
    }

    #[test]
    fn from_slug_round_trips() {
        for id in IconId::ALL {
            let found = IconId::from_slug(id.slug()).unwrap();
            assert_eq!(found, id);
        }
        assert!(IconId::from_slug("does-not-exist").is_none());
    }

    #[test]
    fn categories_cover_every_icon() {
        use std::collections::HashMap;
        let mut counts: HashMap<String, usize> = HashMap::new();
        for id in IconId::ALL {
            *counts.entry(id.category().to_string()).or_insert(0) += 1;
        }
        assert_eq!(counts.get("navigation"), Some(&5));
        assert_eq!(counts.get("actions"), Some(&5));
        assert_eq!(counts.get("status"), Some(&5));
        assert_eq!(counts.get("media"), Some(&4));
        assert_eq!(counts.get("system"), Some(&5));
    }

    #[test]
    fn wrap_helper_produces_valid_shell() {
        let s = wrap("<path d=\"M0 0\"/>");
        assert!(s.starts_with("<svg"));
        assert!(s.contains("<path d=\"M0 0\"/>"));
        assert!(s.ends_with("</svg>"));
    }

    #[test]
    fn category_display() {
        assert_eq!(IconCategory::Navigation.to_string(), "navigation");
        assert_eq!(IconCategory::System.to_string(), "system");
    }
}
