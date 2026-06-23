//! Edit command and result model (RFC-004, RFC-005, RFC-008).
//!
//! All public document mutations enter through explicit command structs that
//! carry the base revision they were prepared against, enabling stale-buffer
//! detection before any byte is touched.

use crate::outline::NodeId;
use crate::range::ByteRange;
use crate::revision::DocumentRevision;

/// Replace the body of one section, preserving its heading line, child
/// sections, siblings, ancestors, and every unrelated source byte (RFC-004).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplaceSectionBody {
    pub node_id: NodeId,
    /// Revision the caller's edit buffer was created from (RFC-002).
    pub base_revision: DocumentRevision,
    /// Exact replacement for the body range. Core stores it verbatim:
    /// no blank-line normalization, no trailing-newline insertion.
    pub new_body: String,
}

/// Outcome of a successful document mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditResult {
    pub old_revision: DocumentRevision,
    pub new_revision: DocumentRevision,
    /// The range that was replaced, in pre-edit coordinates.
    pub replaced_range: ByteRange,
    /// The range now occupied by the replacement, in post-edit coordinates.
    pub new_range: ByteRange,
    /// Whether the outline was rebuilt as part of this mutation
    /// (always true for M1 synchronous re-indexing).
    pub reindexed: bool,
}
