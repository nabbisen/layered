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

// ── Internal modules ────────────────────────────────────────────────────────
// doc/ groups the document model; index/ groups the heading tree.
// Public-facing modules stay at the top level for a clean surface.

mod doc;
mod error;
mod index;
mod range;

pub mod preview;
pub mod structural;

#[cfg(test)]
mod tests;

// ── Public API ──────────────────────────────────────────────────────────────
pub use doc::document::{Document, FocusSnapshot, OutlineItem};
pub use doc::edit::{EditResult, ReplaceSectionBody};
pub use doc::history::{DEFAULT_HISTORY_CAPACITY, EditHistory, EditRecord};
pub use doc::revision::DocumentRevision;
pub use error::{DocumentError, EditError, IndexError};
pub use index::outline::{HeadingLevel, NodeId, Outline, SectionNode};
pub use preview::{document_html, section_html};
pub use range::{ByteRange, RangeError};
pub use structural::{MoveTarget, StructuralEditError};
