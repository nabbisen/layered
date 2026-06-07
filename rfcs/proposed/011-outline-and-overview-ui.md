<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-011: Outline and Overview UI

**Project:** layerd — Layer EDitor  
**Milestone:** M2 — Basic Desktop UX  
**Status.** Proposed  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Design the overview mode that presents Markdown headings as navigable thought layers.

## 2. Goals

- Show document structure as clickable/keyboard-selectable heading cards.
- Make heading levels visually and semantically clear.
- Represent empty and headingless documents.
- Support zoom-in from overview.

## 3. Non-Goals

- No drag-and-drop movement in M2.
- No graph/canvas view.
- No rich preview.

## 4. Design

### Overview Wireframe

```text
+-------------------------------------------------------------+
| Root Overview                                                |
| Document: research-notes.md                                  |
|                                                             |
| [H1] Introduction                                            |
|      3 child sections · 428 words                            |
|                                                             |
| [H1] Prior Work                                              |
|      5 child sections · 1,240 words                          |
|                                                             |
| [H1] Proposed Method                                         |
|      4 child sections · 980 words                            |
|                                                             |
| Enter: Zoom in   ↑/↓: Move selection   Esc: Root             |
+-------------------------------------------------------------+
```

### Heading Card Fields

```text
title
heading level
optional child count
optional body word count
selection/focus state
warnings, if section cannot be indexed cleanly
```

### Interaction

- Click or Enter zooms into a heading.
- Arrow keys move card selection.
- Cards expose level and position to assistive technology.
- Empty documents show a starter action: “Create first heading”.

## 5. Internal Design Notes

### UI Data Source

Use `OutlineItem` from `layerd-core`; do not inspect source text in the component. Word counts are optional derived UI data and must not affect core correctness.

## 6. Validation and Test Plan

- Keyboard selection reaches every visible card.
- Duplicate heading titles are distinguishable by position/path.
- Headingless document empty state appears.
- Overview card exposes accessible name.

## 7. Acceptance Criteria

- Overview mode allows zooming into any visible top-level or local child heading.
- Hierarchy is shown by text/indentation, not color alone.
- The UI does not require a mouse.

## 8. Dependencies

- RFC-007
- RFC-010

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
