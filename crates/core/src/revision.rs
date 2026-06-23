//! Document revision tracking (RFC-002).
//!
//! Every successful mutation of the canonical text produces a new revision.
//! Edit buffers remember the revision they were created from; a commit
//! against an older revision fails with `EditError::RevisionMismatch`.

/// Monotonic revision counter for the canonical document text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DocumentRevision(pub u64);

impl DocumentRevision {
    /// The revision of a freshly parsed document.
    pub const INITIAL: DocumentRevision = DocumentRevision(0);

    /// Returns the next revision.
    #[must_use]
    pub fn next(self) -> DocumentRevision {
        DocumentRevision(self.0 + 1)
    }
}
