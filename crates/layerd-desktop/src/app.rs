//! Root Dioxus component and global keyboard dispatcher (RFC-010 shell,
//! RFC-014 keyboard contract, RFC-015/016 file lifecycle and dirty guards,
//! RFC-017 raw-source overlay, RFC-019..022 navigation and search).
//!
//! Signal mutation: `use_callback(move |()| { let mut sig = sig; … })` shadows
//! signals with `let mut` so closures are `Fn` (Dioxus 0.7 Writable &mut self).

use std::time::SystemTime;

use dioxus::prelude::*;
use layerd_ui::i18n::{Locale, t};
use layerd_ui::{EditorSession, ViewMode};

use crate::components::{
    CommandPalette, ConfirmDeleteChoice, ConfirmDeleteDialog, ErrorDialog, ExtModifiedChoice,
    ExtModifiedDialog, FocusEditor, OutlinePane, OverviewPane, RawSourceView, SearchPanel,
    SplitChoice, SplitDialog, StatusBar, Toolbar, UnsavedChoice, UnsavedDialog, WelcomeScreen,
};
use crate::file_dialog::{self, OpenOutcome, SaveOutcome};
use crate::keyboard::{self, AppCommand};
use crate::settings::AppSettings;

const STYLE: &str = include_str!("../assets/style.css");

/// Syncs the draft editor to the committed body of the focused section.
fn sync_draft(session: &EditorSession, draft: &mut Signal<String>) {
    draft.set(
        session
            .current_snapshot()
            .map(|s| s.body)
            .unwrap_or_default(),
    );
}

// ── modal state ──────────────────────────────────────────────────────────────

/// Which modal dialog (if any) is currently visible.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum Modal {
    #[default]
    None,
    /// Unsaved changes guard: the pending action runs after the user decides.
    UnsavedBeforeOpen,
    UnsavedBeforeNew,
    /// External modification detected before overwriting the disk file.
    ExternalModified,
    /// Confirm deletion of a section subtree (RFC-025).
    ConfirmDelete {
        title: String,
        child_count: usize,
    },
    /// Collect the title for a new child section (RFC-025 split).
    SplitSection,
    /// File open failed — show cause and recovery (RFC-039).
    OpenError {
        cause: String,
    },
}

