//! Application toolbar: file actions, navigation controls, dirty marker, and
//! locale switcher (RFC-010 shell, RFC-014 keyboard contract).

use dioxus::prelude::*;
use layerd_ui::EditorSession;
use layerd_ui::i18n::{Locale, t};

#[component]
pub fn Toolbar(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    draft: Signal<String>,
    status: Signal<String>,
    on_open: EventHandler<()>,
    on_save: EventHandler<()>,
    on_save_as: EventHandler<()>,
) -> Element {
    let lang = *locale.read();
    let dirty = session.read().is_dirty();
    let can_undo = session.read().can_undo();
    let can_redo = session.read().can_redo();
    let can_back = session.read().can_go_back();
    let can_forward = session.read().can_go_forward();
    let file = session
        .read()
        .file_name()
        .map(|n| format!(" — {n}"))
        .unwrap_or_default();

    let undo = move |_| {
        if session.write().undo().is_ok() {
            let body = session
                .read()
                .current_snapshot()
                .map(|s| s.body)
                .unwrap_or_default();
            draft.set(body);
        }
    };
    let redo = move |_| {
        if session.write().redo().is_ok() {
            let body = session
                .read()
                .current_snapshot()
                .map(|s| s.body)
                .unwrap_or_default();
            draft.set(body);
        }
    };
    let back = move |_| {
        session.write().back();
        let body = session
            .read()
            .current_snapshot()
            .map(|s| s.body)
            .unwrap_or_default();
        draft.set(body);
    };
    let forward = move |_| {
        session.write().forward();
        let body = session
            .read()
            .current_snapshot()
            .map(|s| s.body)
            .unwrap_or_default();
        draft.set(body);
    };

    rsx! {
        div { class: "toolbar",
            button { onclick: move |_| on_open.call(()), {t(lang, "menu.file.open")} }
            button { onclick: move |_| on_save.call(()), {t(lang, "menu.file.save")} }
            button { onclick: move |_| on_save_as.call(()), {t(lang, "menu.file.save_as")} }
            div { class: "toolbar-sep" }
            button { disabled: !can_undo, onclick: undo, {t(lang, "toolbar.undo")} }
            button { disabled: !can_redo, onclick: redo, {t(lang, "toolbar.redo")} }
            div { class: "toolbar-sep" }
            button { disabled: !can_back, onclick: back, {t(lang, "nav.back")} }
            button { disabled: !can_forward, onclick: forward, {t(lang, "nav.forward")} }
            div { class: "spacer" }
            if dirty {
                span { class: "dirty-indicator", "●" }
            }
            span { class: "file-label", "{file}" }
            select {
                "aria-label": t(lang, "menu.language"),
                onchange: move |event| {
                    if let Some(picked) = Locale::from_tag(&event.value()) {
                        locale.set(picked);
                    }
                },
                for entry in Locale::ALL {
                    option {
                        value: entry.tag(),
                        selected: *entry == lang,
                        {entry.native_name()}
                    }
                }
            }
        }
    }
}
