<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-027: Semantic HTML and ARIA Model

**Project:** omriss — Omriss Editor  
**Milestone:** M6 — Accessibility and Usability Hardening  
**Status.** Implemented (v0.10.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define the semantic DOM and ARIA contracts for Dioxus-rendered UI.

## 2. Goals

- Use semantic HTML regions consistently.
- Define ARIA for outline and tree-like controls.
- Expose current focus and hierarchy depth.
- Create component accessibility responsibilities.

## 3. Non-Goals

- No claim of full screen-reader certification without testing.
- No custom ARIA where native semantics suffice.

## 4. Design

### Region Model

```text
<header> app toolbar
<nav aria-label="Current section path"> breadcrumbs
<aside> outline / local navigator
<main> active workspace
<footer/status> save/index/error status
```

### Outline Semantics

Use native lists/buttons when possible. If tree interaction becomes necessary:

```text
role="tree"
role="treeitem"
aria-level
aria-expanded
aria-current
```

Do not apply tree roles unless keyboard behavior matches expected tree interaction.

### Component Matrix

| Component | Semantic Base | ARIA |
|---|---|---|
| Breadcrumb | nav + ordered list/buttons | `aria-current` on current segment |
| Heading card | button/article summary | label includes level/path |
| Status | status/live region | polite updates |
| Modal | dialog | labelled, focus trapped |

## 5. Validation and Test Plan

- Rendered shell contains landmark regions.
- Current focus is announced.
- Tree roles only appear with matching keyboard support.
- Automated axe-like checks where feasible.

## 6. Acceptance Criteria

- Every interactive component has an accessible name.
- Hierarchy is conveyed beyond visual indentation.
- Accessibility rules are documented for component implementers.

## 7. Dependencies

- RFC-010
- RFC-011
- RFC-013

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
