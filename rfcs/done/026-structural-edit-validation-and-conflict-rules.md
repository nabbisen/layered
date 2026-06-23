<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-026: Structural Edit Validation and Conflict Rules

**Project:** omriss — Omriss Editor  
**Milestone:** M5 — Structural Editing  
**Status.** Implemented (v0.9.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define shared validation rules for structural edits.

## 2. Goals

- Centralize structural edit preflight checks.
- Handle stale IDs and uncommitted local edits.
- Define user-facing conflict messages.
- Prevent corrupt outline states.

## 3. Non-Goals

- No concurrent multi-user editing.
- No automatic conflict merge.
- No external file merge tool integration.

## 4. Design

### Preflight Checks

```text
1. no uncommitted local body buffer, or commit first
2. source node exists
3. target node exists where applicable
4. operation-specific range relationships are valid
5. resulting heading level is in H1..H6
6. predicted operation does not create impossible containment
```

### Conflict Taxonomy

| Conflict | Example | Resolution |
|---|---|---|
| LocalDirty | focused body has unsaved local edits | commit/save/cancel first |
| StaleNode | section changed after re-index | refresh view |
| InvalidTarget | move into descendant | choose another target |
| InvalidLevel | promote H1 | command disabled/explain |
| IndexFailure | result cannot be indexed | rollback or raw recovery |

## 5. Internal Design Notes

### Validation API

```rust
pub fn validate_structural_edit(
    document: &Document,
    command: &StructuralCommand,
) -> Result<ValidatedStructuralEdit, StructuralEditError>;
```

Validated commands carry resolved ranges to avoid time-of-check/time-of-use ambiguity inside a single synchronous operation.

## 6. Validation and Test Plan

- Each conflict type has a fixture.
- Commands disabled where validation would fail.
- Validated edit ranges match current revision.
- Uncommitted local edit guard is enforced by UI.

## 7. Acceptance Criteria

- All structural RFCs use the same validation vocabulary.
- User messages are actionable.
- Invalid structural edits never partially mutate the document.

## 8. Dependencies

- RFC-008
- RFC-016

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