#[component]
pub fn App() -> Element {
    let initial_locale = try_consume_context::<Locale>().unwrap_or_default();
    // RFC-036: load persisted settings injected by main.rs at startup.
    let initial_settings = try_consume_context::<AppSettings>().unwrap_or_default();

    let session = use_signal(EditorSession::new_empty);
    let locale = use_signal(move || initial_locale);
    let draft = use_signal(String::new);
    let status = use_signal(|| "status.ready".to_string());
    let selected_card = use_signal(|| 0usize);
    let modal = use_signal(Modal::default);
    // Last-known mtime of the file on disk; used to detect external changes.
    let saved_mtime: Signal<Option<SystemTime>> = use_signal(|| None);
    // RFC-036: recent files list (valid paths only at startup).
    let recent_files = use_signal(move || initial_settings.valid_recent_files());

    // ── helpers ───────────────────────────────────────────────────────────────

    // Commit any pending draft into core before a save.
    let commit_pending = |session: &mut Signal<EditorSession>, draft: &mut Signal<String>| {
        let snap = session.read().current_snapshot();
        if let Some(snapshot) = snap {
            let d = draft.read().clone();
            if d != snapshot.body {
                let _ = session.write().commit_focused_body(&snapshot, d);
            }
        }
    };

    // ── action callbacks ──────────────────────────────────────────────────────

    let do_load = use_callback(move |outcome: OpenOutcome| {
        let mut session = session;
        let mut draft = draft;
        let mut status = status;
        let mut selected_card = selected_card;
        let mut saved_mtime = saved_mtime;
        let mut modal = modal;
        let mut recent_files = recent_files;
        match outcome {
            OpenOutcome::Cancelled => {}
            // RFC-039: surface the specific cause in an error dialog.
            OpenOutcome::Failed { cause } => {
                modal.set(Modal::OpenError { cause });
            }
            OpenOutcome::Loaded {
                text,
                name,
                profile,
                mtime,
            } => {
                match EditorSession::open_with_profile(text, Some(name.clone()), profile) {
                    Ok(opened) => {
                        session.set(opened);
                        selected_card.set(0);
                        sync_draft(&session.read(), &mut draft);
                        saved_mtime.set(mtime);
                        status.set("status.ready".into());
                        // RFC-036: persist the path to the recent-files list.
                        let mut settings = AppSettings::load();
                        settings.push_recent(&name);
                        settings.save();
                        recent_files.set(settings.valid_recent_files());
                    }
                    Err(_) => modal.set(Modal::OpenError {
                        cause: "Could not parse the file structure.".into(),
                    }),
                }
            }
        }
    });

    let do_open_guarded = use_callback(move |()| {
        let mut modal = modal;
        if session.read().is_dirty() {
            modal.set(Modal::UnsavedBeforeOpen);
        } else {
            do_load.call(file_dialog::open_markdown());
        }
    });

    let perform_save = use_callback(move |force_new_path: bool| {
        let mut session = session;
        let mut draft = draft;
        let mut status = status;
        let mut saved_mtime = saved_mtime;
        let mut modal = modal;
        commit_pending(&mut session, &mut draft);
        let existing = if force_new_path {
            None
        } else {
            session.read().file_name().map(|s| s.to_string())
        };
        // External modification check (RFC-015).
        if let (Some(path), Some(mtime)) = (existing.as_deref(), *saved_mtime.read()) {
            if file_dialog::was_modified_externally(path, mtime) {
                modal.set(Modal::ExternalModified);
                return;
            }
        }
        let profile = session.read().profile().clone();
        let outcome =
            file_dialog::save_markdown(existing.as_deref(), session.read().source(), &profile);
        match outcome {
            SaveOutcome::Cancelled => {}
            SaveOutcome::Failed => status.set("error.save_failed".into()),
            SaveOutcome::Saved { name, mtime } => {
                session.write().mark_saved(Some(name));
                saved_mtime.set(mtime);
                status.set("status.saved".into());
            }
        }
    });

    let do_save = use_callback(move |()| perform_save.call(false));
    let do_save_as = use_callback(move |()| perform_save.call(true));

    let do_new_guarded = use_callback(move |()| {
        let mut modal = modal;
        if session.read().is_dirty() {
            modal.set(Modal::UnsavedBeforeNew);
        } else {
            let mut session = session;
            let mut draft = draft;
            let mut status = status;
            let mut selected_card = selected_card;
            let mut saved_mtime = saved_mtime;
            session.set(EditorSession::new_empty());
            selected_card.set(0);
            draft.set(String::new());
            saved_mtime.set(None);
            status.set("status.ready".into());
        }
    });

    // ── modal handlers ────────────────────────────────────────────────────────

    let on_unsaved_choice = use_callback(move |choice: UnsavedChoice| {
        let mut modal = modal;
        let pending = modal.read().clone();
        match choice {
            UnsavedChoice::Save => {
                do_save.call(());
                // If save succeeded (no longer dirty), execute the deferred action.
                if !session.read().is_dirty() {
                    modal.set(Modal::None);
                    match pending {
                        Modal::UnsavedBeforeOpen => do_load.call(file_dialog::open_markdown()),
                        Modal::UnsavedBeforeNew => {
                            let mut session = session;
                            let mut draft = draft;
                            let mut status = status;
                            let mut selected_card = selected_card;
                            let mut saved_mtime = saved_mtime;
                            session.set(EditorSession::new_empty());
                            selected_card.set(0);
                            draft.set(String::new());
                            saved_mtime.set(None);
                            status.set("status.ready".into());
                        }
                        _ => {}
                    }
                }
            }
            UnsavedChoice::Discard => {
                modal.set(Modal::None);
                match pending {
                    Modal::UnsavedBeforeOpen => do_load.call(file_dialog::open_markdown()),
                    Modal::UnsavedBeforeNew => {
                        let mut session = session;
                        let mut draft = draft;
                        let mut status = status;
                        let mut selected_card = selected_card;
                        let mut saved_mtime = saved_mtime;
                        session.set(EditorSession::new_empty());
                        selected_card.set(0);
                        draft.set(String::new());
                        saved_mtime.set(None);
                        status.set("status.ready".into());
                    }
                    _ => {}
                }
            }
            UnsavedChoice::Cancel => modal.set(Modal::None),
        }
    });

    let on_ext_modified_choice = use_callback(move |choice: ExtModifiedChoice| {
        let mut modal = modal;
        modal.set(Modal::None);
        match choice {
            ExtModifiedChoice::Overwrite => {
                // Force write, ignoring the stale mtime.
                let mut session = session;
                let mut draft = draft;
                let mut status = status;
                let mut saved_mtime = saved_mtime;
                commit_pending(&mut session, &mut draft);
                let existing = session.read().file_name().map(|s| s.to_string());
                let profile = session.read().profile().clone();
                let outcome = file_dialog::save_markdown(
                    existing.as_deref(),
                    session.read().source(),
                    &profile,
                );
                match outcome {
                    SaveOutcome::Saved { name, mtime } => {
                        session.write().mark_saved(Some(name));
                        saved_mtime.set(mtime);
                        status.set("status.saved".into());
                    }
                    SaveOutcome::Failed => status.set("error.save_failed".into()),
                    SaveOutcome::Cancelled => {}
                }
            }
            ExtModifiedChoice::SaveAs => do_save_as.call(()),
            ExtModifiedChoice::Cancel => {}
        }
    });

    // ── structural dialog handlers (RFC-023..025) ────────────────────────────

    let on_confirm_delete_choice = use_callback(move |choice: ConfirmDeleteChoice| {
        let mut modal = modal;
        modal.set(Modal::None);
        if choice == ConfirmDeleteChoice::Delete {
            let mut session = session;
            let mut draft = draft;
            let mut status = status;
            let del_result = session.write().delete_focused();
            match del_result {
                Ok(_) => {
                    sync_draft(&session.read(), &mut draft);
                    status.set("status.unsaved".into());
                }
                Err(_) => status.set("error.struct.stale_node".into()),
            }
        }
    });

    let on_split_choice = use_callback(move |choice: SplitChoice| {
        let mut modal = modal;
        modal.set(Modal::None);
        if let SplitChoice::Confirm(title) = choice {
            let mut session = session;
            let mut status = status;
            // Split at end of body (append new child section).
            let body_len = session
                .read()
                .current_snapshot()
                .map(|s| s.body.len())
                .unwrap_or(0);
            let level = session
                .read()
                .current_snapshot()
                .and_then(|s| s.level)
                .map(|l| {
                    use layerd_core::HeadingLevel::*;
                    match l {
                        H1 => H2,
                        H2 => H3,
                        H3 => H4,
                        H4 => H5,
                        _ => H6,
                    }
                })
                .unwrap_or(layerd_core::HeadingLevel::H2);
            match session.write().split_focused(body_len, &title, level) {
                Ok(_) => status.set("status.unsaved".into()),
                Err(_) => status.set("error.struct.stale_node".into()),
            }
        }
    });

    // ── global keyboard handler ───────────────────────────────────────────────

    let search_open = use_signal(|| false);
    let palette_open = use_signal(|| false);
    let preview_open = use_signal(|| false);

    let on_keydown = use_callback(move |event: Event<KeyboardData>| {
        let Some(cmd) = keyboard::interpret(&event.data()) else {
            return;
        };
        let mut session = session;
        let mut draft = draft;
        let mut selected_card = selected_card;
        let mut search_open = search_open;
        let mut palette_open = palette_open;
        let mut status = status;
        let mut preview_open = preview_open;
        let mode = session.read().view_mode();

        match cmd {
            AppCommand::Open => do_open_guarded.call(()),
            AppCommand::Save => do_save.call(()),
            AppCommand::SaveAs => do_save_as.call(()),
            AppCommand::ToggleRaw => {
                if session.read().is_raw() {
                    session.write().leave_raw();
                } else {
                    session.write().show_raw();
                }
            }
            AppCommand::OpenSearch => {
                let v = !*search_open.read();
                search_open.set(v);
            }
            AppCommand::OpenPalette => {
                let v = !*palette_open.read();
                palette_open.set(v);
            }
            AppCommand::TogglePreview => {
                // Commit draft before switching to preview (RFC-045).
                let snap = session.read().current_snapshot();
                if let Some(s) = snap {
                    let d = draft.read().clone();
                    if d != s.body {
                        let _ = session.write().commit_focused_body(&s, d);
                    }
                }
                let v = !*preview_open.read();
                preview_open.set(v);
            }
            AppCommand::Undo => {
                let in_focus = matches!(mode, ViewMode::Focus(_));
                let uncommitted = in_focus && {
                    let d = draft.read().clone();
                    session
                        .read()
                        .current_snapshot()
                        .is_some_and(|s| d != s.body)
                };
                if !uncommitted && session.write().undo().is_ok() {
                    sync_draft(&session.read(), &mut draft);
                }
            }
            AppCommand::Redo => {
                let in_focus = matches!(mode, ViewMode::Focus(_));
                let uncommitted = in_focus && {
                    let d = draft.read().clone();
                    session
                        .read()
                        .current_snapshot()
                        .is_some_and(|s| d != s.body)
                };
                if !uncommitted && session.write().redo().is_ok() {
                    sync_draft(&session.read(), &mut draft);
                }
            }
            AppCommand::Back => {
                session.write().back();
                let stale = session.write().prune_and_report();
                sync_draft(&session.read(), &mut draft);
                if stale {
                    status.set("nav.stale_section".into());
                }
            }
            AppCommand::Forward => {
                session.write().forward();
                let stale = session.write().prune_and_report();
                sync_draft(&session.read(), &mut draft);
                if stale {
                    status.set("nav.stale_section".into());
                }
            }
            AppCommand::Escape => {
                if *search_open.read() {
                    search_open.set(false);
                } else if *palette_open.read() {
                    palette_open.set(false);
                } else if session.read().is_raw() {
                    session.write().leave_raw();
                } else if matches!(mode, ViewMode::Focus(_)) {
                    let snap = session.read().current_snapshot();
                    if let Some(snapshot) = snap {
                        let d = draft.read().clone();
                        if d != snapshot.body {
                            let _ = session.write().commit_focused_body(&snapshot, d);
                        }
                    }
                    session.write().zoom_out();
                    sync_draft(&session.read(), &mut draft);
                    selected_card.set(0);
                }
            }
            AppCommand::Enter => {
                if matches!(mode, ViewMode::Outline) {
                    let items = session.read().current_children();
                    let idx = *selected_card.read();
                    if let Some(item) = items.get(idx) {
                        let id = item.id;
                        let _ = session.write().focus(id);
                        sync_draft(&session.read(), &mut draft);
                    }
                }
            }
            AppCommand::SelectUp => {
                if matches!(mode, ViewMode::Outline) {
                    let len = session.read().current_children().len();
                    if len > 0 {
                        let cur = *selected_card.read();
                        selected_card.set(if cur == 0 { len - 1 } else { cur - 1 });
                    }
                }
            }
            AppCommand::SelectDown => {
                if matches!(mode, ViewMode::Outline) {
                    let len = session.read().current_children().len();
                    if len > 0 {
                        let next = (*selected_card.read() + 1) % len;
                        selected_card.set(next);
                    }
                }
            }
        }
    });

    // Palette command execution: translate id → action.
    let on_palette_execute = use_callback(move |id: crate::components::CommandId| match id {
        "file.open" => do_open_guarded.call(()),
        "file.new" => do_new_guarded.call(()),
        "file.save" => do_save.call(()),
        "file.save_as" => do_save_as.call(()),
        "view.raw" => {
            let mut session = session;
            if session.read().is_raw() {
                session.write().leave_raw();
            } else {
                session.write().show_raw();
            }
        }
        "search.open" => {
            let mut so = search_open;
            so.set(true);
        }
        _ => {}
    });

    // ── layout ────────────────────────────────────────────────────────────────

    // Intercept sentinel status values written by FocusEditor structural buttons
    // and open the corresponding modal (RFC-025). Clear the sentinel immediately.
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
        }
    }

    let is_welcome = session.read().source().is_empty() && !session.read().is_dirty();
    let is_raw = session.read().is_raw();
    let mode = session.read().view_mode();

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
                    OutlinePane { session, locale, draft, selected_card }
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
                            ViewMode::Focus(_) => rsx! {
                                FocusEditor { session, locale, draft, status, preview_open }
                            },
                        }
                    }
                }
            }

            StatusBar {
                session, locale, status,
                on_save_as: move |()| do_save_as.call(()),
            }

            // ── overlays and modal dialogs ────────────────────────────────────
            if *search_open.read() {
                SearchPanel {
                    session, locale,
                    on_close: move |()| { let mut so = search_open; so.set(false); },
                    on_navigate: move |id| {
                        let mut session = session;
                        let mut draft = draft;
                        let _ = session.write().focus(id);
                        sync_draft(&session.read(), &mut draft);
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
