//! Core error and validation taxonomy (RFC-009).
//!
//! `layered-core` returns structured errors and never panics on user-provided
//! Markdown. The UI layer owns the mapping from these variants to localized,
//! user-facing prose (RFC-043); core carries no user-facing strings.

use crate::outline::NodeId;
use crate::range::RangeError;
use crate::revision::DocumentRevision;

/// Errors produced while building or validating the derived outline index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexError {
    /// An internal outline invariant (RFC-006) failed after construction.
    /// This indicates an indexer bug, never invalid user Markdown.
    InvariantViolation(&'static str),
}

impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexError::InvariantViolation(what) => {
                write!(f, "outline invariant violation: {what}")
            }
        }
    }
}

impl std::error::Error for IndexError {}

/// Errors for read-side document operations (parse, lookup, projection).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentError {
    /// Input bytes were not valid UTF-8 (load boundary; see `Document::from_bytes`).
    InvalidUtf8,
    /// The requested node does not exist in the current outline.
    NodeNotFound(NodeId),
    /// A range was invalid against the current source text.
    Range(RangeError),
    /// Outline construction or validation failed.
    Index(IndexError),
}

impl std::fmt::Display for DocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentError::InvalidUtf8 => write!(f, "source is not valid UTF-8"),
            DocumentError::NodeNotFound(id) => write!(f, "node not found: {id:?}"),
            DocumentError::Range(e) => write!(f, "range error: {e}"),
            DocumentError::Index(e) => write!(f, "index error: {e}"),
        }
    }
}

impl std::error::Error for DocumentError {}

impl From<RangeError> for DocumentError {
    fn from(e: RangeError) -> Self {
        DocumentError::Range(e)
    }
}

impl From<IndexError> for DocumentError {
    fn from(e: IndexError) -> Self {
        DocumentError::Index(e)
    }
}

/// Errors for mutating document operations (RFC-008, RFC-044).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditError {
    /// The edit was based on an older document revision; the caller must
    /// reload or rebase its local buffer before committing (RFC-002).
    RevisionMismatch {
        expected: DocumentRevision,
        actual: DocumentRevision,
    },
    /// The target node no longer exists in the current outline.
    StaleNode(NodeId),
    /// The target range is invalid against the current source text.
    InvalidRange(RangeError),
    /// Text was replaced but re-indexing failed; the edit was rolled back
    /// and the document is unchanged (RFC-008 transaction shape).
    IndexAfterEdit(IndexError),
    /// Undo requested with an empty undo history (RFC-044).
    NothingToUndo,
    /// Redo requested with an empty redo history (RFC-044).
    NothingToRedo,
}

impl std::fmt::Display for EditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditError::RevisionMismatch { expected, actual } => write!(
                f,
                "revision mismatch: document is at {expected:?}, edit based on {actual:?}"
            ),
            EditError::StaleNode(id) => write!(f, "stale node: {id:?}"),
            EditError::InvalidRange(e) => write!(f, "invalid range: {e}"),
            EditError::IndexAfterEdit(e) => {
                write!(f, "re-index failed after edit (rolled back): {e}")
            }
            EditError::NothingToUndo => write!(f, "nothing to undo"),
            EditError::NothingToRedo => write!(f, "nothing to redo"),
        }
    }
}

impl std::error::Error for EditError {}

impl From<RangeError> for EditError {
    fn from(e: RangeError) -> Self {
        EditError::InvalidRange(e)
    }
}
