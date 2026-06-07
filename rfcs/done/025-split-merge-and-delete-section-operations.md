<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-025: Split, Merge, and Delete Section Operations

**Project:** layerd — Layer EDitor  
**Milestone:** M5 — Structural Editing  
**Status.** Implemented (v0.9.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define common restructuring operations beyond heading movement.

## 2. Goals

- Support splitting a body into a new section.
- Support merging adjacent sections.
- Support deleting a section/subtree safely.
- Define confirmation and recovery behavior.

## 3. Non-Goals

- No recycle bin implementation required.
- No cross-document clipboard.
- No rich selection model.

## 4. Design

### Split Section

User chooses a cursor position in the focused body and provides a new heading title/level. Core inserts a heading at that position.

```text
before body part

## New Heading

after body part
```

### Merge Section

MVP merge target: merge current section body with previous or next sibling when safe. Descendant behavior must be explicit in UI.

### Delete Section

Delete removes `full_range` of the target section. Because this may remove descendants, UI must show a confirmation:

```text
Delete “Section 1.2” and its 3 child sections?
[Delete] [Cancel]
```

## 5. Internal Design Notes

### Safety

All destructive structural operations should produce a reversible edit record once undo/redo is implemented. Until then, they require confirmation and are not part of the earliest MVP unless explicitly accepted.

## 6. Validation and Test Plan

- Split preserves text before/after cursor.
- Delete removes subtree range only.
- Delete root rejected.
- Merge rejects non-adjacent incompatible targets.
- Confirmation required for subtree delete.

## 7. Acceptance Criteria

- Operations are specified enough for UI and core implementation.
- Dangerous operations are guarded.
- Source-preservation remains the default even when restructuring.

## 8. Dependencies

- RFC-024
- RFC-026

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
