<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-010: Desktop Application Shell

**Project:** omriss — Omriss Editor  
**Milestone:** M2 — Basic Desktop UX  
**Status.** Implemented (v0.2.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define the main application shell and state-dependent layout for the desktop app.

## 2. Goals

- Provide a stable whole-app layout.
- Define no-document, loaded-document, dirty, and error states.
- Place primary file and navigation actions.
- Support keyboard and screen-reader users from shell level.

## 3. Non-Goals

- No final visual theme.
- No full settings UI.
- No multi-window support.

## 4. Design

### Whole-App Layout

```text
+--------------------------------------------------------------------------------+
| omriss        [Open] [Save] [Save As]           document.md *        [Search]   |
+--------------------------------------------------------------------------------+
| Breadcrumb / Focus Path:  Root > Chapter 1 > Section 1.2                        |
+------------------------------+-------------------------------------------------+
| Outline / Local Navigator    | Main Workspace                                  |
|                              |                                                 |
|  Root                        |  Focus / Overview / Raw Source                  |
|  ├─ Chapter 1                |                                                 |
|  │  ├─ Section 1.1           |                                                 |
|  │  └─ Section 1.2  ◀        |                                                 |
|  └─ Chapter 2                |                                                 |
+------------------------------+-------------------------------------------------+
| Status: Saved / Unsaved / Index warning / Error details                         |
+--------------------------------------------------------------------------------+
```

### Shell States

| State | Main Workspace | Header | Status |
|---|---|---|---|
| No document | Welcome/open screen | Open only | Ready |
| Loaded clean | Overview or focus | file name | Saved |
| Loaded dirty | current mode | `*` marker | Unsaved changes |
| Index warning | raw/focus with warning | warning icon/text | Outline needs attention |
| Save error | current mode | dirty marker remains | Save failed |

## 5. User Workflow / Interaction Flow

### No-Document Workflow

```text
Open omriss
  -> Welcome screen
  -> Open Markdown
  -> Parse and index
  -> Overview mode
```

### Loaded Workflow

```text
Open file
  -> Overview
  -> Select heading
  -> Focus editor
  -> Edit body
  -> Save
```

## 6. Validation and Test Plan

- Shell renders no-document state.
- Dirty marker appears after edit.
- Status area is visible and text-based.
- Open/save commands are keyboard-accessible.

## 7. Acceptance Criteria

- A user can understand where file actions, structure, content, and status live.
- Shell can host overview, focus, raw source, search, and error views.
- No document content is hidden behind hover-only UI.

## 8. Dependencies

- RFC-001
- RFC-005

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
