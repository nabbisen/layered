<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-020: Sibling and Depth Navigation

**Project:** layered — Layer EDitor  
**Milestone:** M4 — Navigation and Search  
**Status.** Implemented (v0.8.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Support fast movement across adjacent sections and hierarchy levels without returning to full overview.

## 2. Goals

- Define previous/next section operations.
- Define parent/first-child zoom operations.
- Expose available navigation affordances.
- Handle edge cases at root and leaves.

## 3. Non-Goals

- No custom keybinding UI.
- No drag movement.
- No semantic document navigation beyond heading tree.

## 4. Design

### Navigation Definitions

| Operation | Definition |
|---|---|
| Parent | current node parent, or root if available |
| First child | first child in source order |
| Previous sibling | previous child of same parent |
| Next sibling | next child of same parent |
| Previous section | source-order previous node, optional future command |
| Next section | source-order next node, optional future command |

### UI Affordance

```text
[← Previous] [↑ Parent] [↓ First Child] [Next →]
```

Disabled actions remain visible with textual disabled reason for accessibility.

## 5. Validation and Test Plan

- Root has no parent action.
- Leaf has no first-child action.
- Sibling order follows source order.
- Disabled controls are not focus traps.

## 6. Acceptance Criteria

- User can traverse nearby sections quickly.
- Depth and sibling operations are distinct.
- Keyboard and visible controls stay consistent.

## 7. Dependencies

- RFC-007
- RFC-019

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
