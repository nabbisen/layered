//! Root Dioxus component — signal wiring and render tree.
//!
//! Action logic lives in [`actions`]. Keyboard/palette dispatch lives in
//! [`dispatch`]. Modal enum and signal helpers live in [`app_ctx`].

use std::time::SystemTime;

use dioxus::prelude::*;
use omriss_ui::i18n::{Locale, t};
use omriss_ui::{EditorSession, ViewMode};

use crate::components::{
    CommandPalette, ConfirmDeleteChoice, ConfirmDeleteDialog, DocumentMapPane, ErrorDialog,
    ExtModifiedChoice, ExtModifiedDialog, FocusedContentPane, OverviewPane, RawSourceView,
    SearchPanel, SplitChoice, SplitDialog, StatusBar, Toolbar, UnsavedChoice, UnsavedDialog,
    WelcomeScreen,
};
use crate::file::file_dialog;
use crate::input::keyboard;
use crate::shell::actions::{
    handle_confirm_delete, handle_ext_modified_choice, handle_load, handle_new_guarded,
    handle_open_guarded, handle_save, handle_split_choice, handle_unsaved_choice,
};
use crate::shell::app_ctx::{AppCtx, Modal};
use crate::shell::dispatch::{dispatch_command, dispatch_palette};
use crate::storage::settings::AppSettings;

const STYLE: &str = include_str!("../../assets/style.css");

