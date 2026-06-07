//! Editor session: one open document plus everything the GUI needs around it.
//!
//! `EditorSession` glues `layerd_core::Document` to the view state: it owns
//! dirty tracking (content-based, so undoing back to the saved bytes clears
//! the flag), routes focused-body edits through the core command API, and
//! prunes navigation history when sections disappear after structural edits.
//! It also stores the `FileTextProfile` detected when the file was opened
//! (line endings, BOM, trailing newline — RFC-018).

use std::hash::{DefaultHasher, Hash, Hasher};

use layerd_core::{
    Document, DocumentError, DocumentRevision, EditError, EditResult, FocusSnapshot, NodeId,
    OutlineItem, ReplaceSectionBody,
};

use crate::file_profile::FileTextProfile;
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
    /// File-level text characteristics detected at open time (RFC-018).
    profile: FileTextProfile,
}

impl EditorSession {
    /// Opens a session over Markdown text (e.g. file contents just read).
    pub fn open(markdown: String, file_name: Option<String>) -> Result<Self, DocumentError> {
        let profile = FileTextProfile::detect(&markdown, false);
        let document = Document::parse(markdown)?;
        let saved = SavedFingerprint::of(document.source(), document.revision());
        Ok(Self {
            document,
            view: ViewState::new(),
            saved,
            file_name,
            profile,
        })
    }

    /// Opens a session with a pre-computed file text profile (RFC-018). Use
    /// this when the desktop crate has already stripped a BOM or detected
    /// line endings before passing the text here.
    pub fn open_with_profile(
        markdown: String,
        file_name: Option<String>,
        profile: FileTextProfile,
    ) -> Result<Self, DocumentError> {
        let document = Document::parse(markdown)?;
        let saved = SavedFingerprint::of(document.source(), document.revision());
        Ok(Self {
            document,
            view: ViewState::new(),
            saved,
            file_name,
            profile,
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

    /// File text profile detected at open time (line endings, BOM — RFC-018).
    pub fn profile(&self) -> &FileTextProfile {
        &self.profile
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

    /// Current view (outline, focused section, or raw-source overlay).
    pub fn view_mode(&self) -> ViewMode {
        self.view.mode()
    }

    /// Whether the raw-source overlay is currently active (RFC-017).
    pub fn is_raw(&self) -> bool {
        self.view.is_raw()
    }

    /// Enters the read-only raw-source overlay; returns to structured view via
    /// `leave_raw()`. Does not push a navigation history entry.
    pub fn show_raw(&mut self) {
        self.view.show_raw();
    }

    /// Leaves the raw-source overlay, returning to the mode that was active.
    pub fn leave_raw(&mut self) {
        self.view.leave_raw();
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

    /// Outline items relevant to the current view:
    /// - In overview mode: the top-level items (same as [`outline_items`]).
    /// - In focus mode: the immediate children of the focused section.
    ///
    /// Used to populate the left outline pane and the card list for keyboard
    /// selection (RFC-011, RFC-014).
    pub fn current_children(&self) -> Vec<OutlineItem> {
        match self.view.focused() {
            Some(_) => self
                .current_snapshot()
                .map(|s| s.children)
                .unwrap_or_default(),
            None => self.outline_items(),
        }
    }

    /// Zooms out one level: moves focus to the parent section, or returns to
    /// the outline overview if the current focus is a direct child of the root.
    ///
    /// This is the structural "Esc" action (RFC-014). It pushes the current
    /// node onto the back history (via `ViewState::focus`/`show_outline`) so
    /// the user can return forward with Alt+Right.
    pub fn zoom_out(&mut self) {
        let Some(id) = self.view.focused() else {
            return;
        };
        let parent_id = self.document.outline().node(id).and_then(|n| n.parent_id);
        match parent_id {
            Some(pid) if pid != self.document.outline().root_id() => {
                self.view.focus(pid);
            }
            _ => {
                self.view.show_outline();
            }
        }
        self.prune_dead_history();
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

    fn prune_dead_history(&mut self) -> bool {
        let mode_before = self.view.mode();
        let outline = self.document.outline();
        self.view.retain_alive(|id| outline.contains(id));
        // If the current mode changed, a stale node was pruned.
        self.view.mode() != mode_before
    }

    /// Prunes dead history and returns `true` if a stale focus target was
    /// encountered (the UI can then show a non-blocking status message per
    /// RFC-019).
    pub fn prune_and_report(&mut self) -> bool {
        self.prune_dead_history()
    }

    /// Navigation availability from the currently focused node (RFC-020).
    pub fn sibling_info(&self) -> crate::navigation::SiblingInfo {
        match self.view.focused() {
            Some(id) => crate::navigation::sibling_info(self.document.outline(), id),
            None => crate::navigation::SiblingInfo::default(),
        }
    }

    /// Navigates to the parent section (or overview if parent is root).
    /// Returns `true` if navigation happened.
    pub fn navigate_parent(&mut self) -> bool {
        let Some(id) = self.view.focused() else {
            return false;
        };
        let outline = self.document.outline();
        let Some(node) = outline.node(id) else {
            return false;
        };
        match node.parent_id {
            Some(pid) if pid != outline.root_id() => {
                self.view.focus(pid);
                true
            }
            Some(_) => {
                self.view.show_outline();
                true
            }
            None => false,
        }
    }

    /// Navigates to the first child of the focused section.
    pub fn navigate_first_child(&mut self) -> bool {
        let Some(id) = self.view.focused() else {
            return false;
        };
        let outline = self.document.outline();
        let Some(node) = outline.node(id) else {
            return false;
        };
        let Some(&child_id) = node.children.first() else {
            return false;
        };
        self.view.focus(child_id);
        true
    }

    /// Navigates to the previous sibling in source order.
    pub fn navigate_prev_sibling(&mut self) -> bool {
        let Some(id) = self.view.focused() else {
            return false;
        };
        let info = crate::navigation::sibling_info(self.document.outline(), id);
        if let Some(prev) = info.prev_sibling {
            self.view.focus(prev);
            true
        } else {
            false
        }
    }

    /// Navigates to the next sibling in source order.
    pub fn navigate_next_sibling(&mut self) -> bool {
        let Some(id) = self.view.focused() else {
            return false;
        };
        let info = crate::navigation::sibling_info(self.document.outline(), id);
        if let Some(next) = info.next_sibling {
            self.view.focus(next);
            true
        } else {
            false
        }
    }

    /// Searches the whole document for `query` (RFC-021).
    pub fn search_document(&self, query: &str) -> Vec<crate::search::SearchMatch> {
        crate::search::search_document(&self.document, query)
    }

    /// Searches only the focused section's body for `query`.
    pub fn search_section(&self, query: &str) -> Vec<crate::search::SearchMatch> {
        match self.view.focused() {
            Some(id) => crate::search::search_section(&self.document, id, query),
            None => crate::search::search_document(&self.document, query),
        }
    }
}

impl Default for EditorSession {
    fn default() -> Self {
        Self::new_empty()
    }
}
