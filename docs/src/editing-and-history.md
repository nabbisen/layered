# Editing and History

## Committing a body edit

Edits are explicit: refine the focused section's body, then commit. Each
commit carries the document revision your draft was based on. If the document
changed underneath (for example, an undo elsewhere), the stale commit is
rejected before anything mutates, and your text stays in the editor.

## Structural operations

When a section is focused, the **structural toolbar** below the editor offers
promote, demote, move up, move down, merge up, add child section, and delete.
These reorganise the heading hierarchy without touching body text. Every
structural operation is recorded in the same undo history as body edits — see
[Structural Editing](structural-editing.md) for details.

## Undo and redo

Undo restores the previous text **byte-exactly**, and redo re-applies the
edit byte-exactly. History is bounded (100 entries) and survives structural
changes: undoing an edit that introduced new headings also retracts those
sections from the outline.

## Unsaved changes

The status bar shows when the text differs from what is on disk. Undoing back
to the exact saved bytes clears the indicator — omriss compares content, not
edit counts.
