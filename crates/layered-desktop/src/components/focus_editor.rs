//! Focus editor: edit the body of one section while seeing immediate children
//! (RFC-012). Keeps a local draft buffer; commits on blur, explicit save, and
//! Esc (zoom-out). Also hosts the Markdown preview pane toggle (RFC-045).

use dioxus::prelude::*;
use layered_ui::EditorSession;
use layered_ui::i18n::{Locale, t};

use super::{Breadcrumb, PreviewPane};

#[component]
pub fn FocusEditor(
    session: Signal<EditorSession>,
    locale: Signal<Locale>,
    draft: Signal<String>,
    status: Signal<String>,
    /// When true, the preview pane is shown instead of the textarea (RFC-045).
    preview_open: Signal<bool>,
) -> Element {
    let lang = *locale.read();
    let Some(snapshot) = session.read().current_snapshot() else {
        return rsx! { main { class: "main-pane" } };
    };

    // Local dirty: draft differs from the committed body.
    let local_dirty = *draft.read() != snapshot.body;

    // Structural toolbar is hidden by default; toggled by the ⋯ button.
    let mut show_struct = use_signal(|| false);

    let commit_base = snapshot.clone();
    let do_commit = move |_: Event<MouseData>| {
        let body = draft.read().clone();
        // Assign to a let so the write guard drops before the match arms
        // can call session.read() (RFC-012).
        let result = session.write().commit_focused_body(&commit_base, body);
        match result {
            Ok(_) => {
                status.set("status.unsaved".into());
                let refreshed = session
                    .read()
                    .current_snapshot()
                    .map(|s| s.body)
                    .unwrap_or_default();
                draft.set(refreshed);
            }
            Err(_) => {
                // Keep draft unchanged so the user can see and recover
                // their unsaved text (RFC-012 AC: local edit recoverable
                // after commit failure).
                status.set("error.stale_edit".into());
            }
        }
    };

    // Commit on textarea blur (RFC-012 commit lifecycle).
    let blur_base = snapshot.clone();
    let do_blur = move |_: Event<FocusData>| {
        let body = draft.read().clone();
        if body == blur_base.body {
            return;
        }
        let result = session.write().commit_focused_body(&blur_base, body);
        match result {
            Ok(_) => {
                let refreshed = session
                    .read()
                    .current_snapshot()
                    .map(|s| s.body)
                    .unwrap_or_default();
                draft.set(refreshed);
            }
            Err(_) => {
                // Leave draft in place on failure.
            }
        }
    };

    rsx! {
        main { class: "main-pane",
            Breadcrumb { session, locale, draft }

            // RFC-045: show preview pane instead of editor when toggled.
            if *preview_open.read() {
                PreviewPane {
                    session,
                    locale,
                    on_close: move |()| {
                        let mut po = preview_open;
                        po.set(false);
                    },
                }
            } else {
            h1 { class: "focus-title",
                if snapshot.title.is_empty() {
                    {t(lang, "breadcrumb.root")}
                } else {
                    "{snapshot.title}"
                }
                if let Some(level) = snapshot.level {
                    span { class: "level", "H{level.as_u8()}" }
                }
                if local_dirty {
                    span { class: "local-dirty", " \u{25cf}" }
                }
            }

            // ── sibling/depth navigation bar (RFC-020) ───────────────────
            {
                let info = session.read().sibling_info();
                let has_prev   = info.prev_sibling.is_some();
                let has_parent = info.parent.is_some();
                let has_child  = info.first_child.is_some();
                let has_next   = info.next_sibling.is_some();

                let commit_and_navigate = |nav: fn(&mut layered_ui::EditorSession) -> bool,
                                            mut session: Signal<layered_ui::EditorSession>,
                                            mut draft: Signal<String>| {
                    let snap = session.read().current_snapshot();
                    if let Some(s) = snap {
                        let d = draft.read().clone();
                        if d != s.body {
                            let _ = session.write().commit_focused_body(&s, d);
                        }
                    }
                    let navigated = nav(&mut session.write());
                    if navigated {
                        let body = session.read().current_snapshot()
                            .map(|s| s.body).unwrap_or_default();
                        draft.set(body);
                    }
                };

                rsx! {
                    div { class: "sibling-nav",
                        button {
                            class: "sibling-btn",
                            disabled: !has_prev,
                            title: t(lang, "nav.prev_sibling"),
                            onclick: move |_| commit_and_navigate(
                                layered_ui::EditorSession::navigate_prev_sibling,
                                session, draft
                            ),
                            "\u{2190} {t(lang, \"nav.prev_sibling\")}"
                        }
                        button {
                            class: "sibling-btn",
                            disabled: !has_parent,
                            title: t(lang, "nav.parent"),
                            onclick: move |_| commit_and_navigate(
                                layered_ui::EditorSession::navigate_parent,
                                session, draft
                            ),
                            "\u{2191} {t(lang, \"nav.parent\")}"
                        }
                        button {
                            class: "sibling-btn",
                            disabled: !has_child,
                            title: t(lang, "nav.first_child"),
                            onclick: move |_| commit_and_navigate(
                                layered_ui::EditorSession::navigate_first_child,
                                session, draft
                            ),
                            "\u{2193} {t(lang, \"nav.first_child\")}"
                        }
                        button {
                            class: "sibling-btn",
                            disabled: !has_next,
                            title: t(lang, "nav.next_sibling"),
                            onclick: move |_| commit_and_navigate(
                                layered_ui::EditorSession::navigate_next_sibling,
                                session, draft
                            ),
                            "{t(lang, \"nav.next_sibling\")} \u{2192}"
                        }
                    }
                }
            }

            textarea {
                class: "body-editor",
                "aria-label": t(lang, "aria.editor"),
                placeholder: t(lang, "editor.body.placeholder"),
                // RFC-028: move focus into the editor when the section is zoomed into.
                autofocus: true,
                value: "{draft}",
                oninput: move |event| draft.set(event.value()),
                onblur: do_blur,
            }
            div { class: "editor-actions",
                button {
                    class: "primary",
                    onclick: do_commit,
                    {t(lang, "toolbar.edit")}
                }
                // RFC-045: toggle between edit and preview.
                button {
                    class: if *preview_open.read() { "btn-preview active" } else { "btn-preview" },
                    title: t(lang, "editor.preview"),
                    onclick: move |_| {
                        let currently_open = *preview_open.read();
                        if !currently_open {
                            // Commit draft before entering preview so HTML reflects latest text.
                            let snap = session.read().current_snapshot();
                            if let Some(s) = snap {
                                let d = draft.read().clone();
                                if d != s.body {
                                    let _ = session.write().commit_focused_body(&s, d);
                                }
                            }
                        }
                        let mut po = preview_open;
                        po.set(!currently_open);
                    },
                    {t(lang, "editor.preview")}
                }
                if snapshot.body.is_empty() {
                    span { class: "hint-text", {t(lang, "focus.empty_body")} }
                }
            }

            // ── structural toolbar (RFC-023..025) — hidden by default ──────
            // Rearrangement and merge ops; Add section lives in the children
            // area below where it's always accessible.
            div { class: "struct-disclosure",
                button {
                    class: if *show_struct.read() { "struct-toggle struct-toggle--open" } else { "struct-toggle" },
                    title: t(lang, "struct.toolbar.toggle"),
                    onclick: move |_| { let v = !*show_struct.read(); show_struct.set(v); },
                    "⋯"
                }
                if *show_struct.read() {
                    div { class: "struct-toolbar",
                button {
                    class: "struct-btn",
                    title: t(lang, "struct.promote"),
                    disabled: !session.read().can_promote(),
                    onclick: move |_| {
                        if session.write().promote_focused().is_ok() {
                            let body = session.read().current_snapshot()
                                .map(|s| s.body).unwrap_or_default();
                            draft.set(body);
                        } else {
                            status.set("error.struct.invalid_level".into());
                        }
                    },
                    {t(lang, "struct.promote")}
                }
                button {
                    class: "struct-btn",
                    title: t(lang, "struct.demote"),
                    disabled: !session.read().can_demote(),
                    onclick: move |_| {
                        if session.write().demote_focused().is_ok() {
                            let body = session.read().current_snapshot()
                                .map(|s| s.body).unwrap_or_default();
                            draft.set(body);
                        } else {
                            status.set("error.struct.invalid_level".into());
                        }
                    },
                    {t(lang, "struct.demote")}
                }
                button {
                    class: "struct-btn",
                    title: t(lang, "struct.move_up"),
                    disabled: !session.read().can_move_up(),
                    onclick: move |_| {
                        if session.write().move_focused_up().is_ok() {
                            let body = session.read().current_snapshot()
                                .map(|s| s.body).unwrap_or_default();
                            draft.set(body);
                        }
                    },
                    {t(lang, "struct.move_up")}
                }
                button {
                    class: "struct-btn",
                    title: t(lang, "struct.move_down"),
                    disabled: !session.read().can_move_down(),
                    onclick: move |_| {
                        if session.write().move_focused_down().is_ok() {
                            let body = session.read().current_snapshot()
                                .map(|s| s.body).unwrap_or_default();
                            draft.set(body);
                        }
                    },
                    {t(lang, "struct.move_down")}
                }
                button {
                    class: "struct-btn",
                    title: t(lang, "struct.merge_up"),
                    disabled: !session.read().can_merge_up(),
                    onclick: move |_| {
                        if session.write().merge_focused_up().is_ok() {
                            let body = session.read().current_snapshot()
                                .map(|s| s.body).unwrap_or_default();
                            draft.set(body);
                        } else {
                            status.set("error.struct.no_sibling".into());
                        }
                    },
                    {t(lang, "struct.merge_up")}
                }
                button {
                    class: "struct-btn struct-btn--danger",
                    title: t(lang, "struct.delete"),
                    disabled: !session.read().can_delete(),
                    onclick: move |_| {
                        status.set("struct.delete.pending".into());
                    },
                    {t(lang, "struct.delete")}
                }
            }  // end struct-toolbar
            }  // end if *show_struct
            }  // end struct-disclosure

            // ── child sections + add button (always visible) ──────────────
            // "Add section" is here, not inside ⋯, because it is a routine
            // creative act rather than a rare structural operation.
            section {
                class: "children",
                "aria-label": t(lang, "focus.children"),
                for child in snapshot.children.clone() {
                    div {
                        class: "child-row",
                        key: "{child.id.0}",
                        button {
                            class: "child-card",
                            onclick: move |_| {
                                // Commit before zooming into child.
                                let snap = session.read().current_snapshot();
                                if let Some(s) = snap {
                                    let d = draft.read().clone();
                                    if d != s.body {
                                        let _ = session.write().commit_focused_body(&s, d);
                                    }
                                }
                                let _ = session.write().focus(child.id);
                                let body = session.read().current_snapshot()
                                    .map(|s| s.body).unwrap_or_default();
                                draft.set(body);
                            },
                            "{child.title}"
                            if child.child_count > 0 {
                                span { class: "count", " ({child.child_count})" }
                            }
                        }
                        button {
                            class: "child-delete",
                            title: t(lang, "struct.delete_child"),
                            onclick: move |_| {
                                // Navigate into the child, then open the
                                // delete confirmation (same flow as ⋯ Delete).
                                let _ = session.write().focus(child.id);
                                let body = session.read().current_snapshot()
                                    .map(|s| s.body).unwrap_or_default();
                                draft.set(body);
                                status.set("struct.delete.pending".into());
                            },
                            "×"
                        }
                    }
                }
                button {
                    class: "add-child-btn",
                    title: t(lang, "struct.split"),
                    onclick: move |_| {
                        status.set("struct.split.pending".into());
                    },
                    "+ "
                    {t(lang, "struct.split")}
                }
            }
            }
        }
    }
}
