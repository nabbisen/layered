<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-028: Keyboard Focus and Navigation Accessibility

**Project:** omriss — Omriss Editor  
**Milestone:** M6 — Accessibility and Usability Hardening  
**Status.** Implemented (v0.10.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Ensure zoom navigation and dialogs remain understandable without a mouse.

## 2. Goals

- Define focus transitions after zoom and modal actions.
- Support keyboard-only workflows.
- Define roving tabindex if used.
- Prevent focus loss after re-render.

## 3. Non-Goals

- No custom screen-reader mode.
- No advanced shortcut customization.

## 4. Design

### Focus Transition Rules

| Action | New Focus |
|---|---|
| Zoom into heading card | focused section title or body editor, depending user intent |
| Zoom out with Esc | card/heading representing previous section |
| Breadcrumb jump | target section title |
| Search result select | match context or focused editor |
| Open modal | first safe action or dialog title |
| Close modal | invoking control |

### Roving Tabindex

If overview cards are presented as a navigable collection, use one tab stop for the collection and arrow keys inside it. Otherwise, each card may be a normal button for simpler M2 behavior.

## 5. User Workflow / Interaction Flow

### Keyboard-Only Workflow Map

```text
Open -> Tab to outline -> Enter zoom -> Tab to editor -> type -> Ctrl+S -> Esc -> arrow to sibling -> Enter
```

## 6. Validation and Test Plan

- Focus target after zoom-in is deterministic.
- Esc returns focus to previous card.
- Modal traps focus and returns it.
- No keyboard trap in editor or outline.

## 7. Acceptance Criteria

- Primary workflows work with keyboard only.
- Focus is never lost to document body after re-render.
- Destructive confirmations are accessible.

## 8. Dependencies

- RFC-014
- RFC-027

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
