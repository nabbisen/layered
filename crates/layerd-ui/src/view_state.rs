//! Renderer-independent view state for the layer-by-layer workflow.
//!
//! The GUI shows either the whole-document outline or a single focused
//! section ("one layer at a time"). Navigation keeps browser-style
//! back/forward history of focused nodes. None of this touches the
//! document text; it only records *where* the user is looking.

use layerd_core::NodeId;

/// What the main pane is currently showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    /// The whole-document outline (the "map").
    #[default]
    Outline,
    /// One section focused as the current working layer.
    Focus(NodeId),
}

/// Focus location plus browser-style navigation history.
#[derive(Debug, Clone, Default)]
pub struct ViewState {
    mode: ViewMode,
    back: Vec<ViewMode>,
    forward: Vec<ViewMode>,
}

impl ViewState {
    /// Starts at the whole-document outline.
    pub fn new() -> Self {
        Self::default()
    }

    /// The current view.
    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    /// The currently focused node, if any.
    pub fn focused(&self) -> Option<NodeId> {
        match self.mode {
            ViewMode::Focus(id) => Some(id),
            ViewMode::Outline => None,
        }
    }

    /// Whether `back()` would change the view.
    pub fn can_go_back(&self) -> bool {
        !self.back.is_empty()
    }

    /// Whether `forward()` would change the view.
    pub fn can_go_forward(&self) -> bool {
        !self.forward.is_empty()
    }

    /// Focuses `id`, pushing the previous view onto the back history and
    /// clearing the forward history (a new branch, like a browser).
    /// Re-focusing the current node is a no-op.
    pub fn focus(&mut self, id: NodeId) {
        self.navigate(ViewMode::Focus(id));
    }

    /// Returns to the whole-document outline through the same history rules.
    pub fn show_outline(&mut self) {
        self.navigate(ViewMode::Outline);
    }

    /// Steps back in history; returns the new mode, or `None` at the start.
    pub fn back(&mut self) -> Option<ViewMode> {
        let previous = self.back.pop()?;
        self.forward.push(self.mode);
        self.mode = previous;
        Some(self.mode)
    }

    /// Steps forward in history; returns the new mode, or `None` at the end.
    pub fn forward(&mut self) -> Option<ViewMode> {
        let next = self.forward.pop()?;
        self.back.push(self.mode);
        self.mode = next;
        Some(self.mode)
    }

    /// Drops history entries (and the focus itself) that reference nodes no
    /// longer present, e.g. after a structural edit removed a section.
    /// `is_alive` reports whether a node still exists in the outline.
    pub fn retain_alive(&mut self, mut is_alive: impl FnMut(NodeId) -> bool) {
        let alive = |mode: &ViewMode, is_alive: &mut dyn FnMut(NodeId) -> bool| match mode {
            ViewMode::Outline => true,
            ViewMode::Focus(id) => is_alive(*id),
        };
        self.back.retain(|mode| alive(mode, &mut is_alive));
        self.forward.retain(|mode| alive(mode, &mut is_alive));
        if !alive(&self.mode, &mut is_alive) {
            // The focused section vanished: fall back to the outline without
            // polluting history with the dead node.
            self.mode = ViewMode::Outline;
        }
    }

    fn navigate(&mut self, target: ViewMode) {
        if target == self.mode {
            return;
        }
        self.back.push(self.mode);
        self.forward.clear();
        self.mode = target;
    }
}
