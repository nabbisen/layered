//! Application settings: persistent local configuration (RFC-036).
//!
//! Settings are stored in the OS-appropriate user config directory:
//!   Linux   : ~/.config/layered/settings.toml
//!   macOS   : ~/Library/Application Support/layered/settings.toml
//!   Windows : %APPDATA%\layered\settings.toml
//!
//! INVARIANT (RFC-036): settings never touch or reference the content of
//! Markdown documents. The settings file may hold file *paths* in the
//! recent-files list, but never document bodies or headings.
//!
//! Failure policy: a missing or corrupt settings file silently falls back to
//! defaults; the user is never blocked from opening the editor by a bad
//! config file.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Maximum number of paths kept in the recent-files list.
const MAX_RECENT: usize = 10;

/// All persisted application preferences (RFC-036 §4 initial settings).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Ordered list of recently opened file paths (most recent first).
    #[serde(default)]
    pub recent_files: Vec<String>,

    /// Editor font size in points.
    #[serde(default = "default_font_size")]
    pub font_size: u8,

    /// Whether long lines wrap in the body editor.
    #[serde(default = "default_true")]
    pub line_wrap: bool,
}

fn default_font_size() -> u8 {
    14
}
fn default_true() -> bool {
    true
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            recent_files: Vec::new(),
            font_size: default_font_size(),
            line_wrap: default_true(),
        }
    }
}

impl AppSettings {
    /// Returns the platform-specific path for the settings file, or `None`
    /// when the config directory cannot be determined.
    pub fn settings_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("layered").join("settings.toml"))
    }

    /// Loads settings from disk, falling back to `Default` on any error.
    /// Errors are silently discarded per RFC-036 failure policy.
    pub fn load() -> Self {
        let Some(path) = Self::settings_path() else {
            return Self::default();
        };
        let Ok(text) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str::<Self>(&text).unwrap_or_default()
    }

    /// Saves settings to disk. Errors are silently discarded.
    pub fn save(&self) {
        let Some(path) = Self::settings_path() else {
            return;
        };
        // Ensure the parent directory exists.
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(text) = toml::to_string_pretty(self) {
            let _ = std::fs::write(&path, text);
        }
    }

    /// Adds `path` to the front of the recent-files list, deduplicating and
    /// capping at `MAX_RECENT` entries.
    pub fn push_recent(&mut self, path: &str) {
        self.recent_files.retain(|p| p != path);
        self.recent_files.insert(0, path.to_string());
        self.recent_files.truncate(MAX_RECENT);
    }

    /// Removes a specific path from the recent-files list.
    #[allow(dead_code)]
    pub(crate) fn remove_recent(&mut self, path: &str) {
        self.recent_files.retain(|p| p != path);
    }

    /// Clears the entire recent-files list.
    #[allow(dead_code)]
    pub(crate) fn clear_recent(&mut self) {
        self.recent_files.clear();
    }

    /// Returns only the recent paths that still exist on disk.
    pub fn valid_recent_files(&self) -> Vec<String> {
        self.recent_files
            .iter()
            .filter(|p| std::path::Path::new(p.as_str()).exists())
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_recents(paths: &[&str]) -> AppSettings {
        let mut s = AppSettings::default();
        // push_recent inserts at front, so push in reverse to get the given order.
        for p in paths.iter().rev() {
            s.push_recent(p);
        }
        s
    }

    #[test]
    fn push_recent_inserts_at_front() {
        let mut s = AppSettings::default();
        s.push_recent("/a.md");
        s.push_recent("/b.md");
        assert_eq!(s.recent_files, vec!["/b.md", "/a.md"]);
    }

    #[test]
    fn push_recent_deduplicates_and_promotes_to_front() {
        let mut s = with_recents(&["/a.md", "/b.md", "/c.md"]);
        // Re-opening an existing file moves it to the front without duplicating.
        s.push_recent("/c.md");
        assert_eq!(s.recent_files, vec!["/c.md", "/a.md", "/b.md"]);
        assert_eq!(s.recent_files.iter().filter(|p| *p == "/c.md").count(), 1);
    }

    #[test]
    fn push_recent_caps_at_max() {
        let mut s = AppSettings::default();
        for i in 0..(MAX_RECENT + 5) {
            s.push_recent(&format!("/file{i}.md"));
        }
        assert_eq!(s.recent_files.len(), MAX_RECENT);
        // The most recently pushed survives; the oldest are dropped.
        assert_eq!(s.recent_files[0], format!("/file{}.md", MAX_RECENT + 4));
    }

    #[test]
    fn remove_recent_drops_only_the_named_path() {
        let mut s = with_recents(&["/a.md", "/b.md", "/c.md"]);
        s.remove_recent("/b.md");
        assert_eq!(s.recent_files, vec!["/a.md", "/c.md"]);
    }
}
