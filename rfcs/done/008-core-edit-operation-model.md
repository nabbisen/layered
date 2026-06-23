<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-008: Core Edit Operation Model

**Project:** omriss — Omriss Editor  
**Milestone:** M1 — Core Document Engine  
**Status.** Implemented (v0.1.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define primitive edit commands as the foundation for undo/redo and structural editing.

## 2. Goals

- Model text replacement as an explicit command.
- Validate revision and range before mutation.
- Define which operations trigger re-indexing.
- Create a consistent `EditResult`.

## 3. Non-Goals

- No full undo stack yet.
- No structural operations yet.
- No collaborative conflict resolution.

## 4. Design

### Command Model

```rust
pub enum EditCommand {
    ReplaceSectionBody(ReplaceSectionBody),
    ReplaceRange(ReplaceRange), // internal or restricted until raw source editing RFC
}
```

Command fields include:

```text
base_revision
operation target
replacement text
user-visible intent label
```

### Validation Sequence

```text
1. check base revision
2. resolve target node/range
3. validate UTF-8 boundaries
4. apply replacement to canonical text
5. rebuild outline
6. validate outline invariants
7. increment revision
8. return EditResult
```

### Re-index Trigger Policy

All M1 core mutations re-index synchronously after commit. Debounce and local edit buffers belong to UI/performance RFCs.

## 5. Internal Design Notes

### Transaction Shape

M1 may implement simple transaction logic:

```text
clone old text metadata
apply replacement
attempt re-index
if invariant failure is fatal, rollback text
```

Because Markdown text is permissive, parse failures should be rare; range and internal invariant failures are more important.

## 6. Validation and Test Plan

- Revision mismatch.
- Invalid range.
- Successful replacement increments revision.
- Invariant validation after replacement.
- Operation result includes old/new ranges.

## 7. Acceptance Criteria

- All document mutations enter through command methods.
- Edit operations never panic for user-provided document text.
- The model can later attach undo records to every command.

## 8. Dependencies

- RFC-004
- RFC-005
- RFC-006

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
