//! Desktop entry point: configures the window, detects the startup locale,
//! and launches the root component. All editor logic lives in `layerd-ui`;
//! this crate is only the platform shell (RFC-001, RFC-010, RFC-035).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod file_dialog;

use dioxus::desktop::{Config, WindowBuilder};
use layerd_ui::i18n::Locale;

/// Detects the OS locale at startup (RFC-043 layer table: detection belongs
/// to the desktop crate). Environment-variable based for now; an explicit
/// user setting will take precedence once RFC-036 settings storage lands.
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
