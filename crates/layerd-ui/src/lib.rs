//! # layerd-ui
//!
//! Renderer-independent GUI logic for the layerd editor: editor sessions,
//! focus navigation state, file text profile, search, command registry,
//! and internationalized UI strings (RFC-043). Everything here is plain Rust
//! with no windowing or WebView dependency, so it builds and tests on any
//! host; the desktop shell in `layerd-desktop` wires these types to Dioxus.
//!
//! ```
//! use layerd_ui::{i18n::{t, Locale}, EditorSession};
//!
//! let mut session = EditorSession::open("# Idea\n\nDraft.\n".to_string(), None)?;
//! let top = session.outline_items();
//! let snapshot = session.focus(top[0].id)?;
//! assert_eq!(snapshot.title, "Idea");
//! assert_eq!(t(Locale::Ja, "toolbar.undo"), "元に戻す");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod commands;
pub mod file_profile;
pub mod i18n;
pub mod navigation;
pub mod search;
mod session;
mod view_state;

pub use commands::{COMMANDS, CommandSpec, filter_commands};
pub use file_profile::{FileTextProfile, NewlinePolicy};
pub use navigation::SiblingInfo;
pub use search::SearchMatch;
pub use session::EditorSession;
pub use view_state::{ViewMode, ViewState};

#[cfg(test)]
mod tests;
