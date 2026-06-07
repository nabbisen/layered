//! Split-section dialog: the user enters a title for the new child section
//! to be appended at the end of the focused body (RFC-025).

use dioxus::prelude::*;
use layerd_ui::i18n::{Locale, t};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SplitChoice {
    /// Insert the new heading with the given title.
    Confirm(String),
    Cancel,
}

#[component]
pub fn SplitDialog(locale: Signal<Locale>, on_choice: EventHandler<SplitChoice>) -> Element {
    let lang = *locale.read();
    let mut title = use_signal(String::new);

    rsx! {
        div {
            class: "modal-overlay",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": "split-title-label",
            div { class: "modal",
                h2 { id: "split-title-label", {t(lang, "dialog.split.title")} }
                input {
                    class: "search-input",
                    r#type: "text",
                    placeholder: t(lang, "dialog.split.placeholder"),
                    autofocus: true,
                    value: "{title}",
                    oninput: move |evt| title.set(evt.value()),
                    onkeydown: move |evt| {
                        use keyboard_types::Code;
                        match evt.data().code() {
                            Code::Enter if !title.read().trim().is_empty() => {
                                let t = title.read().trim().to_string();
                                on_choice.call(SplitChoice::Confirm(t));
                            }
                            Code::Escape => on_choice.call(SplitChoice::Cancel),
                            _ => {}
                        }
                    },
                }
                div { class: "modal-actions",
                    button {
                        class: "primary",
                        disabled: title.read().trim().is_empty(),
                        onclick: {
                            move |_| {
                                let t = title.read().trim().to_string();
                                if !t.is_empty() {
                                    on_choice.call(SplitChoice::Confirm(t));
                                }
                            }
                        },
                        {t(lang, "dialog.split.confirm")}
                    }
                    button {
                        onclick: move |_| on_choice.call(SplitChoice::Cancel),
                        {t(lang, "dialog.split.cancel")}
                    }
                }
            }
        }
    }
}
