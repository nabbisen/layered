<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Added during architecture/design review to close the undo/redo requirements gap
Language: English
-->
# RFC-044: Undo and Redo for Body Edit Operations

**Project:** layerd — Layer EDitor
**Milestone:** M1 — Core Document Engine (engine) / M2 — Basic Desktop UX (shortcuts)
**Status.** Implemented (v0.1.0) — deferred: Ctrl/Cmd+Z/Y keyboard bindings land with RFC-014; toolbar undo/redo shipped
**Document type:** Detailed RFC design
**Primary audience:** Architect, Rust developer, QA engineer

---

## 1. Summary

The app requirements make basic undo/redo an MVP acceptance criterion
(FR-UNDO-001/002) and the roadmap names undo/redo as an RFC-worthy capability,
but no theme in RFC-001..042 designs it. This RFC defines operation-based
undo/redo for committed body edits, built directly on the RFC-008 command model.

## 2. Goals

- Undo/redo committed `ReplaceSectionBody` / `ReplaceRange` operations in `layerd-core`.
- Base history on document edit operations, not UI widget state.
- Keep history consistent with the revision model: undo/redo are themselves revision-incrementing mutations.
- Define interaction with the local focus edit buffer.

## 3. Non-Goals

- No structural-edit undo (promote/move/split/merge); RFC-023..026 must attach
  their own reversible records to this engine when implemented.
- No persistent (cross-session) history.
- No keystroke-granular history inside the local edit buffer; the platform text
  field's native undo covers uncommitted typing.
- No history merging/coalescing heuristics in the first version.

## 4. Design

### History Model

Every successful core mutation records the inverse data needed to reverse it:

```rust
pub struct EditRecord {
    pub replaced_range: ByteRange,   // range in the pre-edit text
    pub old_text: String,            // text that was replaced
    pub new_range: ByteRange,        // range in the post-edit text
    pub new_text: String,            // replacement that was applied
    pub revision_before: DocumentRevision,
    pub revision_after: DocumentRevision,
}

pub struct EditHistory {
    undo_stack: Vec<EditRecord>,
    redo_stack: Vec<EditRecord>,
}
```

### Semantics

```text
apply edit  -> push record on undo stack, clear redo stack
undo        -> replace new_range with old_text, re-index, increment revision,
               move record to redo stack
redo        -> replace replaced_range (recomputed) with new_text, re-index,
               increment revision, move record back to undo stack
```

Undo and redo are ordinary mutations: they re-index and produce a fresh
`DocumentRevision`, so stale focus buffers are detected by the existing
RFC-008 revision check rather than by special cases.

### Granularity

One committed focus-edit equals one history entry. Because typing happens in
the local buffer (RFC-012) and only commits at save/blur/focus-transition,
history entries are naturally section-sized and meaningful to the user.

### UI Contract

- `Ctrl/Cmd+Z` undo, `Ctrl/Cmd+Y` and `Ctrl/Cmd+Shift+Z` redo (RFC-014 table extends).
- While the focus editor holds uncommitted local changes, the shortcut first
  applies the text field's native undo; document-level undo applies to
  committed history only. The status region announces which level acted.
- After undo/redo, the UI refreshes the focus snapshot; if the focused node no
  longer exists, navigate to the nearest surviving ancestor (RFC-006 stale rule).

### Capacity

History is bounded (default 100 entries, oldest dropped first) to keep memory
predictable for long sessions; the bound is a core constant until settings need it.

## 5. Internal Design Notes

- `EditRecord` stores both directions explicitly instead of recomputing inverses,
  trading bytes for simplicity and testability.
- Source-preservation invariant applies to undo/redo themselves: reversing an
  edit must restore the prior text byte-for-byte, verifiable by golden tests.

## 6. Validation and Test Plan

- apply → undo restores the exact pre-edit source (byte-identical).
- apply → undo → redo restores the exact post-edit source (byte-identical).
- New edit after undo clears the redo stack.
- Undo on empty history is a no-op error (`EditError::NothingToUndo`).
- Undo across a re-index keeps unrelated sections byte-identical.
- History bound drops oldest entries without corrupting the stacks.

## 7. Acceptance Criteria

- Committed body edits are undoable and redoable through the public core API.
- Undo/redo round-trips are byte-exact in golden tests.
- Revision semantics hold: every undo/redo yields a new revision.
- The engine exposes hooks that structural-edit RFCs can attach records to later.

## 8. Dependencies

- RFC-002
- RFC-004
- RFC-008
- RFC-012
- RFC-014

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
