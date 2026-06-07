//! Status bar at the bottom of the application shell (RFC-010, RFC-018, RFC-029).
//!
//! RFC-029 live-region policy:
//! - Informational messages (saved, ready, unsaved): `aria-live="polite"`.
//! - Error messages (save failed, stale node, …): `aria-live="assertive"` so
//!   screen readers interrupt to announce the failure immediately.
//! - Save-failure status includes an inline "Save As" recovery affordance.
//! - Dirty state is shown as text (not colour-only) per RFC-030.

use dioxus::prelude::*;
use layerd_ui::EditorSession;
use layerd_ui::i18n::{Locale, t};

/// Keys whose values are errors that warrant assertive announcement.
fn is_error_key(key: &str) -> bool {
    key.starts_with("error.")
}

#[component]
pub fn StatusBar(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    status: Signal<String>,
    /// Callback so the status bar can trigger Save As for error recovery.
    on_save_as: EventHandler<()>,
) -> Element {
    let lang = *locale.read();
    let dirty = session.read().is_dirty();
    let file = session.read().file_name().unwrap_or_default().to_string();
    let raw = session.read().is_raw();
    let newline_label = session.read().profile().newline.label().to_string();
    let key = status.read().clone();
    let status_msg = t(lang, key.as_str()).to_string();
    let is_error = is_error_key(&key);
    let is_save_error = key == "error.save_failed";

    rsx! {
        footer { class: "statusbar",
            // Polite region for informational updates; assertive for errors.
            if is_error {
                span {
                    "aria-live": "assertive",
                    "aria-atomic": "true",
                    class: "status-error",
                    {status_msg}
                    // RFC-029: save failure includes a recovery affordance.
                    if is_save_error {
                        " "
                        button {
                            class: "status-action",
                            onclick: move |_| on_save_as.call(()),
                            {t(lang, "menu.file.save_as")}
                        }
                    }
                }
            } else {
                span { "aria-live": "polite", "aria-atomic": "true", {status_msg} }
            }
            if dirty {
                span { class: "dirty", {t(lang, "status.unsaved")} }
            }
            if raw {
                span { class: "raw-badge", {t(lang, "raw.title")} }
            }
            span { class: "newline-label", "{newline_label}" }
            if !file.is_empty() {
                span { class: "file", "{file}" }
            }
        }
    }
}
