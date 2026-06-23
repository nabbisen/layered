//! Operation-based undo/redo for committed body edits (RFC-044).
//!
//! Every successful mutation records both directions explicitly: the text it
//! removed and the text it inserted, with their ranges in the respective
//! coordinate spaces. Undo and redo are ordinary revision-incrementing
//! mutations applied by [`crate::Document`]; this module only owns the stacks.

use crate::range::ByteRange;
use crate::revision::DocumentRevision;

/// Default bound on retained history entries; oldest entries drop first.
pub const DEFAULT_HISTORY_CAPACITY: usize = 100;

/// A reversible record of one committed edit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditRecord {
    /// Range that was replaced, in pre-edit coordinates.
    pub replaced_range: ByteRange,
    /// Text that the edit removed.
    pub old_text: String,
    /// Range the replacement occupies, in post-edit coordinates.
    pub new_range: ByteRange,
    /// Text that the edit inserted.
    pub new_text: String,
    pub revision_before: DocumentRevision,
    pub revision_after: DocumentRevision,
}

/// Undo/redo stacks over [`EditRecord`]s.
#[derive(Debug, Clone, Default)]
pub struct EditHistory {
    undo_stack: Vec<EditRecord>,
    redo_stack: Vec<EditRecord>,
}

impl EditHistory {
    /// Records a freshly applied edit; clears the redo stack and enforces
    /// the capacity bound.
    pub(crate) fn record(&mut self, record: EditRecord) {
        self.redo_stack.clear();
        self.undo_stack.push(record);
        if self.undo_stack.len() > DEFAULT_HISTORY_CAPACITY {
            self.undo_stack.remove(0);
        }
    }

    /// Pops the most recent edit for undoing.
    pub(crate) fn pop_undo(&mut self) -> Option<EditRecord> {
        self.undo_stack.pop()
    }

    /// Pops the most recently undone edit for redoing.
    pub(crate) fn pop_redo(&mut self) -> Option<EditRecord> {
        self.redo_stack.pop()
    }

    /// Pushes an undone edit onto the redo stack.
    pub(crate) fn push_redo(&mut self, record: EditRecord) {
        self.redo_stack.push(record);
    }

    /// Pushes a redone edit back onto the undo stack (no redo clearing).
    pub(crate) fn push_undo(&mut self, record: EditRecord) {
        self.undo_stack.push(record);
    }

    /// Whether an undo is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Whether a redo is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
