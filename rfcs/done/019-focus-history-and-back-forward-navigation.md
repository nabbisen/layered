<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-019: Focus History and Back/Forward Navigation

**Project:** layered — Layer EDitor  
**Milestone:** M4 — Navigation and Search  
**Status.** Implemented (v0.8.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define reversible navigation history for zoom focus changes.

## 2. Goals

- Track focus navigation history.
- Support back and forward actions.
- Handle stale nodes after edits.
- Define persistence policy.

## 3. Non-Goals

- No browser URL history.
- No cross-file history in M4.
- No session restoration guarantee.

## 4. Design

### Model

```rust
pub struct FocusHistory {
    back: Vec<NodeId>,
    current: NodeId,
    forward: Vec<NodeId>,
}
```

Events that push history:

- zoom into heading;
- breadcrumb jump;
- search result selection;
- sibling/depth navigation.

Events that should not push duplicate history:

- refresh current focus after re-index;
- editing body without focus change.

### Stale Node Handling

If history target no longer exists:

```text
try nearest surviving ancestor
else root
show non-blocking status: Previous section no longer exists.
```

## 5. Validation and Test Plan

- Back returns to previous focus.
- Forward restored after back.
- New navigation clears forward stack.
- Stale history target recovers to ancestor/root.

## 6. Acceptance Criteria

- Users can recover from accidental zoom jumps.
- History behavior is predictable and documented.
- History does not corrupt current document state.

## 7. Dependencies

- RFC-013

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
