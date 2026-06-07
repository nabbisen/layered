//! Native file dialogs and disk I/O for the desktop shell (RFC-001: this
//! crate owns platform integration; RFC-015 will formalize the full
//! open/save lifecycle, atomic writes, and external-modification handling).
//!
//! The canonical text crosses this boundary verbatim in both directions
//! (RFC-002): bytes read from disk become the document source unchanged, and
//! saving writes `EditorSession::source()` back exactly.

use std::fs;
use std::path::PathBuf;

const MD_EXTENSIONS: &[&str] = &["md", "markdown", "txt"];

/// Result of asking the user to open a Markdown file.
pub enum OpenOutcome {
    /// The user dismissed the dialog; nothing changes.
    Cancelled,
    /// The file could not be read or was not valid UTF-8.
    Failed,
    /// File contents and display name, ready for `EditorSession::open`.
    Loaded { text: String, name: String },
}

/// Result of saving the canonical source text.
pub enum SaveOutcome {
    /// The user dismissed the Save As dialog; nothing changes.
    Cancelled,
    /// The write failed; in-memory state is untouched.
    Failed,
    /// Display name of the path that was written.
    Saved { name: String },
}

fn markdown_dialog() -> rfd::FileDialog {
    rfd::FileDialog::new().add_filter("Markdown", MD_EXTENSIONS)
}

/// Shows the open dialog and reads the chosen file as UTF-8 text.
pub fn open_markdown() -> OpenOutcome {
    let Some(path) = markdown_dialog().pick_file() else {
        return OpenOutcome::Cancelled;
    };
    let Ok(bytes) = fs::read(&path) else {
        return OpenOutcome::Failed;
    };
    match String::from_utf8(bytes) {
        Ok(text) => OpenOutcome::Loaded {
            text,
            name: path.display().to_string(),
        },
        Err(_) => OpenOutcome::Failed,
    }
}

/// Writes `source` to the session's existing path, or asks for one (Save As)
/// when the document has never been saved.
pub fn save_markdown(existing: Option<&str>, source: &str) -> SaveOutcome {
    let target = match existing {
        Some(name) => Some(PathBuf::from(name)),
        None => markdown_dialog().save_file(),
    };
    let Some(path) = target else {
        return SaveOutcome::Cancelled;
    };
    match fs::write(&path, source) {
        Ok(()) => SaveOutcome::Saved {
            name: path.display().to_string(),
        },
        Err(_) => SaveOutcome::Failed,
    }
}
