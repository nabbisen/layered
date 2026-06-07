//! Welcome screen rendered when no document is open (RFC-010 §4 no-document
//! shell state). Primary action buttons mirror the toolbar shortcuts so
//! keyboard-only users have an obvious starting point.

use dioxus::prelude::*;
use layerd_ui::i18n::{Locale, t};

#[component]
pub fn WelcomeScreen(
    locale: Signal<Locale>,
    on_open: EventHandler<()>,
    on_new: EventHandler<()>,
) -> Element {
    let lang = *locale.read();
    rsx! {
        main { class: "main-pane welcome",
            div { class: "welcome-inner",
                h1 { class: "welcome-title", {t(lang, "app.title")} }
                p { class: "welcome-hint", {t(lang, "welcome.hint")} }
                div { class: "welcome-actions",
                    button {
                        class: "primary",
                        autofocus: true,
                        onclick: move |_| on_open.call(()),
                        {t(lang, "menu.file.open")}
                    }
                    button {
                        onclick: move |_| on_new.call(()),
                        {t(lang, "menu.file.new")}
                    }
                }
                p { class: "welcome-shortcut", "Ctrl+O  ·  Ctrl+N" }
            }
        }
    }
}
