//! Root component of the desktop GUI: an MVP subset of the application
//! shell (RFC-010) and focus editor (RFC-012) with breadcrumb navigation
//! (RFC-013) and the localized string catalog (RFC-043).
//!
//! One `EditorSession` signal is the single source of truth; the focused
//! section's draft body lives in its own signal and is committed explicitly
//! through the session, which enforces the core revision check (RFC-008).

use dioxus::prelude::*;
use layerd_ui::i18n::{Locale, t};
use layerd_ui::{EditorSession, ViewMode};

use crate::file_dialog::{self, OpenOutcome, SaveOutcome};

const STYLE: &str = include_str!("../assets/style.css");

#[component]
pub fn App() -> Element {
    // Startup locale: detected by main.rs (RFC-043 layer table) and provided
    // through context; defaults to English when no context is supplied.
    let initial_locale = try_consume_context::<Locale>().unwrap_or_default();
    let session = use_signal(EditorSession::new_empty);
    let locale = use_signal(move || initial_locale);
    let draft = use_signal(String::new);
    let status = use_signal(|| "status.ready".to_string());

    rsx! {
        style { {STYLE} }
        div { class: "app",
            Toolbar { session, locale, draft, status }
            div { class: "body",
                OutlinePane { session, locale, draft }
                MainPane { session, locale, draft, status }
            }
            StatusBar { session, locale, status }
        }
    }
}

/// Loads the focused section's body into the draft editor.
fn sync_draft(session: &EditorSession, mut draft: Signal<String>) {
    let body = session
        .current_snapshot()
        .map(|snapshot| snapshot.body)
        .unwrap_or_default();
    draft.set(body);
}

