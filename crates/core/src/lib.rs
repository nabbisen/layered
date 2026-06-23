//! # omriss
//!
//! Source-preserving Markdown document engine for **Omriss**, the layer
//! editor. This crate owns the canonical document model, the derived outline
//! index, and all edit operations. It is UI-independent by design (RFC-001):
//! it must never depend on Dioxus, WebView, file dialogs, or any desktop
//! runtime, and every behavior here is testable through `cargo test` alone.
//!
//! ## The core invariant
//!
//! > The canonical document is the raw Markdown source text. The outline tree
//! > is a derived navigational index. Editing one section must not rewrite
//! > unrelated source bytes.
//!
//! ## Quick example
//!
//! ```
//! use omriss::{Document, ReplaceSectionBody};
//!
//! let mut doc = Document::parse("# A\nbody\n\n# B\nkeep me\n".to_string()).unwrap();
//! let first = doc.outline().root().children[0];
//! let result = doc.replace_section_body(ReplaceSectionBody {
//!     node_id: first,
//!     base_revision: doc.revision(),
//!     new_body: "rewritten\n\n".to_string(),
//! }).unwrap();
//! assert!(result.reindexed);
//! assert_eq!(doc.source(), "# A\nrewritten\n\n# B\nkeep me\n");
//! doc.undo().unwrap();
//! assert_eq!(doc.source(), "# A\nbody\n\n# B\nkeep me\n");
//! ```

mod document;
mod edit;
mod error;
mod history;
mod index;
mod outline;
pub mod preview;
mod range;
mod revision;
pub mod structural;

#[cfg(test)]
mod tests;

pub use document::{Document, FocusSnapshot, OutlineItem};
pub use edit::{EditResult, ReplaceSectionBody};
pub use error::{DocumentError, EditError, IndexError};
pub use history::{DEFAULT_HISTORY_CAPACITY, EditHistory, EditRecord};
pub use outline::{HeadingLevel, NodeId, Outline, SectionNode};
pub use preview::{document_html, section_html};
pub use range::{ByteRange, RangeError};
pub use revision::DocumentRevision;
pub use structural::{MoveTarget, StructuralEditError};
