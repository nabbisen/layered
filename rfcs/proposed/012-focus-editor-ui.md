<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-012: Focus Editor UI

**Project:** layerd — Layer EDitor  
**Milestone:** M2 — Basic Desktop UX  
**Status.** Proposed  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Design the focused section editing page where the user edits one layer while seeing immediate child headings.

## 2. Goals

- Show current heading, body editor, and immediate children.
- Use local edit buffering to avoid full document mutation on every keystroke.
- Define commit points.
- Handle section disappearance after re-index.

## 3. Non-Goals

- No WYSIWYG editor.
- No Markdown preview in M2.
- No simultaneous multi-section editing.

## 4. Design

### Focus Page Wireframe

```text
+--------------------------------------------------------------------------------+
| Root > Chapter 1 > Section 1.2                                                  |
+--------------------------------------------------------------------------------+
| H2 Section 1.2: Problem Framing                                                 |
|                                                                                |
| Body                                                                           |
| +----------------------------------------------------------------------------+ |
| | The current focused section body appears here as plain Markdown text.       | |
| |                                                                            | |
| | The editor does not show siblings or parent bodies.                         | |
| +----------------------------------------------------------------------------+ |
|                                                                                |
| Immediate Child Layers                                                         |
|   [H3] Assumptions                         [Zoom In]                           |
|   [H3] Constraints                         [Zoom In]                           |
|                                                                                |
| Status: local edit buffer · Ctrl+S save · Esc zoom out                          |
+--------------------------------------------------------------------------------+
```

### Commit Lifecycle

```text
FocusSnapshot loaded -> local body buffer created
User types -> local buffer changes only
Blur / Ctrl+S / focus transition / debounce -> commit ReplaceSectionBody
Commit success -> refresh FocusSnapshot
Commit failure -> keep local buffer and show recovery action
```

### Editor Element

M2 should use a plain `<textarea>`-like editing surface through Dioxus-rendered DOM. The editor must preserve literal Markdown input.

## 5. Internal Design Notes

### State Separation

```text
Global Signal: current document, current focus id, dirty state
Local Component State: focused body buffer, base revision, local dirty flag
```

Typing must not mutate the full document on each keystroke.

## 6. Validation and Test Plan

- Typing updates local buffer without changing document revision until commit.
- Ctrl+S commits local buffer then saves.
- Esc with local dirty buffer commits or prompts according to RFC-016.
- Child heading card zooms into child.

## 7. Acceptance Criteria

- A user can edit a section body and preserve child/sibling content.
- The focus page communicates the current layer and immediate sub-layers.
- Local edit state is recoverable after commit failure.

## 8. Dependencies

- RFC-004
- RFC-005
- RFC-010
- RFC-011

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
