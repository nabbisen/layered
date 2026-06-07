//! Status bar (RFC-010, RFC-018, RFC-029, RFC-046).
//!
//! Shows: status message (polite/assertive live region) · dirty marker ·
//! document statistics (RFC-046: word count, section count) ·
//! line-ending style · file path.

use dioxus::prelude::*;
use layered_ui::EditorSession;
use layered_ui::i18n::{Locale, t};

fn is_error_key(key: &str) -> bool {
    key.starts_with("error.")
}

#[component]
pub fn StatusBar(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    status: Signal<String>,
    on_save_as: EventHandler<()>,
) -> Element {
    let lang = *locale.read();
    let dirty = session.read().is_dirty();
    let file = session.read().file_name().unwrap_or_default().to_string();
    let raw = session.read().is_raw();
    let newline_label = session.read().profile().newline.label().to_string();
    let has_doc = !session.read().source().is_empty() || dirty;

    // RFC-046: document statistics.
    let stats = session.read().stats();

    let key = status.read().clone();
    let status_msg = t(lang, key.as_str()).to_string();
    let is_error = is_error_key(&key);
    let is_save_error = key == "error.save_failed";

    rsx! {
        footer { class: "statusbar",
            if is_error {
                span {
                    "aria-live": "assertive",
                    "aria-atomic": "true",
                    class: "status-error",
                    {status_msg}
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
            // RFC-046: word/section stats (only when a document is open).
            if has_doc && (stats.total_words > 0 || stats.section_count > 0) {
                span { class: "stats",
                    if stats.focused_words > 0 {
                        "{stats.focused_words}\u{00a0}{t(lang, \"stats.words\")} / "
                    }
                    "{stats.total_words}\u{00a0}{t(lang, \"stats.words\")} · "
                    "{stats.section_count}\u{00a0}{t(lang, \"stats.sections\")}"
                }
            }
            span { class: "newline-label", "{newline_label}" }
            if !file.is_empty() {
                span { class: "file", "{file}" }
            }
        }
    }
}
