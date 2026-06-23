//! Structural-editing methods on [`super::EditorSession`] (RFC-023..026).

use layered_core::EditResult;

impl super::EditorSession {
    // ── Structural editing façade (RFC-023..026) ─────────────────────────────

    /// Whether the focused section can be promoted (not H1, not setext).
    pub fn can_promote(&self) -> bool {
        self.view
            .focused()
            .and_then(|id| self.document.outline().node(id))
            .and_then(|n| n.level)
            .map(|l| l != layered_core::HeadingLevel::H1)
            .unwrap_or(false)
    }

    /// Whether the focused section can be demoted (not H6, not setext).
    pub fn can_demote(&self) -> bool {
        self.view
            .focused()
            .and_then(|id| self.document.outline().node(id))
            .and_then(|n| n.level)
            .map(|l| l != layered_core::HeadingLevel::H6)
            .unwrap_or(false)
    }

    /// Whether the focused section can be deleted (not root).
    pub fn can_delete(&self) -> bool {
        self.view
            .focused()
            .and_then(|id| self.document.outline().node(id))
            .map(|n| !n.is_root())
            .unwrap_or(false)
    }

    /// Whether the focused section can be merged with the previous sibling.
    pub fn can_merge_up(&self) -> bool {
        let Some(id) = self.view.focused() else {
            return false;
        };
        let info = crate::navigation::sibling_info(self.document.outline(), id);
        info.prev_sibling.is_some()
    }

    /// Promotes the focused section heading one level (RFC-023).
    pub fn promote_focused(&mut self) -> Result<EditResult, layered_core::StructuralEditError> {
        let id = self
            .view
            .focused()
            .ok_or(layered_core::StructuralEditError::CannotDeleteRoot)?;
        let rev = self.document.revision();
        let result = self.document.promote_section(id, rev)?;
        let stale = self.prune_dead_history();
        if stale {
            self.view.show_outline();
        }
        Ok(result)
    }

    /// Demotes the focused section heading one level (RFC-023).
    pub fn demote_focused(&mut self) -> Result<EditResult, layered_core::StructuralEditError> {
        let id = self
            .view
            .focused()
            .ok_or(layered_core::StructuralEditError::CannotDeleteRoot)?;
        let rev = self.document.revision();
        let result = self.document.demote_section(id, rev)?;
        self.prune_dead_history();
        Ok(result)
    }

    /// Moves the focused section to the given target (RFC-024).
    pub fn move_focused(
        &mut self,
        target: layered_core::MoveTarget,
    ) -> Result<EditResult, layered_core::StructuralEditError> {
        let id = self
            .view
            .focused()
            .ok_or(layered_core::StructuralEditError::StaleNode(
                self.document.outline().root_id(),
            ))?;
        let rev = self.document.revision();
        let result = self.document.move_section(id, target, rev)?;
        self.prune_dead_history();
        Ok(result)
    }

    /// Splits the focused section body at `offset_in_body` bytes (RFC-025).
    pub fn split_focused(
        &mut self,
        offset_in_body: usize,
        new_title: &str,
        new_level: layered_core::HeadingLevel,
    ) -> Result<EditResult, layered_core::StructuralEditError> {
        let id = self
            .view
            .focused()
            .ok_or(layered_core::StructuralEditError::CannotDeleteRoot)?;
        let rev = self.document.revision();
        self.document
            .split_section(id, offset_in_body, new_title, new_level, rev)
    }

    /// Deletes the focused section and its subtree (RFC-025).
    /// The UI must confirm with the user before calling.
    pub fn delete_focused(&mut self) -> Result<EditResult, layered_core::StructuralEditError> {
        let id = self
            .view
            .focused()
            .ok_or(layered_core::StructuralEditError::CannotDeleteRoot)?;
        let rev = self.document.revision();
        let result = self.document.delete_section(id, rev)?;
        self.view.show_outline();
        self.prune_dead_history();
        Ok(result)
    }

    /// Merges the focused section with its previous sibling (RFC-025).
    pub fn merge_focused_up(&mut self) -> Result<EditResult, layered_core::StructuralEditError> {
        let id = self
            .view
            .focused()
            .ok_or(layered_core::StructuralEditError::NoAdjacentSibling)?;
        let rev = self.document.revision();
        let result = self.document.merge_with_prev_sibling(id, rev)?;
        self.prune_dead_history();
        Ok(result)
    }

    /// Move the focused section up one position among its siblings (RFC-024).
    pub fn move_focused_up(&mut self) -> Result<EditResult, layered_core::StructuralEditError> {
        let id = self
            .view
            .focused()
            .ok_or(layered_core::StructuralEditError::NoAdjacentSibling)?;
        let info = crate::navigation::sibling_info(self.document.outline(), id);
        let prev = info
            .prev_sibling
            .ok_or(layered_core::StructuralEditError::NoAdjacentSibling)?;
        self.move_focused(layered_core::MoveTarget::Before(prev))
    }

    /// Move the focused section down one position among its siblings (RFC-024).
    pub fn move_focused_down(&mut self) -> Result<EditResult, layered_core::StructuralEditError> {
        let id = self
            .view
            .focused()
            .ok_or(layered_core::StructuralEditError::NoAdjacentSibling)?;
        let info = crate::navigation::sibling_info(self.document.outline(), id);
        let next = info
            .next_sibling
            .ok_or(layered_core::StructuralEditError::NoAdjacentSibling)?;
        self.move_focused(layered_core::MoveTarget::After(next))
    }

    /// Whether the focused section can be moved up (has a previous sibling).
    pub fn can_move_up(&self) -> bool {
        self.view
            .focused()
            .map(|id| {
                crate::navigation::sibling_info(self.document.outline(), id)
                    .prev_sibling
                    .is_some()
            })
            .unwrap_or(false)
    }

    /// Whether the focused section can be moved down (has a next sibling).
    pub fn can_move_down(&self) -> bool {
        self.view
            .focused()
            .map(|id| {
                crate::navigation::sibling_info(self.document.outline(), id)
                    .next_sibling
                    .is_some()
            })
            .unwrap_or(false)
    }
}
