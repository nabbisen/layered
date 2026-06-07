//! Desktop entry point: detects the startup locale, configures the window,
//! and launches the root component. All editor logic lives in `layerd-ui`;
//! this crate is the platform shell (RFC-001, RFC-010, RFC-035).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod components;
mod file_dialog;
mod keyboard;

use dioxus::desktop::{Config, WindowBuilder};
use layerd_ui::i18n::Locale;

/// Detects the OS locale at startup from environment variables (RFC-043
/// layer table: detection belongs to the desktop crate). The explicit user
/// preference from settings will take precedence once RFC-036 lands.
fn detect_locale() -> Locale {
    ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .filter_map(|var| std::env::var(var).ok())
        .find_map(|tag| Locale::from_tag(&tag))
        .unwrap_or_default()
}

fn main() {
    let window = WindowBuilder::new()
        .with_title("layerd")
        .with_inner_size(dioxus::desktop::LogicalSize::new(1080.0, 720.0));
    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window).with_menu(None))
        .with_context(detect_locale())
        .launch(app::App);
}
