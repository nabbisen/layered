//! # layerd-ui
//!
//! Renderer-independent GUI logic for the layerd editor: editor sessions,
//! focus navigation state, and internationalized UI strings (RFC-009,
//! RFC-043). Everything here is plain Rust with no windowing or WebView
//! dependency, so it builds and tests on any host; the desktop shell in
//! `layerd-desktop` wires these types to Dioxus components.
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

pub mod i18n;
mod session;
mod view_state;

pub use session::EditorSession;
pub use view_state::{ViewMode, ViewState};

#[cfg(test)]
mod tests;
