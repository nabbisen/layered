<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-013: Breadcrumb and Hierarchical Navigation

**Project:** layerd — Layer EDitor  
**Milestone:** M2 — Basic Desktop UX  
**Status.** Proposed  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define breadcrumbs and hierarchy-aware outward traversal.

## 2. Goals

- Display the path from Root to current focus.
- Allow clicking breadcrumb segments.
- Define overflow behavior.
- Keep breadcrumb accurate after heading edits and re-index.

## 3. Non-Goals

- No persistent navigation sidebar history.
- No URL routing contract.
- No cross-document breadcrumb.

## 4. Design

### Breadcrumb Wireframe

```text
Root  >  Chapter 1  >  Section 1.2  >  Assumptions
```

For narrow layouts:

```text
Root > … > Section 1.2 > Assumptions
```

### Behavior

- Each segment is a button/link-like control.
- Selecting a segment changes `current_focus_id`.
- Root segment enters root overview.
- If title changes, breadcrumb updates after re-index.
- If a segment disappears, UI navigates to nearest surviving ancestor or root.

### Accessibility

Breadcrumb container uses navigation semantics and an accessible label such as “Current section path”. Current item is marked as current location.

## 5. Internal Design Notes

### Source of Truth

Use `FocusSnapshot.path` from core. UI must not recompute parent chains from raw maps.

## 6. Validation and Test Plan

- Breadcrumb click navigates to ancestor.
- Long path collapses without hiding current section.
- Deleted ancestor recovery goes to root or nearest surviving parent.
- Keyboard tab order reaches every visible breadcrumb segment.

## 7. Acceptance Criteria

- The user can always understand current depth.
- The user can zoom out without using the sidebar.
- Breadcrumb remains usable with duplicate titles by preserving path order.

## 8. Dependencies

- RFC-005
- RFC-012

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
