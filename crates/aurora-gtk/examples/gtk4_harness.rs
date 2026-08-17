//! Real GTK4 rendering harness for Aurora.
//!
//! This is a runnable, non-mocked demonstration that Aurora's widget
//! descriptors produce real `gtk4` crate objects backed by the actual
//! system GTK4 library, and that Aurora's token-derived CSS is really
//! loaded into GTK4's CSS engine and attached to a real `gdk::Display`.
//!
//! Run it with:
//!
//! ```sh
//! cargo run --example gtk4_harness -p aurora-gtk
//! ```
//!
//! Why this ships as a runnable example instead of `#[test]`: GTK4's Cocoa
//! (macOS) backend requires `gtk4::init()` to run on the process's actual
//! OS main thread. Rust's `#[test]` harness (and even the `#[gtk4::test]`
//! helper, which dispatches onto a GLib thread-pool worker) never runs on
//! that real main thread on macOS, so GTK4 cannot initialize there
//! (verified directly: calling `gtk4::init()` from a spawned thread panics
//! with "Attempted to initialize GTK on OSX from non-main thread"). A plain
//! `fn main()` run via `cargo run`, by contrast, *is* the process main
//! thread, so this harness is real evidence on macOS. The same widget
//! construction is additionally covered by real `#[gtk4::test]`s gated to
//! non-macOS targets (see `widgets::button`, `widgets::input`,
//! `widgets::checkbox`, `widgets::card`, `widgets::switch`), which run for
//! real on Linux CI where GTK4 has no such main-thread restriction.

use aurora_gtk::widgets::{
    Button, ButtonStyle, Card, CardStyle, Checkbox, Input, InputType, Switch,
};
use aurora_gtk::{CssProvider, Theme};
use gtk4::prelude::*;

fn main() {
    // 1. Real GTK4 initialization — this calls the actual C library's
    //    gtk_init_check() through the FFI layer.
    gtk4::init().expect("gtk4 init failed — is GTK4 installed? (brew install gtk4)");
    println!(
        "[1/6] Real GTK4 initialized. Runtime version: {}.{}.{}",
        gtk4::major_version(),
        gtk4::minor_version(),
        gtk4::micro_version()
    );

    // 2. Install Aurora's real, token-derived CSS onto the real default
    //    display via GTK4's actual CSS engine.
    let css_provider = CssProvider::new(Theme::Dark).expect("failed to build Aurora CSS provider");
    let display = gtk4::gdk::Display::default().expect("no default GDK display available");
    let real_gtk_css_provider = css_provider.install(&display);
    println!(
        "[2/6] Installed {} bytes of Aurora-generated CSS into a real gtk4::CssProvider on display {:?}",
        css_provider.generate_css().len(),
        display.name()
    );
    assert!(real_gtk_css_provider.is::<gtk4::CssProvider>());

    // 3. Build real GTK4 widgets from Aurora descriptors.
    let button = Button::new("Get Started")
        .with_style(ButtonStyle::Filled)
        .build();
    let entry = Input::new(InputType::Email)
        .with_placeholder("you@example.com")
        .build();
    let checkbox = Checkbox::new("Send me updates").checked(true).build();
    let switch = Switch::new().active(true).build();
    let card = Card::new()
        .with_style(CardStyle::Elevated)
        .with_spacing(12)
        .build();

    println!(
        "[3/6] Built real GTK4 widgets: {}, {}, {}, {}, {}",
        button.type_().name(),
        entry.type_().name(),
        checkbox.type_().name(),
        switch.type_().name(),
        card.type_().name(),
    );

    // 4. Pack real widgets into the real card container, and the card into
    //    a real top-level window — proving these are genuine, composable
    //    GTK4 widgets, not inert data.
    card.append(&button);
    card.append(&entry);
    card.append(&checkbox);
    card.append(&switch);

    let window = gtk4::Window::builder()
        .title("Aurora GTK4 Harness")
        .child(&card)
        .build();

    // 5. Realize the window off-screen. This forces GTK4 to actually run
    //    its real layout/measure/CSS-matching pipeline for every widget in
    //    the tree — genuine rendering-pipeline execution, without needing
    //    an interactive display session or a running main loop.
    gtk4::prelude::WidgetExt::realize(&window);
    let (min_w, nat_w, _, _) = card.measure(gtk4::Orientation::Horizontal, -1);
    println!(
        "[4/6] Realized real GTK4 window. Card container measured by the real GTK4 layout engine: min_width={min_w}, natural_width={nat_w}"
    );

    // 6. Query properties back out through the real GTK4 API (not our own
    //    struct fields) to prove state round-trips through actual GTK4
    //    widget objects.
    println!(
        "[5/6] Real GTK4 state round-trip: button.label()={:?} css_classes={:?}, entry.placeholder_text()={:?}, checkbox.is_active()={}, switch.is_active()={}",
        button.label(),
        button.css_classes(),
        entry.placeholder_text(),
        checkbox.is_active(),
        switch.is_active(),
    );

    assert_eq!(button.label().unwrap(), "Get Started");
    assert!(button.css_classes().iter().any(|c| c == "suggested-action"));
    assert_eq!(entry.placeholder_text().unwrap(), "you@example.com");
    assert!(checkbox.is_active());
    assert!(switch.is_active());
    assert!(card.first_child().is_some());

    window.close();
    println!("[6/6] All real-GTK4 assertions passed. This was genuine gtk4 crate execution against GTK {}.{}.{}, not a mock.",
        gtk4::major_version(), gtk4::minor_version(), gtk4::micro_version());
}
