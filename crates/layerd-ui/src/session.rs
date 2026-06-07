//! Editor session: one open document plus everything the GUI needs around it.
//!
//! `EditorSession` glues `layerd_core::Document` to the view state: it owns
//! dirty tracking (content-based, so undoing back to the saved bytes clears
//! the flag), routes focused-body edits through the core command API, and
//! prunes navigation history when sections disappear after structural edits.

use std::hash::{DefaultHasher, Hash, Hasher};

use layerd_core::{
    Document, DocumentError, DocumentRevision, EditError, EditResult, FocusSnapshot, NodeId,
    OutlineItem, ReplaceSectionBody,
};

use crate::view_state::{ViewMode, ViewState};

/// Length + hash of the text at the last save. Undo and redo mint fresh
/// revisions (RFC-044), so "back to the saved bytes" must be detected by
/// content, not by revision number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SavedFingerprint {
    revision: DocumentRevision,
    len: usize,
    hash: u64,
}

impl SavedFingerprint {
    fn of(text: &str, revision: DocumentRevision) -> Self {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        Self {
            revision,
            len: text.len(),
            hash: hasher.finish(),
        }
    }

    fn matches(&self, text: &str, revision: DocumentRevision) -> bool {
        if revision == self.revision {
            return true; // fast path: nothing committed since the save
        }
        Self::of(text, revision).hash == self.hash && text.len() == self.len
    }
}

/// One open document and its surrounding editor state.
#[derive(Debug, Clone)]
pub struct EditorSession {
    document: Document,
    view: ViewState,
    saved: SavedFingerprint,
    /// Display name or path of the backing file, if any.
    file_name: Option<String>,
}

impl EditorSession {
    /// Opens a session over Markdown text (e.g. file contents just read).
    pub fn open(markdown: String, file_name: Option<String>) -> Result<Self, DocumentError> {
        let document = Document::parse(markdown)?;
        let saved = SavedFingerprint::of(document.source(), document.revision());
        Ok(Self {
            document,
            view: ViewState::new(),
            saved,
            file_name,
        })
    }

    /// Starts an empty, unsaved document.
    pub fn new_empty() -> Self {
        Self::open(String::new(), None).expect("empty text always parses")
    }

    /// The canonical source text — exactly what `save` must write.
    pub fn source(&self) -> &str {
        self.document.source()
    }

    /// Display name of the backing file, if any.
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }

    /// True when the text differs byte-wise from the last saved text.
    pub fn is_dirty(&self) -> bool {
        !self
            .saved
            .matches(self.document.source(), self.document.revision())
    }

    /// Records that the current text was written out successfully.
    pub fn mark_saved(&mut self, file_name: Option<String>) {
        self.saved = SavedFingerprint::of(self.document.source(), self.document.revision());
        if file_name.is_some() {
            self.file_name = file_name;
        }
    }

    /// Current view (outline or focused section).
    pub fn view_mode(&self) -> ViewMode {
        self.view.mode()
    }

    /// Read-only navigation state for toolbar enablement.
    pub fn can_go_back(&self) -> bool {
        self.view.can_go_back()
    }

    pub fn can_go_forward(&self) -> bool {
        self.view.can_go_forward()
    }

    pub fn can_undo(&self) -> bool {
        self.document.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.document.can_redo()
    }

    /// Top-level outline rows: the root's direct children, or just the root
    /// for a document without headings.
    pub fn outline_items(&self) -> Vec<OutlineItem> {
        let root = self
            .document
            .focus_snapshot(self.document.outline().root_id())
            .expect("root always exists");
        if root.children.is_empty() {
            vec![OutlineItem {
                id: root.node_id,
                title: root.title,
                level: root.level,
                child_count: 0,
            }]
        } else {
            root.children
        }
    }

    /// Focuses a section and returns its snapshot.
    pub fn focus(&mut self, id: NodeId) -> Result<FocusSnapshot, DocumentError> {
        let snapshot = self.document.focus_snapshot(id)?;
        self.view.focus(id);
        Ok(snapshot)
    }

    /// Returns to the whole-document outline view.
    pub fn show_outline(&mut self) {
        self.view.show_outline();
    }

    /// Browser-style back; returns the snapshot if a section is now focused.
    pub fn back(&mut self) -> Option<FocusSnapshot> {
        self.view.back()?;
        self.current_snapshot()
    }

    /// Browser-style forward; returns the snapshot if a section is now focused.
    pub fn forward(&mut self) -> Option<FocusSnapshot> {
        self.view.forward()?;
        self.current_snapshot()
    }

    /// Snapshot of the focused section, if the view is in focus mode.
    pub fn current_snapshot(&self) -> Option<FocusSnapshot> {
        let id = self.view.focused()?;
        self.document.focus_snapshot(id).ok()
    }

    /// Replaces the body of the currently focused section. The snapshot the
    /// edit was composed against supplies the revision check (RFC-008).
    pub fn commit_focused_body(
        &mut self,
        base: &FocusSnapshot,
        new_body: String,
    ) -> Result<EditResult, EditError> {
        let result = self.document.replace_section_body(ReplaceSectionBody {
            node_id: base.node_id,
            base_revision: base.revision,
            new_body,
        })?;
        self.prune_dead_history();
        Ok(result)
    }

    /// Undoes the most recent edit (RFC-044) and prunes dead focus targets.
    pub fn undo(&mut self) -> Result<EditResult, EditError> {
        let result = self.document.undo()?;
        self.prune_dead_history();
        Ok(result)
    }

    /// Re-applies the most recently undone edit.
    pub fn redo(&mut self) -> Result<EditResult, EditError> {
        let result = self.document.redo()?;
        self.prune_dead_history();
        Ok(result)
    }

    fn prune_dead_history(&mut self) {
        let outline = self.document.outline();
        self.view.retain_alive(|id| outline.contains(id));
    }
}

impl Default for EditorSession {
    fn default() -> Self {
        Self::new_empty()
    }
}
