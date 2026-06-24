//! Keyboard-command and command-palette dispatch (RFC-014, RFC-022).
//!
//! Free functions called from `use_callback` wrappers in [`super::app`].
//! Separated here to keep `app.rs` under the 500-ELOC guideline.

use dioxus::prelude::*;
use omriss_ui::ViewMode;

use crate::components::CommandId;
use crate::input::keyboard::AppCommand;
use crate::shell::actions::{handle_new_guarded, handle_open_guarded, handle_save};
use crate::shell::app_ctx::{AppCtx, sync_draft};

// ── Keyboard dispatch ─────────────────────────────────────────────────────────

/// Handle one decoded [`AppCommand`] against the current view mode.
/// Called inside the root `onkeydown` handler.
pub(crate) fn dispatch_command(
    cmd: AppCommand,
    mode: ViewMode,
    ctx: AppCtx,
    mut search_open: Signal<bool>,
    mut palette_open: Signal<bool>,
    mut preview_open: Signal<bool>,
) {
    let AppCtx {
        mut session,
        draft,
        mut selected_card,
        mut status,
        ..
    } = ctx;

    match cmd {
        AppCommand::Open => handle_open_guarded(ctx),
        AppCommand::Save => handle_save(ctx, false),
        AppCommand::SaveAs => handle_save(ctx, true),
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
                sync_draft(ctx);
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
                sync_draft(ctx);
            }
        }
        AppCommand::Back => {
            session.write().back();
            let stale = session.write().prune_and_report();
            sync_draft(ctx);
            if stale {
                status.set("nav.stale_section".into());
            }
        }
        AppCommand::Forward => {
            session.write().forward();
            let stale = session.write().prune_and_report();
            sync_draft(ctx);
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
                sync_draft(ctx);
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
                    sync_draft(ctx);
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
}

// ── Palette dispatch ──────────────────────────────────────────────────────────

/// Handle a command selected from the command palette.
pub(crate) fn dispatch_palette(id: CommandId, ctx: AppCtx, mut search_open: Signal<bool>) {
    let AppCtx { mut session, .. } = ctx;
    match id {
        "file.open" => handle_open_guarded(ctx),
        "file.new" => handle_new_guarded(ctx),
        "file.save" => handle_save(ctx, false),
        "file.save_as" => handle_save(ctx, true),
        "view.raw" => {
            if session.read().is_raw() {
                session.write().leave_raw();
            } else {
                session.write().show_raw();
            }
        }
        "search.open" => search_open.set(true),
        "view.stats" => {
            // RFC-046: show document statistics as a status message.
            let stats = session.read().stats();
            let msg = if stats.focused_words > 0 {
                format!(
                    "{} words (section) / {} words · {} sections",
                    stats.focused_words, stats.total_words, stats.section_count
                )
            } else {
                format!(
                    "{} words · {} sections",
                    stats.total_words, stats.section_count
                )
            };
            ctx.status.clone().set(msg);
        }
        _ => {}
    }
}
