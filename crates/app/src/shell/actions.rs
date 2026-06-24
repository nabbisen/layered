//! Action handlers — stateless free functions that act on [`AppCtx`] signals.
//!
//! Each function corresponds to one `use_callback` in [`super::app`].
//! Extracting them here keeps `app.rs` to signal wiring and the render tree.

use dioxus::prelude::*;
use omriss_ui::EditorSession;

use crate::components::{ConfirmDeleteChoice, ExtModifiedChoice, SplitChoice, UnsavedChoice};
use crate::file::file_dialog::{self, OpenOutcome, SaveOutcome};
use crate::shell::app_ctx::{AppCtx, Modal, commit_pending, sync_draft};
use crate::storage::settings::AppSettings;

// ── File operations ──────────────────────────────────────────────────────────

/// Handle the result of an open-file operation.
pub(crate) fn handle_load(outcome: OpenOutcome, mut ctx: AppCtx) {
    match outcome {
        OpenOutcome::Cancelled => {}
        OpenOutcome::Failed { cause } => {
            ctx.modal.set(Modal::OpenError { cause });
        }
        OpenOutcome::Loaded {
            text,
            name,
            profile,
            mtime,
        } => {
            match EditorSession::open_with_profile(text, Some(name.clone()), profile) {
                Ok(opened) => {
                    ctx.session.set(opened);
                    ctx.selected_card.set(0);
                    sync_draft(ctx);
                    ctx.saved_mtime.set(mtime);
                    ctx.status.set("status.ready".into());
                    // RFC-036: persist the path to the recent-files list.
                    let mut settings = AppSettings::load();
                    settings.push_recent(&name);
                    settings.save();
                    ctx.recent_files.set(settings.valid_recent_files());
                }
                Err(_) => ctx.modal.set(Modal::OpenError {
                    cause: "Could not parse the file structure.".into(),
                }),
            }
        }
    }
}

/// Open a file, guarded by an unsaved-changes check.
pub(crate) fn handle_open_guarded(mut ctx: AppCtx) {
    if ctx.session.read().is_dirty() {
        ctx.modal.set(Modal::UnsavedBeforeOpen);
    } else {
        handle_load(file_dialog::open_markdown(), ctx);
    }
}

/// Save the current document (`force_new_path = true` triggers Save As).
pub(crate) fn handle_save(mut ctx: AppCtx, force_new_path: bool) {
    commit_pending(ctx);
    let existing = if force_new_path {
        None
    } else {
        ctx.session.read().file_name().map(|s| s.to_string())
    };
    // External modification check (RFC-015).
    if let (Some(path), Some(mtime)) = (existing.as_deref(), *ctx.saved_mtime.read()) {
        if file_dialog::was_modified_externally(path, mtime) {
            ctx.modal.set(Modal::ExternalModified);
            return;
        }
    }
    let profile = ctx.session.read().profile().clone();
    let outcome =
        file_dialog::save_markdown(existing.as_deref(), ctx.session.read().source(), &profile);
    match outcome {
        SaveOutcome::Cancelled => {}
        SaveOutcome::Failed => ctx.status.set("error.save_failed".into()),
        SaveOutcome::Saved { name, mtime } => {
            ctx.session.write().mark_saved(Some(name));
            ctx.saved_mtime.set(mtime);
            ctx.status.set("status.saved".into());
        }
    }
}

/// Create a blank document, resetting all transient state.
pub(crate) fn handle_new(mut ctx: AppCtx) {
    ctx.session.set(EditorSession::new_document());
    ctx.selected_card.set(0);
    ctx.draft.set(String::new());
    ctx.saved_mtime.set(None);
    ctx.status.set("status.ready".into());
}

/// Create a blank document, guarded by an unsaved-changes check.
pub(crate) fn handle_new_guarded(mut ctx: AppCtx) {
    if ctx.session.read().is_dirty() {
        ctx.modal.set(Modal::UnsavedBeforeNew);
    } else {
        handle_new(ctx);
    }
}

// ── Modal handlers ────────────────────────────────────────────────────────────

