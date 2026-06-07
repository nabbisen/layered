//! Native file dialogs, disk I/O, and file integrity helpers for the desktop
//! shell (RFC-001: platform integration; RFC-015: file lifecycle; RFC-018:
//! encoding and line-ending integrity).
//!
//! Canonical text crosses this boundary verbatim in both directions (RFC-002).
//! Saves use write-to-temp-then-rename for crash safety (NFR-REL-003).

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use layerd_ui::file_profile::{FileTextProfile, NewlinePolicy};

const MD_EXTENSIONS: &[&str] = &["md", "markdown", "mdown", "txt"];

// ── outcome types ────────────────────────────────────────────────────────────

/// Result of asking the user to open a Markdown file.
pub enum OpenOutcome {
    /// The user dismissed the dialog.
    Cancelled,
    /// File could not be read or is not valid UTF-8.
    Failed,
    /// Text (BOM stripped), display name, profile, and disk mtime.
    Loaded {
        text: String,
        name: String,
        profile: FileTextProfile,
        mtime: Option<SystemTime>,
    },
}

/// Result of saving the canonical source text.
pub enum SaveOutcome {
    /// The user dismissed the Save As dialog.
    Cancelled,
    /// Write failed; in-memory state is untouched.
    Failed,
    /// Path written, new disk mtime recorded.
    Saved {
        name: String,
        mtime: Option<SystemTime>,
    },
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn markdown_dialog() -> rfd::FileDialog {
    rfd::FileDialog::new().add_filter("Markdown", MD_EXTENSIONS)
}

/// Reads `path`, strips a UTF-8 BOM if present, and returns the text
/// together with its profile and last-modified time.
fn read_markdown(path: &Path) -> Result<(String, FileTextProfile, Option<SystemTime>), ()> {
    let bytes = fs::read(path).map_err(|_| ())?;
    let mtime = fs::metadata(path).ok().and_then(|m| m.modified().ok());

    // Strip UTF-8 BOM (EF BB BF) if present.
    let (text_bytes, had_bom) = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        (&bytes[3..], true)
    } else {
        (&bytes[..], false)
    };

    let text = String::from_utf8(text_bytes.to_vec()).map_err(|_| ())?;
    let profile = FileTextProfile::detect(&text, had_bom);
    Ok((text, profile, mtime))
}

/// Writes `source` atomically: writes a temp file in the same directory then
/// renames over the target (NFR-REL-003 crash-safe save).
fn write_atomic(
    path: &Path,
    source: &str,
    profile: &FileTextProfile,
) -> Result<SystemTime, std::io::Error> {
    // Re-prepend BOM if the original file had one.
    let temp = path.with_extension("tmp.layerd");
    if profile.had_utf8_bom {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(source.as_bytes());
        fs::write(&temp, &bytes)?;
    } else {
        fs::write(&temp, source)?;
    }
    fs::rename(&temp, path)?;
    let mtime = fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .unwrap_or(SystemTime::UNIX_EPOCH);
    Ok(mtime)
}

// ── public API ───────────────────────────────────────────────────────────────

/// Shows the open dialog and reads the chosen Markdown file.
pub fn open_markdown() -> OpenOutcome {
    let Some(path) = markdown_dialog().pick_file() else {
        return OpenOutcome::Cancelled;
    };
    match read_markdown(&path) {
        Ok((text, profile, mtime)) => OpenOutcome::Loaded {
            text,
            name: path.display().to_string(),
            profile,
            mtime,
        },
        Err(()) => OpenOutcome::Failed,
    }
}

/// Writes `source` to the session's existing path, or prompts for one.
/// `profile` is used to restore the BOM if the original file had one.
pub fn save_markdown(
    existing: Option<&str>,
    source: &str,
    profile: &FileTextProfile,
) -> SaveOutcome {
    let target = match existing {
        Some(name) => Some(PathBuf::from(name)),
        None => markdown_dialog().save_file(),
    };
    let Some(path) = target else {
        return SaveOutcome::Cancelled;
    };
    match write_atomic(&path, source, profile) {
        Ok(mtime) => SaveOutcome::Saved {
            name: path.display().to_string(),
            mtime: Some(mtime),
        },
        Err(_) => SaveOutcome::Failed,
    }
}

/// Returns `true` if the file at `path` was modified after `saved_mtime`,
/// indicating an external modification that would be overwritten (RFC-015).
pub fn was_modified_externally(path: &str, saved_mtime: SystemTime) -> bool {
    fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .map(|disk_mtime| disk_mtime > saved_mtime)
        .unwrap_or(false)
}

/// Line-ending policy label for the status bar (RFC-018).
pub fn newline_label(policy: NewlinePolicy) -> &'static str {
    policy.label()
}
