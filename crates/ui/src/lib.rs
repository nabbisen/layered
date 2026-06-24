//! # omriss-ui
//!
//! Renderer-independent GUI logic for Omriss: editor sessions,
//! focus navigation state, file text profile, search, command registry,
//! document statistics, and internationalized UI strings. Everything here
//! is plain Rust with no windowing or WebView dependency, so it builds and
//! tests on any host; the desktop shell in `omriss-app` wires these
//! types to Dioxus.
//!
//! ```
//! use omriss_ui::{i18n::{t, Locale}, EditorSession};
//!
//! let mut session = EditorSession::open("# Idea\n\nDraft.\n".to_string(), None)?;
//! let top = session.outline_items();
//! let snapshot = session.focus(top[0].id)?;
//! assert_eq!(snapshot.title, "Idea");
//! assert_eq!(t(Locale::Ja, "toolbar.undo"), "元に戻す");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod commands;
pub mod document_map;
pub mod file_profile;
pub mod i18n;
pub mod navigation;
pub mod search;
mod session;
pub mod stats;
mod view_state;

pub use commands::{COMMANDS, CommandSpec, filter_commands};
pub use document_map::{
    CapabilityReason, DocumentMapNode, DraftState, MapCapability, MapNodeCapabilities,
    node_id_from_raw,
};
pub use file_profile::{FileTextProfile, NewlinePolicy};
pub use navigation::SiblingInfo;
pub use search::SearchMatch;
pub use session::{EditorSession, OutlineNode};
pub use stats::DocumentStats;
pub use view_state::{ViewMode, ViewState};
// Structural editing types re-exported for the desktop crate.
pub use omriss::{MoveTarget, StructuralEditError};

#[cfg(test)]
mod tests;