#[component]
pub fn App() -> Element {
    let initial_locale = try_consume_context::<Locale>().unwrap_or_default();
    let initial_settings = try_consume_context::<AppSettings>().unwrap_or_default();

    // ── signals ───────────────────────────────────────────────────────────────

    let session: Signal<EditorSession> = use_signal(EditorSession::new_empty);
    let locale = use_signal(move || initial_locale);
    let draft = use_signal(String::new);
    let status = use_signal(|| "status.ready".to_string());
    let selected_card = use_signal(|| 0usize);
    let modal: Signal<Modal> = use_signal(Modal::default);
    let saved_mtime: Signal<Option<SystemTime>> = use_signal(|| None);
    let recent_files = use_signal(move || initial_settings.valid_recent_files());
    let search_open = use_signal(|| false);
    let palette_open = use_signal(|| false);
    let preview_open = use_signal(|| false);

    let ctx = AppCtx {
        session,
        draft,
        status,
        selected_card,
        modal,
        saved_mtime,
        recent_files,
    };

    // ── callbacks — thin wrappers around free functions ───────────────────────

    let do_load = use_callback(move |outcome| handle_load(outcome, ctx));
    let do_open_guarded = use_callback(move |()| handle_open_guarded(ctx));
    let do_save = use_callback(move |()| handle_save(ctx, false));
    let do_save_as = use_callback(move |()| handle_save(ctx, true));
    let do_new_guarded = use_callback(move |()| handle_new_guarded(ctx));

    let on_unsaved_choice =
        use_callback(move |choice: UnsavedChoice| handle_unsaved_choice(choice, ctx));
    let on_ext_modified_choice =
        use_callback(move |choice: ExtModifiedChoice| handle_ext_modified_choice(choice, ctx));
    let on_confirm_delete_choice =
        use_callback(move |choice: ConfirmDeleteChoice| handle_confirm_delete(choice, ctx));
    let on_split_choice = use_callback(move |choice: SplitChoice| handle_split_choice(choice, ctx));

    let on_keydown = use_callback(move |event: Event<KeyboardData>| {
        let Some(cmd) = keyboard::interpret(&event.data()) else {
            return;
        };
        let mode = session.read().view_mode();
        dispatch_command(cmd, mode, ctx, search_open, palette_open, preview_open);
    });

    let on_palette_execute = use_callback(move |id| dispatch_palette(id, ctx, search_open));

    // ── structural sentinel intercept (RFC-025) ───────────────────────────────
    // FocusEditor writes a sentinel into `status` to request a modal; detect
    // and replace it here on each render pass.
    {
        let st = status.read().clone();
        if st == "struct.delete.pending" {
            let mut modal = modal;
            let mut status = status;
            let snap = session.read().current_snapshot();
            if let Some(s) = snap {
                let title = s.title.clone();
                let child_count = s.children.len();
                modal.set(Modal::ConfirmDelete { title, child_count });
            }
            status.set("status.ready".into());
        } else if st == "struct.split.pending" {
            let mut modal = modal;
            let mut status = status;
            modal.set(Modal::SplitSection);
            status.set("status.ready".into());
            // Focus the dialog input after the DOM fully settles.
            // The + button's session.write().focus() triggers DocumentMapPane's
            // use_effect → item_tree write → another render+patch inside the
            // first rAF. A second rAF is guaranteed to fire after that patch.
            spawn(async move {
                let _ = document::eval(
                    "requestAnimationFrame(() => requestAnimationFrame(() => document.querySelector('.split-dialog-input')?.focus()))",
                );
            });
        }
    }

    let is_welcome = !session.read().document_open();
    let is_raw = session.read().is_raw();
    let mode = session.read().view_mode();

    // ── render tree ───────────────────────────────────────────────────────────

    rsx! {
        style { {STYLE} }
        div {
            class: "app",
            tabindex: 0,
            onkeydown: move |event| on_keydown.call(event),

            Toolbar {
                session, locale, draft, status,
                on_open: move |()| do_open_guarded.call(()),
                on_save: move |()| do_save.call(()),
                on_save_as: move |()| do_save_as.call(()),
            }

            if is_welcome {
                WelcomeScreen {
                    locale,
                    recent_files,
                    on_open: move |()| do_open_guarded.call(()),
                    on_new: move |()| do_new_guarded.call(()),
                    on_open_recent: move |path: String| {
                        do_load.call(file_dialog::open_markdown_path(&path));
                    },
                }
            } else {
                div { class: "body",
                    // RFC-049: Document Map is the single structure-organization surface.
                    DocumentMapPane { session, locale, draft, status }
                    if is_raw {
                        RawSourceView {
                            session,
                            locale,
                            on_back: move |()| {
                                let mut session = session;
                                session.write().leave_raw();
                            },
                        }
                    } else {
                        match mode {
                            ViewMode::Outline | ViewMode::RawSource => rsx! {
                                OverviewPane { session, locale, draft, selected_card }
                            },
                            // RFC-050: FocusedContentPane has no structure controls.
                            ViewMode::Focus(_) => rsx! {
                                FocusedContentPane { session, locale, draft, status, preview_open }
                            },
                        }
                    }
                }
            }

            StatusBar {
                session, locale, status,
                on_save_as: move |()| do_save_as.call(()),
            }

            if *search_open.read() {
                SearchPanel {
                    session, locale,
                    on_close: move |()| { let mut so = search_open; so.set(false); },
                    on_navigate: move |id| {
                        let mut session = session;
                        let mut draft = draft;
                        let _ = session.write().focus(id);
                        let body = session.read().current_snapshot()
                            .map(|s| s.body).unwrap_or_default();
                        draft.set(body);
                    },
                }
            }

            if *palette_open.read() {
                CommandPalette {
                    locale,
                    on_close: move |()| { let mut po = palette_open; po.set(false); },
                    on_execute: move |id| on_palette_execute.call(id),
                }
            }

            match *modal.read() {
                Modal::None => rsx! {},
                Modal::UnsavedBeforeOpen | Modal::UnsavedBeforeNew => rsx! {
                    UnsavedDialog {
                        locale,
                        on_choice: move |choice| on_unsaved_choice.call(choice),
                    }
                },
                Modal::ExternalModified => rsx! {
                    ExtModifiedDialog {
                        locale,
                        on_choice: move |choice| on_ext_modified_choice.call(choice),
                    }
                },
                Modal::ConfirmDelete { ref title, child_count } => rsx! {
                    ConfirmDeleteDialog {
                        locale,
                        section_title: title.clone(),
                        child_count,
                        on_choice: move |c| on_confirm_delete_choice.call(c),
                    }
                },
                Modal::SplitSection => rsx! {
                    SplitDialog {
                        locale,
                        on_choice: move |c| on_split_choice.call(c),
                    }
                },
                Modal::OpenError { ref cause } => rsx! {
                    ErrorDialog {
                        locale,
                        title: "error.open_failed".to_string(),
                        cause: cause.clone(),
                        dismiss_label: t(*locale.read(), "dialog.discard.cancel").to_string(),
                        secondary_label: None,
                        on_dismiss: move |()| { let mut m = modal; m.set(Modal::None); },
                        on_secondary: None,
                    }
                },
            }
        }
    }
}