/// Handle the user's response to the "unsaved changes" dialog.
pub(crate) fn handle_unsaved_choice(choice: UnsavedChoice, mut ctx: AppCtx) {
    let pending = ctx.modal.read().clone();
    match choice {
        UnsavedChoice::Save => {
            handle_save(ctx, false);
            if !ctx.session.read().is_dirty() {
                ctx.modal.set(Modal::None);
                match pending {
                    Modal::UnsavedBeforeOpen => {
                        handle_load(file_dialog::open_markdown(), ctx);
                    }
                    Modal::UnsavedBeforeNew => handle_new(ctx),
                    _ => {}
                }
            }
        }
        UnsavedChoice::Discard => {
            ctx.modal.set(Modal::None);
            match pending {
                Modal::UnsavedBeforeOpen => handle_load(file_dialog::open_markdown(), ctx),
                Modal::UnsavedBeforeNew => handle_new(ctx),
                _ => {}
            }
        }
        UnsavedChoice::Cancel => ctx.modal.set(Modal::None),
    }
}

/// Handle the user's response to the "external modification" dialog.
pub(crate) fn handle_ext_modified_choice(choice: ExtModifiedChoice, mut ctx: AppCtx) {
    ctx.modal.set(Modal::None);
    match choice {
        ExtModifiedChoice::Overwrite => {
            commit_pending(ctx);
            let existing = ctx.session.read().file_name().map(|s| s.to_string());
            let profile = ctx.session.read().profile().clone();
            let outcome = file_dialog::save_markdown(
                existing.as_deref(),
                ctx.session.read().source(),
                &profile,
            );
            match outcome {
                SaveOutcome::Saved { name, mtime } => {
                    ctx.session.write().mark_saved(Some(name));
                    ctx.saved_mtime.set(mtime);
                    ctx.status.set("status.saved".into());
                }
                SaveOutcome::Failed => ctx.status.set("error.save_failed".into()),
                SaveOutcome::Cancelled => {}
            }
        }
        ExtModifiedChoice::SaveAs => handle_save(ctx, true),
        ExtModifiedChoice::Cancel => {}
    }
}

/// Handle the user's response to the section-delete confirmation.
pub(crate) fn handle_confirm_delete(choice: ConfirmDeleteChoice, mut ctx: AppCtx) {
    ctx.modal.set(Modal::None);
    if choice == ConfirmDeleteChoice::Delete {
        let del_result = ctx.session.write().delete_focused();
        match del_result {
            Ok(_) => {
                sync_draft(ctx);
                ctx.status.set("status.unsaved".into());
            }
            Err(_) => ctx.status.set("error.struct.stale_node".into()),
        }
    }
}

/// Handle the user's response to the split-section dialog.
pub(crate) fn handle_split_choice(choice: SplitChoice, mut ctx: AppCtx) {
    ctx.modal.set(Modal::None);
    if let SplitChoice::Confirm(title) = choice {
        // Determine whether we have a focused section to split inside, or
        // whether we are adding a new top-level section from overview mode.
        let has_focus = ctx.session.read().current_snapshot().is_some();

        let result = if has_focus {
            let body_len = ctx
                .session
                .read()
                .current_snapshot()
                .map(|s| s.body.len())
                .unwrap_or(0);
            let level = ctx
                .session
                .read()
                .current_snapshot()
                .and_then(|s| s.level)
                .map(|l| {
                    use omriss::HeadingLevel::*;
                    match l {
                        H1 => H2,
                        H2 => H3,
                        H3 => H4,
                        H4 => H5,
                        _ => H6,
                    }
                })
                .unwrap_or(omriss::HeadingLevel::H2);
            ctx.session.write().split_focused(body_len, &title, level)
        } else {
            ctx.session.write().add_top_level_section(&title)
        };

        match result {
            Ok(_) => {
                // Navigate to the newly created child so it is immediately
                // visible and editable. For split_focused: last child of
                // the currently focused section. For add_top_level_section:
                // last top-level item.
                let new_child = if has_focus {
                    ctx.session
                        .read()
                        .current_snapshot()
                        .and_then(|s| s.children.last().map(|c| c.id))
                } else {
                    ctx.session
                        .read()
                        .outline_items()
                        .last()
                        .map(|item| item.id)
                };
                if let Some(child_id) = new_child {
                    let _ = ctx.session.write().focus(child_id);
                }
                // Sync draft to the new section's body (empty for a freshly
                // created section) so the textarea reflects reality.
                sync_draft(ctx);
                ctx.status.set("status.unsaved".into());
            }
            Err(_) => ctx.status.set("error.struct.stale_node".into()),
        }
    }
}
