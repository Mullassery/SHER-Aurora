//! Core Icon Set — metadata for Aurora's real icons
//!
//! Registers metadata (category, tags, aliases) for exactly the icons that
//! `aurora-icons` ships real SVG artwork for. This intentionally does not
//! pad the library out to a round number with placeholder entries — an
//! earlier version of this file registered ~200 "stub" icons named things
//! like `nav-23` and `act-41` with no real geometry behind them, purely to
//! make `total_count()` look larger. Every icon registered here resolves to
//! real SVG via `aurora_icons::IconId::from_slug` + `aurora_icons::icon_svg`.

use crate::icons::{IconCategory, IconContext, IconLibrary, IconMetadata};

/// Initialize the icon library with metadata for every real icon Aurora
/// currently ships (24, see `aurora_icons::ICON_COUNT`).
pub fn load_core_icons() -> IconLibrary {
    let mut library = IconLibrary::new();

    load_navigation_icons(&mut library);
    load_actions_icons(&mut library);
    load_status_icons(&mut library);
    load_media_icons(&mut library);
    load_system_icons(&mut library);

    library
}

/// Navigation icons: back, forward, home, menu, search
fn load_navigation_icons(library: &mut IconLibrary) {
    library.register(
        IconMetadata::new("arrow-left", "Back", IconCategory::Navigation)
            .with_tag("navigation")
            .with_tag("back")
            .with_alias("chevron-left"),
    );
    library.register(
        IconMetadata::new("arrow-right", "Forward", IconCategory::Navigation)
            .with_tag("navigation")
            .with_tag("forward")
            .with_alias("chevron-right"),
    );
    library.register(
        IconMetadata::new("home", "Home", IconCategory::Navigation)
            .with_tag("navigation")
            .with_tag("house"),
    );
    library.register(
        IconMetadata::new("menu", "Menu", IconCategory::Navigation)
            .with_tag("navigation")
            .with_tag("hamburger")
            .with_alias("list"),
    );
    library.register(
        IconMetadata::new("search", "Search", IconCategory::Navigation)
            .with_tag("navigation")
            .with_tag("find")
            .with_alias("magnifying-glass"),
    );
}

/// Action icons: save, delete, edit, copy, refresh
fn load_actions_icons(library: &mut IconLibrary) {
    library.register(
        IconMetadata::new("save", "Save", IconCategory::Actions)
            .with_tag("action")
            .with_tag("file")
            .with_context(IconContext::Success),
    );
    library.register(
        IconMetadata::new("delete", "Delete", IconCategory::Actions)
            .with_tag("action")
            .with_tag("remove")
            .with_context(IconContext::Error),
    );
    library.register(
        IconMetadata::new("edit", "Edit", IconCategory::Actions)
            .with_tag("action")
            .with_tag("pencil"),
    );
    library.register(
        IconMetadata::new("copy", "Copy", IconCategory::Actions)
            .with_tag("action")
            .with_tag("duplicate"),
    );
    library.register(
        IconMetadata::new("refresh", "Refresh", IconCategory::Actions)
            .with_tag("action")
            .with_tag("reload")
            .with_tag("sync"),
    );
}

/// Status icons: success, error, warning, info, loading
fn load_status_icons(library: &mut IconLibrary) {
    library.register(
        IconMetadata::new("check", "Success", IconCategory::Status)
            .with_tag("status")
            .with_tag("success")
            .with_context(IconContext::Success),
    );
    library.register(
        IconMetadata::new("close-circle", "Error", IconCategory::Status)
            .with_tag("status")
            .with_tag("error")
            .with_context(IconContext::Error),
    );
    library.register(
        IconMetadata::new("alert", "Warning", IconCategory::Status)
            .with_tag("status")
            .with_tag("warning")
            .with_context(IconContext::Warning),
    );
    library.register(
        IconMetadata::new("info", "Information", IconCategory::Status)
            .with_tag("status")
            .with_tag("help")
            .with_context(IconContext::Info),
    );
    library.register(
        IconMetadata::new("clock", "Clock", IconCategory::Status)
            .with_tag("status")
            .with_tag("time"),
    );
}

/// Media icons: play, pause, volume, image
fn load_media_icons(library: &mut IconLibrary) {
    library.register(
        IconMetadata::new("play", "Play", IconCategory::Media)
            .with_tag("media")
            .with_tag("video"),
    );
    library.register(
        IconMetadata::new("pause", "Pause", IconCategory::Media)
            .with_tag("media")
            .with_tag("video"),
    );
    library.register(
        IconMetadata::new("volume", "Volume", IconCategory::Media)
            .with_tag("media")
            .with_tag("audio")
            .with_tag("speaker"),
    );
    library.register(
        IconMetadata::new("image", "Image", IconCategory::Media)
            .with_tag("media")
            .with_tag("photo")
            .with_tag("picture"),
    );
}

/// System icons: settings, user, wifi, battery, power
fn load_system_icons(library: &mut IconLibrary) {
    library.register(
        IconMetadata::new("settings", "Settings", IconCategory::System)
            .with_tag("system")
            .with_tag("config")
            .with_tag("gear"),
    );
    library.register(
        IconMetadata::new("user", "User", IconCategory::System)
            .with_tag("system")
            .with_tag("person")
            .with_tag("profile"),
    );
    library.register(
        IconMetadata::new("wifi", "Wi-Fi", IconCategory::System)
            .with_tag("system")
            .with_tag("network")
            .with_tag("internet"),
    );
    library.register(
        IconMetadata::new("battery", "Battery", IconCategory::System)
            .with_tag("system")
            .with_tag("power"),
    );
    library.register(
        IconMetadata::new("power", "Power", IconCategory::System)
            .with_tag("system")
            .with_tag("off"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_core_icons() {
        let library = load_core_icons();
        // Matches aurora_icons::ICON_COUNT exactly — every metadata entry
        // here corresponds to one real SVG icon, no padding.
        assert_eq!(library.total_count(), aurora_icons::ICON_COUNT);
    }

    #[test]
    fn test_every_registered_icon_resolves_to_real_svg() {
        let library = load_core_icons();
        for icon in library.all() {
            let id = aurora_icons::IconId::from_slug(icon.id())
                .unwrap_or_else(|| panic!("{} has no matching real aurora-icons entry", icon.id()));
            let svg = aurora_icons::icon_svg(id).unwrap();
            assert!(svg.starts_with("<svg"));
        }
    }

    #[test]
    fn test_core_icons_by_category() {
        let library = load_core_icons();
        assert_eq!(library.by_category(IconCategory::Navigation).len(), 5);
        assert_eq!(library.by_category(IconCategory::Actions).len(), 5);
        assert_eq!(library.by_category(IconCategory::Status).len(), 5);
        assert_eq!(library.by_category(IconCategory::Media).len(), 4);
        assert_eq!(library.by_category(IconCategory::System).len(), 5);
    }

    #[test]
    fn test_core_icons_search() {
        let library = load_core_icons();
        let results = library.search("save");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_core_icons_by_tag() {
        let library = load_core_icons();
        let action_icons = library.by_tag("action");
        assert_eq!(action_icons.len(), 5);
    }
}
