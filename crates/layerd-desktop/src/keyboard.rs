//! Keyboard shortcut contract for layerd (RFC-014).
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
/// This function is pure and has no side effects; it is independently
/// testable without a desktop window (RFC-014 §6 test plan).
pub fn interpret(data: &KeyboardData) -> Option<AppCommand> {
    let mods = data.modifiers();
    let ctrl = mods.contains(Modifiers::CONTROL) || mods.contains(Modifiers::META);
    let shift = mods.contains(Modifiers::SHIFT);
    let alt = mods.contains(Modifiers::ALT);

    match data.code() {
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
