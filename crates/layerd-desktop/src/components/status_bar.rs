//! Status bar at the bottom of the application shell (RFC-010, RFC-018, RFC-029).
//!
//! Shows: status message (polite live region) · dirty marker · line ending
//! policy (LF / CRLF / Mixed from RFC-018 FileTextProfile) · file path.

use dioxus::prelude::*;
use layerd_ui::EditorSession;
use layerd_ui::i18n::{Locale, t};

#[component]
pub fn StatusBar(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    status: Signal<String>,
) -> Element {
    let lang = *locale.read();
    let dirty = session.read().is_dirty();
    let file = session.read().file_name().unwrap_or_default().to_string();
    let raw = session.read().is_raw();
    let newline_label = session.read().profile().newline.label().to_string();
    let status_msg = {
        let key = status.read();
        t(lang, key.as_str()).to_string()
    };

    rsx! {
        footer { class: "statusbar",
            span { "aria-live": "polite", "aria-atomic": "true", {status_msg} }
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
