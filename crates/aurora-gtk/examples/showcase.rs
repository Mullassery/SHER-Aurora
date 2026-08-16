//! Aurora GTK4 Component Showcase
//!
//! A real, runnable GTK4 application that opens an actual window and packs
//! it with Aurora's currently real-GTK4-backed widgets: Button, Input,
//! Checkbox, Switch, and Card. This replaces an earlier version of this
//! file that imported a nonexistent `gtk` crate (this project's real
//! dependency is `gtk4`) and never compiled.
//!
//! Only the widgets that currently have a real `build()` method (see
//! `aurora_gtk::widgets`) are shown here; the rest of the widget library
//! (DataTable, Tabs, Menu, etc.) is still a logic-only descriptor layer and
//! is intentionally left out of this showcase rather than faked.
//!
//! Run it with:
//!
//! ```sh
//! cargo run --example showcase -p aurora-gtk
//! ```
//!
//! This opens a real, interactive GTK4 window (requires a display/window
//! session). For a non-interactive, assertion-driven proof that these
//! widgets are real (suitable for headless verification), see
//! `examples/gtk4_harness.rs`.

use aurora_gtk::widgets::{
    Button, ButtonStyle, Card, CardStyle, Checkbox, Input, InputType, Switch,
};
use aurora_gtk::{CssProvider, Theme};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Orientation};

const APP_ID: &str = "org.aurora.Showcase";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    // `run` hands control to GTK's real main loop. Pass an empty argv so it
    // doesn't try to parse this process's own command-line arguments.
    app.run_with_args::<&str>(&[]);
}

fn build_ui(app: &Application) {
    // Install Aurora's real, token-derived CSS onto the real display this
    // window will appear on.
    if let Some(display) = gtk4::gdk::Display::default() {
        let css = CssProvider::new(Theme::Light).expect("failed to build Aurora CSS provider");
        css.install(&display);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Aurora GTK4 Showcase")
        .default_width(420)
        .default_height(480)
        .build();

    let root = gtk4::Box::new(Orientation::Vertical, 16);
    root.set_margin_top(24);
    root.set_margin_bottom(24);
    root.set_margin_start(24);
    root.set_margin_end(24);

    let heading = gtk4::Label::new(Some("Aurora components, rendered for real"));
    heading.add_css_class("title-2");
    root.append(&heading);

    // Card container holding the rest of the components — a real gtk4::Box
    // built from Aurora's Card descriptor.
    let card = Card::new()
        .with_style(CardStyle::Elevated)
        .with_spacing(12)
        .build();

    card.append(
        &Button::new("Primary action")
            .with_style(ButtonStyle::Filled)
            .build(),
    );
    card.append(
        &Button::new("Secondary action")
            .with_style(ButtonStyle::Outlined)
            .build(),
    );
    card.append(
        &Input::new(InputType::Email)
            .with_placeholder("you@example.com")
            .build(),
    );
    card.append(&Checkbox::new("Remember me").checked(true).build());
    card.append(&Switch::new().active(false).build());

    root.append(&card);
    window.set_child(Some(&root));
    window.present();
}
