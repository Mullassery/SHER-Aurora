//! Reactive OS theme-preference listener.
//!
//! `libadwaita` is not a dependency of this crate, so this listens to the
//! real GTK `Settings` properties that back the same preference:
//! `gtk-application-prefer-dark-theme` and `gtk-theme-name` (for GNOME's
//! `HighContrast` themes). On either notify signal the *existing*
//! `gtk4::CssProvider` is reloaded from Aurora's color tokens; widgets are
//! never rebuilt or re-initialized.
//!
//! The Light/Dark selection itself lives in [`ThemePreference`] and is pure,
//! so it is unit-tested without a display.

use gtk4::prelude::*;

use crate::css::CssProvider;
use crate::errors::{GtkError, GtkResult};
use crate::theme::{Theme, ThemePreference};

/// Keeps an installed CSS provider in sync with the OS theme preference.
///
/// Dropping the listener disconnects its `Settings` signal handlers; the CSS
/// provider stays installed on the display (remove it yourself if needed).
pub struct ThemeListener {
    settings: gtk4::Settings,
    provider: gtk4::CssProvider,
    handlers: Vec<glib::SignalHandlerId>,
}

impl ThemeListener {
    /// Installs a listener using the default `gtk4::Settings` (requires GTK
    /// to have been initialized, e.g. from `Application::connect_activate`).
    pub fn install(display: &gtk4::gdk::Display) -> GtkResult<Self> {
        let settings = gtk4::Settings::default().ok_or_else(|| {
            GtkError::InitializationError(
                "no default GTK settings; initialize GTK before installing the theme listener"
                    .to_string(),
            )
        })?;
        Self::install_with_settings(&settings, display)
    }

    /// Installs a listener against an explicit `Settings` object (the same
    /// real notify signals, useful when the caller already holds it).
    pub fn install_with_settings(
        settings: &gtk4::Settings,
        display: &gtk4::gdk::Display,
    ) -> GtkResult<Self> {
        let provider = gtk4::CssProvider::new();
        apply_preference(settings, &provider)?;
        gtk4::style_context_add_provider_for_display(
            display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let dark_provider = provider.clone();
        let dark_handler =
            settings.connect_gtk_application_prefer_dark_theme_notify(move |settings| {
                apply_preference_logged(settings, &dark_provider)
            });

        let theme_provider = provider.clone();
        let theme_handler = settings.connect_gtk_theme_name_notify(move |settings| {
            apply_preference_logged(settings, &theme_provider)
        });

        Ok(Self {
            settings: settings.clone(),
            provider,
            handlers: vec![dark_handler, theme_handler],
        })
    }

    /// The provider installed on the display and updated in place.
    pub fn provider(&self) -> &gtk4::CssProvider {
        &self.provider
    }

    /// The palette currently selected from the live GTK settings.
    pub fn current_theme(&self) -> Theme {
        preference_from_settings(&self.settings).theme()
    }
}

impl Drop for ThemeListener {
    fn drop(&mut self) {
        for handler in self.handlers.drain(..) {
            self.settings.disconnect(handler);
        }
    }
}

fn preference_from_settings(settings: &gtk4::Settings) -> ThemePreference {
    ThemePreference::from_gtk_settings(
        settings.is_gtk_application_prefer_dark_theme(),
        settings.gtk_theme_name().as_deref(),
    )
}

fn apply_preference(settings: &gtk4::Settings, provider: &gtk4::CssProvider) -> GtkResult<()> {
    let preference = preference_from_settings(settings);
    CssProvider::new(preference.theme())?.load_into(provider);
    Ok(())
}

fn apply_preference_logged(settings: &gtk4::Settings, provider: &gtk4::CssProvider) {
    if let Err(error) = apply_preference(settings, provider) {
        tracing::warn!("failed to regenerate Aurora CSS after theme change: {error}");
    }
}
