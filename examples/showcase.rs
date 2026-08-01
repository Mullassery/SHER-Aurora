//! Aurora Component Showcase Application
//!
//! Demonstrates all 10 core Aurora components in action.
//! Build with: cargo build --example showcase
//! Run with: cargo run --example showcase

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Label, Orientation, ScrolledWindow};
use aurora_gtk::widgets::*;

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Aurora Component Showcase"));
    window.set_default_size(1000, 800);

    // Main container
    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.add_css_class("aurora-showcase");

    // Header
    let header = GtkBox::new(Orientation::Horizontal, 0);
    header.add_css_class("aurora-showcase-header");
    let title = Label::new(Some("Aurora GTK4 Components"));
    title.add_css_class("aurora-showcase-title");
    header.append(&title);
    main_box.append(&header);

    // Scrolled content
    let scrolled = ScrolledWindow::new();
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);

    let content = GtkBox::new(Orientation::Vertical, 16);
    content.set_margin_top(24);
    content.set_margin_bottom(24);
    content.set_margin_start(24);
    content.set_margin_end(24);

    // Button Component
    let button_section = create_section("Buttons");
    let button_box = GtkBox::new(Orientation::Horizontal, 8);

    let filled = Button::new("Filled").with_style(ButtonStyle::Filled);
    let tinted = Button::new("Tinted").with_style(ButtonStyle::Tinted);
    let outlined = Button::new("Outlined").with_style(ButtonStyle::Outlined);
    let ghost = Button::new("Ghost").with_style(ButtonStyle::Ghost);

    button_box.append(filled.inner());
    button_box.append(tinted.inner());
    button_box.append(outlined.inner());
    button_box.append(ghost.inner());

    content.append(&button_section);
    content.append(&button_box);

    // Card Component
    let card_section = create_section("Cards");
    let card_box = GtkBox::new(Orientation::Horizontal, 8);

    let filled_card = Card::new().with_style(CardStyle::Filled);
    let outlined_card = Card::new().with_style(CardStyle::Outlined);
    let elevated_card = Card::new().with_style(CardStyle::Elevated);

    let card_label1 = Label::new(Some("Filled Card"));
    filled_card.inner().append(&card_label1);

    let card_label2 = Label::new(Some("Outlined Card"));
    outlined_card.inner().append(&card_label2);

    let card_label3 = Label::new(Some("Elevated Card"));
    elevated_card.inner().append(&card_label3);

    card_box.append(filled_card.inner());
    card_box.append(outlined_card.inner());
    card_box.append(elevated_card.inner());

    content.append(&card_section);
    content.append(&card_box);

    // Input Component
    let input_section = create_section("Input Fields");
    let input_box = GtkBox::new(Orientation::Vertical, 8);

    let text_input = Input::new(InputType::Text)
        .with_placeholder("Enter text");
    let email_input = Input::new(InputType::Email)
        .with_placeholder("user@example.com");
    let password_input = Input::new(InputType::Password)
        .with_placeholder("Enter password");
    let search_input = Input::new(InputType::Search)
        .with_placeholder("Search...");

    input_box.append(text_input.inner());
    input_box.append(email_input.inner());
    input_box.append(password_input.inner());
    input_box.append(search_input.inner());

    content.append(&input_section);
    content.append(&input_box);

    // Checkbox Component
    let checkbox_section = create_section("Checkboxes");
    let checkbox_box = GtkBox::new(Orientation::Vertical, 8);

    let checkbox1 = Checkbox::new("Accept terms and conditions");
    let checkbox2 = Checkbox::new("Subscribe to newsletter").checked(true);
    let checkbox3 = Checkbox::new("Remember me").checked(true);

    checkbox_box.append(checkbox1.inner());
    checkbox_box.append(checkbox2.inner());
    checkbox_box.append(checkbox3.inner());

    content.append(&checkbox_section);
    content.append(&checkbox_box);

    // Radio Button Component
    let radio_section = create_section("Radio Buttons");
    let radio_box = GtkBox::new(Orientation::Vertical, 8);

    let radio1 = RadioButton::new("Option 1").selected(true);
    let radio2 = RadioButton::new("Option 2");
    let radio3 = RadioButton::new("Option 3");

    radio2.inner().set_group(Some(radio1.inner()));
    radio3.inner().set_group(Some(radio1.inner()));

    radio_box.append(radio1.inner());
    radio_box.append(radio2.inner());
    radio_box.append(radio3.inner());

    content.append(&radio_section);
    content.append(&radio_box);

    // Badge Component
    let badge_section = create_section("Badges");
    let badge_box = GtkBox::new(Orientation::Horizontal, 8);

    let badge1 = Badge::new("Default");
    let badge2 = Badge::new("Success").with_style(BadgeStyle::Success);
    let badge3 = Badge::new("Warning").with_style(BadgeStyle::Warning);
    let badge4 = Badge::new("Error").with_style(BadgeStyle::Error);
    let badge5 = Badge::new("Info").with_style(BadgeStyle::Info);

    badge_box.append(badge1.inner());
    badge_box.append(badge2.inner());
    badge_box.append(badge3.inner());
    badge_box.append(badge4.inner());
    badge_box.append(badge5.inner());

    content.append(&badge_section);
    content.append(&badge_box);

    // List Component
    let list_section = create_section("List");
    let list = List::new();

    content.append(&list_section);
    content.append(list.inner());

    // Sidebar Component
    let sidebar_section = create_section("Sidebar");
    let sidebar = Sidebar::new();

    content.append(&sidebar_section);
    content.append(sidebar.inner());

    scrolled.set_child(Some(&content));
    main_box.append(&scrolled);

    window.set_child(Some(&main_box));
    window.present();
}

/// Create a section header label
fn create_section(title: &str) -> Label {
    let label = Label::new(Some(title));
    label.add_css_class("aurora-showcase-section");
    label.set_halign(gtk::Align::Start);
    label
}

fn main() {
    let app = Application::builder()
        .application_id("org.gnome.aurora.showcase")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}
