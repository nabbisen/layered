//! Keyboard shortcut contract for layered (RFC-014).
//!
//! [`interpret`] maps a raw keyboard event to an [`AppCommand`] without
//! reading any application state. Mode-specific dispatch (e.g. Enter means
//! zoom-in in outline mode but newline in the focus editor) is the caller's
//! responsibility.

use dioxus::prelude::{KeyboardData, ModifiersInteraction};
use keyboard_types::{Code, Modifiers};

/// An application-level action derived from a keyboard event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCommand {
    /// Open a Markdown file (Ctrl/Cmd+O).
    Open,
    /// Commit focused edit and save the file (Ctrl/Cmd+S).
    Save,
    /// Save to a new path (Ctrl/Cmd+Shift+S).
    SaveAs,
    /// Undo the most recent committed edit (Ctrl/Cmd+Z).
    Undo,
    /// Redo the most recently undone edit (Ctrl/Cmd+Y or Ctrl/Cmd+Shift+Z).
    Redo,
    /// Browser-style back through focus history (Alt+Left).
    Back,
    /// Browser-style forward through focus history (Alt+Right).
    Forward,
    /// Context-sensitive zoom-out or dialog dismiss (Esc).
    Escape,
    /// Context-sensitive confirm or zoom-in (Enter — only when not in a text field).
    Enter,
    /// Move card selection up (↑).
    SelectUp,
    /// Move card selection down (↓).
    SelectDown,
    /// Toggle the raw-source overlay on/off (Ctrl/Cmd+`).
    ToggleRaw,
    /// Open the search panel (Ctrl/Cmd+F).
    OpenSearch,
    /// Open the command palette (Ctrl/Cmd+P).
    OpenPalette,
    /// Toggle the Markdown preview pane (Ctrl/Cmd+Shift+P) — RFC-045.
    TogglePreview,
}

/// Translates a keyboard event into an [`AppCommand`], or `None` if the
/// keystroke is not a recognized shortcut.
///
/// Extracts modifier state from the Dioxus event, then delegates to the
/// pure [`interpret_code`] mapping (RFC-014 §6 test plan).
pub fn interpret(data: &KeyboardData) -> Option<AppCommand> {
    let mods = data.modifiers();
    let ctrl = mods.contains(Modifiers::CONTROL) || mods.contains(Modifiers::META);
    let shift = mods.contains(Modifiers::SHIFT);
    let alt = mods.contains(Modifiers::ALT);
    interpret_code(data.code(), ctrl, shift, alt)
}

/// Pure shortcut mapping: `(code, ctrl, shift, alt) → AppCommand`.
///
/// `ctrl` should already fold in the Cmd/Meta key for macOS. This function
/// has no dependency on Dioxus and is unit-tested directly.
fn interpret_code(code: Code, ctrl: bool, shift: bool, alt: bool) -> Option<AppCommand> {
    match code {
        Code::KeyO if ctrl && !shift && !alt => Some(AppCommand::Open),
        Code::KeyS if ctrl && !shift && !alt => Some(AppCommand::Save),
        Code::KeyS if ctrl && shift && !alt => Some(AppCommand::SaveAs),
        Code::KeyZ if ctrl && !shift && !alt => Some(AppCommand::Undo),
        Code::KeyY if ctrl && !shift && !alt => Some(AppCommand::Redo),
        Code::KeyZ if ctrl && shift && !alt => Some(AppCommand::Redo),
        Code::ArrowLeft if alt && !ctrl && !shift => Some(AppCommand::Back),
        Code::ArrowRight if alt && !ctrl && !shift => Some(AppCommand::Forward),
        Code::Escape if !ctrl && !shift && !alt => Some(AppCommand::Escape),
        Code::Enter if !ctrl && !shift && !alt => Some(AppCommand::Enter),
        Code::ArrowUp if !ctrl && !shift && !alt => Some(AppCommand::SelectUp),
        Code::ArrowDown if !ctrl && !shift && !alt => Some(AppCommand::SelectDown),
        Code::Backquote if ctrl && !shift && !alt => Some(AppCommand::ToggleRaw),
        Code::KeyF if ctrl && !shift && !alt => Some(AppCommand::OpenSearch),
        Code::KeyP if ctrl && !shift && !alt => Some(AppCommand::OpenPalette),
        Code::KeyP if ctrl && shift && !alt => Some(AppCommand::TogglePreview),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_navigation_keys_map_without_modifiers() {
        assert_eq!(
            interpret_code(Code::Escape, false, false, false),
            Some(AppCommand::Escape)
        );
        assert_eq!(
            interpret_code(Code::Enter, false, false, false),
            Some(AppCommand::Enter)
        );
        assert_eq!(
            interpret_code(Code::ArrowUp, false, false, false),
            Some(AppCommand::SelectUp)
        );
        assert_eq!(
            interpret_code(Code::ArrowDown, false, false, false),
            Some(AppCommand::SelectDown)
        );
    }

    #[test]
    fn ctrl_shortcuts_require_ctrl_and_reject_extra_modifiers() {
        assert_eq!(
            interpret_code(Code::KeyS, true, false, false),
            Some(AppCommand::Save)
        );
        // Without ctrl, a bare 'S' is a literal character, not a shortcut.
        assert_eq!(interpret_code(Code::KeyS, false, false, false), None);
        // Ctrl+Alt+S is not a defined binding.
        assert_eq!(interpret_code(Code::KeyS, true, false, true), None);
    }

    #[test]
    fn shift_distinguishes_save_from_save_as() {
        assert_eq!(
            interpret_code(Code::KeyS, true, false, false),
            Some(AppCommand::Save)
        );
        assert_eq!(
            interpret_code(Code::KeyS, true, true, false),
            Some(AppCommand::SaveAs)
        );
    }

    #[test]
    fn both_redo_bindings_resolve() {
        // Ctrl+Y and Ctrl+Shift+Z are both Redo.
        assert_eq!(
            interpret_code(Code::KeyY, true, false, false),
            Some(AppCommand::Redo)
        );
        assert_eq!(
            interpret_code(Code::KeyZ, true, true, false),
            Some(AppCommand::Redo)
        );
        // Ctrl+Z alone is Undo.
        assert_eq!(
            interpret_code(Code::KeyZ, true, false, false),
            Some(AppCommand::Undo)
        );
    }

    #[test]
    fn shift_distinguishes_palette_from_preview() {
        assert_eq!(
            interpret_code(Code::KeyP, true, false, false),
            Some(AppCommand::OpenPalette)
        );
        assert_eq!(
            interpret_code(Code::KeyP, true, true, false),
            Some(AppCommand::TogglePreview)
        );
    }

    #[test]
    fn alt_arrows_navigate_history() {
        assert_eq!(
            interpret_code(Code::ArrowLeft, false, false, true),
            Some(AppCommand::Back)
        );
        assert_eq!(
            interpret_code(Code::ArrowRight, false, false, true),
            Some(AppCommand::Forward)
        );
        // Plain arrows (no alt) are selection movement, handled elsewhere.
        assert_eq!(interpret_code(Code::ArrowLeft, false, false, false), None);
    }

    #[test]
    fn unmapped_keys_return_none() {
        assert_eq!(interpret_code(Code::KeyQ, true, false, false), None);
        assert_eq!(interpret_code(Code::Tab, false, false, false), None);
    }
}
