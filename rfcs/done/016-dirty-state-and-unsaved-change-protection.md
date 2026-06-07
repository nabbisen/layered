<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-016: Dirty State and Unsaved Change Protection

**Project:** layered — Layer EDitor  
**Milestone:** M3 — File Lifecycle and Recovery  
**Status.** Implemented (v0.3.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Prevent accidental loss of work by defining dirty state, local edit state, and close/open guards.

## 2. Goals

- Define document dirty state.
- Define focused local buffer dirty state.
- Guard destructive navigation and file actions.
- Clear dirty state only after successful save.

## 3. Non-Goals

- No autosave in initial implementation.
- No crash recovery journal in M3.
- No background sync.

## 4. Design

### Dirty State Model

```text
DocumentClean: document revision == saved revision and no local dirty buffer
LocalDirty: focused editor buffer differs from FocusSnapshot body
DocumentDirty: core document revision differs from saved revision
SaveFailed: document remains dirty and error is shown
```

### Commit and Save

`Ctrl+S` in focus mode:

```text
if LocalDirty:
  commit section body to core
if commit succeeds:
  save canonical source to disk
if save succeeds:
  saved_revision = document.revision
else:
  keep DocumentDirty and show error
```

### Guards

Warn before:

- closing app;
- opening another file;
- reverting from disk;
- switching to a mode that would discard local buffer, if auto-commit is not possible.

## 5. User Workflow / Interaction Flow

### Unsaved Close Dialog

```text
You have unsaved changes.
[Save] [Discard] [Cancel]
```

If there is a local dirty buffer, Save first commits it, then writes file.

## 6. Validation and Test Plan

- LocalDirty does not mark saved revision until commit.
- Close with unsaved changes prompts.
- Cancel returns to editor with buffer intact.
- Discard clears buffer and reloads saved state.
- Save failure keeps dirty marker.

## 7. Acceptance Criteria

- No user text is discarded without explicit confirmation.
- Dirty indicators are textual, not color-only.
- Dirty state is deterministic across overview/focus/raw modes.

## 8. Dependencies

- RFC-012
- RFC-015

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