#[component]
fn Toolbar(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    draft: Signal<String>,
    status: Signal<String>,
) -> Element {
    let lang = *locale.read();
    let open = move |_| match file_dialog::open_markdown() {
        OpenOutcome::Cancelled => {}
        OpenOutcome::Failed => status.set("error.open_failed".into()),
        OpenOutcome::Loaded { text, name } => match EditorSession::open(text, Some(name)) {
            Ok(opened) => {
                session.set(opened);
                sync_draft(&session.read(), draft);
                status.set("status.ready".into());
            }
            Err(_) => status.set("error.open_failed".into()),
        },
    };
    let save = move |_| {
        // The canonical text is written back verbatim (RFC-002).
        let outcome = {
            let current = session.read();
            file_dialog::save_markdown(current.file_name(), current.source())
        };
        match outcome {
            SaveOutcome::Cancelled => {}
            SaveOutcome::Failed => status.set("error.save_failed".into()),
            SaveOutcome::Saved { name } => {
                session.write().mark_saved(Some(name));
                status.set("status.saved".into());
            }
        }
    };
    let undo = move |_| {
        if session.write().undo().is_ok() {
            sync_draft(&session.read(), draft);
        }
    };
    let redo = move |_| {
        if session.write().redo().is_ok() {
            sync_draft(&session.read(), draft);
        }
    };
    let back = move |_| {
        session.write().back();
        sync_draft(&session.read(), draft);
    };
    let forward = move |_| {
        session.write().forward();
        sync_draft(&session.read(), draft);
    };

    rsx! {
        div { class: "toolbar",
            button { onclick: open, {t(lang, "menu.file.open")} }
            button { onclick: save, {t(lang, "menu.file.save")} }
            button {
                disabled: !session.read().can_undo(),
                onclick: undo,
                {t(lang, "toolbar.undo")}
            }
            button {
                disabled: !session.read().can_redo(),
                onclick: redo,
                {t(lang, "toolbar.redo")}
            }
            button {
                disabled: !session.read().can_go_back(),
                onclick: back,
                {t(lang, "nav.back")}
            }
            button {
                disabled: !session.read().can_go_forward(),
                onclick: forward,
                {t(lang, "nav.forward")}
            }
            div { class: "spacer" }
            select {
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

#[component]
fn OutlinePane(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    draft: Signal<String>,
) -> Element {
    let lang = *locale.read();
    let items = session.read().outline_items();
    rsx! {
        nav { class: "outline-pane",
            h2 { {t(lang, "outline.title")} }
            if items.is_empty() {
                p { class: "outline-empty", {t(lang, "outline.empty")} }
            }
            for item in items {
                button {
                    class: "outline-item",
                    key: "{item.id.0}",
                    onclick: move |_| {
                        let _ = session.write().focus(item.id);
                        sync_draft(&session.read(), draft);
                    },
                    if item.title.is_empty() {
                        {t(lang, "breadcrumb.root")}
                    } else {
                        "{item.title}"
                    }
                    if item.child_count > 0 {
                        span { class: "count", "({item.child_count})" }
                    }
                }
            }
        }
    }
}

#[component]
fn MainPane(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    draft: Signal<String>,
    status: Signal<String>,
) -> Element {
    let lang = *locale.read();
    let mode = session.read().view_mode();
    match mode {
        ViewMode::Outline => rsx! {
            main { class: "main-pane",
                p { class: "outline-empty", {t(lang, "outline.empty")} }
            }
        },
        ViewMode::Focus(_) => {
            let Some(snapshot) = session.read().current_snapshot() else {
                return rsx! {
                    main { class: "main-pane" }
                };
            };
            let commit_base = snapshot.clone();
            let commit = move |_| {
                let outcome = session
                    .write()
                    .commit_focused_body(&commit_base, draft.read().clone());
                match outcome {
                    Ok(_) => status.set("status.unsaved".into()),
                    Err(_) => status.set("error.stale_edit".into()),
                }
                sync_draft(&session.read(), draft);
            };
            rsx! {
                main { class: "main-pane",
                    div { class: "breadcrumb",
                        for (index, crumb) in snapshot.path.iter().enumerate() {
                            if index > 0 {
                                span { class: "sep", "›" }
                            }
                            if index + 1 == snapshot.path.len() {
                                span { class: "here",
                                    if crumb.title.is_empty() {
                                        {t(lang, "breadcrumb.root")}
                                    } else {
                                        "{crumb.title}"
                                    }
                                }
                            } else {
                                button {
                                    onclick: {
                                        let id = crumb.id;
                                        move |_| {
                                            let _ = session.write().focus(id);
                                            sync_draft(&session.read(), draft);
                                        }
                                    },
                                    if crumb.title.is_empty() {
                                        {t(lang, "breadcrumb.root")}
                                    } else {
                                        "{crumb.title}"
                                    }
                                }
                            }
                        }
                    }
                    h1 { class: "focus-title",
                        if snapshot.title.is_empty() {
                            {t(lang, "breadcrumb.root")}
                        } else {
                            "{snapshot.title}"
                        }
                        if let Some(level) = snapshot.level {
                            span { class: "level", "H{level.as_u8()}" }
                        }
                    }
                    textarea {
                        class: "body-editor",
                        placeholder: t(lang, "editor.body.placeholder"),
                        value: "{draft}",
                        oninput: move |event| draft.set(event.value()),
                    }
                    div { class: "editor-actions",
                        button { class: "primary", onclick: commit, {t(lang, "toolbar.edit")} }
                    }
                    if !snapshot.children.is_empty() {
                        section { class: "children",
                            h3 { {t(lang, "focus.children")} }
                            for child in snapshot.children.clone() {
                                button {
                                    class: "child-card",
                                    key: "{child.id.0}",
                                    onclick: move |_| {
                                        let _ = session.write().focus(child.id);
                                        sync_draft(&session.read(), draft);
                                    },
                                    "{child.title}"
                                    if child.child_count > 0 {
                                        span { class: "count", "({child.child_count})" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatusBar(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    status: Signal<String>,
) -> Element {
    let lang = *locale.read();
    let dirty = session.read().is_dirty();
    let file = session.read().file_name().unwrap_or_default().to_string();
    let status_key = status.read().clone();
    rsx! {
        footer { class: "statusbar",
            span { {t(lang, status_key.as_str()).to_string()} }
            if dirty {
                span { class: "dirty", {t(lang, "status.unsaved")} }
            }
            span { class: "file", "{file}" }
        }
    }
}
