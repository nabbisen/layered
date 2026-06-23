//! Read-only raw Markdown source view (RFC-017): transparent inspection of the
//! canonical text and recovery path when outline projection is incomplete.
//!
//! M3 decision: the view is intentionally read-only. Editable raw source
//! will follow once RFC-008 and RFC-016 are fully mature.

use dioxus::prelude::*;
use omriss_ui::EditorSession;
use omriss_ui::i18n::{Locale, t};

#[component]
pub fn RawSourceView(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    on_back: EventHandler<()>,
) -> Element {
    let lang = *locale.read();
    // Snapshot the source once to avoid repeated borrows inside rsx!
    let source = session.read().source().to_string();
    let line_count = source.lines().count();

    rsx! {
        main { class: "main-pane raw-source",
            div { class: "raw-toolbar",
                h2 { {t(lang, "raw.title")} }
                span { class: "raw-meta hint-text",
                    "{line_count} lines"
                }
                button {
                    class: "raw-back",
                    onclick: move |_| on_back.call(()),
                    {t(lang, "raw.back")}
                }
            }
            p {
                class: "raw-notice hint-text",
                "aria-live": "polite",
                {t(lang, "raw.readonly_notice")}
            }
            pre {
                class: "raw-pre",
                // role="region" + label so screen readers can browse the source.
                role: "region",
                "aria-label": t(lang, "raw.title"),
                code { class: "raw-code", {source} }
            }
        }
    }
}
