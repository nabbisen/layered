//! Left-panel outline: heading cards with keyboard selection (RFC-011,
//! RFC-014). Arrow keys move the selection; Enter zooms into the selected
//! item; clicking any item zooms in immediately.
//!
//! The currently selected card index lives as a `Signal<usize>` in `App` so
//! that the global keyboard handler (root div `onkeydown`) can drive it from
//! outside this component.

use dioxus::prelude::*;
use layerd_ui::i18n::{Locale, t};
use layerd_ui::{EditorSession, ViewMode};

#[component]
pub fn OutlinePane(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    draft: Signal<String>,
    /// Zero-based index of the keyboard-selected card.
    selected_card: Signal<usize>,
) -> Element {
    let lang = *locale.read();
    let items = session.read().current_children();
    let sel = *selected_card.read();

    rsx! {
        nav {
            class: "outline-pane",
            "aria-label": t(lang, "aria.outline"),
            h2 { {t(lang, "outline.title")} }

            if items.is_empty() {
                p { class: "outline-empty", {t(lang, "outline.empty")} }
                p { class: "outline-hint", {t(lang, "outline.no-headings.hint")} }
            } else {
                // RFC-011 accessibility: list of navigable items exposed with
                // role="listbox" / role="option" semantics.
                div { role: "listbox", "aria-label": t(lang, "aria.outline"),
                    for (idx , item) in items.iter().enumerate() {
                        {
                            let id = item.id;
                            let count = item.child_count;
                            let title = item.title.clone();
                            let is_selected = idx == sel;
                            rsx! {
                                div {
                                    key: "{id.0}",
                                    role: "option",
                                    "aria-selected": if is_selected { "true" } else { "false" },
                                    tabindex: if is_selected { "0" } else { "-1" },
                                    class: if is_selected { "outline-item outline-item--selected" } else { "outline-item" },
                                    onclick: move |_| {
                                        selected_card.set(idx);
                                        let _ = session.write().focus(id);
                                        let body = session.read().current_snapshot()
                                            .map(|s| s.body).unwrap_or_default();
                                        draft.set(body);
                                    },
                                    onkeydown: move |event| {
                                        use keyboard_types::Code;
                                        match event.data().code() {
                                            Code::Enter | Code::Space => {
                                                selected_card.set(idx);
                                                let _ = session.write().focus(id);
                                                let body = session.read().current_snapshot()
                                                    .map(|s| s.body).unwrap_or_default();
                                                draft.set(body);
                                            }
                                            _ => {}
                                        }
                                    },
                                    if title.is_empty() {
                                        em { {t(lang, "breadcrumb.root")} }
                                    } else {
                                        "{title}"
                                    }
                                    if count > 0 {
                                        span { class: "count", " ({count})" }
                                    }
                                }
                            }
                        }
                    }
                }
                // Keyboard hint for sighted users (hidden from screen readers
                // because it is supplementary to the ARIA roles above).
                p { class: "outline-hint", "aria-hidden": "true",
                    {t(lang, "keyboard.overview.hint")}
                }
            }

            if let ViewMode::Focus(_) = session.read().view_mode() {
                button {
                    class: "outline-up",
                    onclick: move |_| {
                        // Commit draft before navigating away.
                        let snap = session.read().current_snapshot();
                        if let Some(snapshot) = snap {
                            let current_draft = draft.read().clone();
                            if current_draft != snapshot.body {
                                let _ = session.write()
                                    .commit_focused_body(&snapshot, current_draft);
                            }
                        }
                        session.write().zoom_out();
                        let body = session.read().current_snapshot()
                            .map(|s| s.body).unwrap_or_default();
                        draft.set(body);
                    },
                    {t(lang, "nav.up")}
                }
            }
        }
    }
}
