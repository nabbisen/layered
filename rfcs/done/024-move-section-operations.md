<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-024: Move Section Operations

**Project:** layered — Layer EDitor  
**Milestone:** M5 — Structural Editing  
**Status.** Implemented (v0.9.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define safe movement of whole section subtrees within the Markdown source.

## 2. Goals

- Move a section with all child sections.
- Define target positions.
- Reject impossible/cyclic moves.
- Preserve section bytes as much as possible.

## 3. Non-Goals

- No mouse drag required in first implementation.
- No cross-document move.
- No semantic merge of headings.

## 4. Design

### Move Range

Moving a section moves `full_range`, including:

```text
heading line
body
all descendant sections
trailing whitespace belonging to the section, according to range policy
```

### Target Position Model

```rust
pub enum MoveTarget {
    Before(NodeId),
    After(NodeId),
    AsFirstChildOf(NodeId),
    AsLastChildOf(NodeId),
}
```

### Invalid Moves

- move root;
- move a section into its own descendant;
- move using stale node ID;
- move to a target whose range overlaps the moved range incorrectly.

## 5. Internal Design Notes

### Algorithm

```text
1. resolve source full range and target insertion point
2. validate target after removing source range virtually
3. extract exact source bytes
4. remove source range
5. insert bytes at adjusted insertion point
6. re-index and validate
```

Blank line normalization is deferred. M5 should preserve moved bytes and avoid aesthetic formatting changes.

## 6. Validation and Test Plan

- Move before sibling.
- Move after sibling.
- Move as last child.
- Move into descendant rejected.
- Moved subtree bytes preserved exactly.

## 7. Acceptance Criteria

- Structural movement does not lose descendants.
- Invalid moves are rejected before mutation.
- User-facing preview can be built from operation model.

## 8. Dependencies

- RFC-008
- RFC-026

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
