<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-014: Basic Keyboard Interaction

**Project:** omriss — Omriss Editor  
**Milestone:** M2 — Basic Desktop UX  
**Status.** Implemented (v0.2.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define the first keyboard contract for editing and navigation.

## 2. Goals

- Support keyboard-first operation for core workflows.
- Define mode-specific behavior.
- Avoid shortcut conflicts with text editing.
- Reserve future shortcuts intentionally.

## 3. Non-Goals

- No full command palette yet.
- No user-customizable shortcuts in M2.
- No Vim/Emacs modal editing mode.

## 4. Design

### Shortcut Table

| Shortcut | Overview | Focus Editor | Raw Source | Notes |
|---|---|---|---|---|
| Enter | Zoom into selected heading | Insert newline in editor | Insert newline if editable | Context-sensitive |
| Esc | Zoom out/root | Zoom out or leave editor focus | Return to structured view | May prompt if local dirty |
| Ctrl/Cmd+O | Open file | Open file | Open file | guarded by dirty state |
| Ctrl/Cmd+S | Save | Commit then save | Save raw source | primary save |
| Ctrl/Cmd+Shift+S | Save As | Save As | Save As | desktop shell |
| Ctrl/Cmd+F | Search current scope | Search current scope | Browser-like find later | M4 expands |
| Ctrl/Cmd+P | Reserved quick switcher | Reserved | Reserved | future command/search |

### Platform Modifier Policy

- macOS uses Cmd for primary app shortcuts.
- Linux/Windows use Ctrl.
- Documentation should display platform-specific notation where possible.

## 5. User Workflow / Interaction Flow

### Keyboard-Only M2 Workflow

```text
Ctrl+O -> choose file
Tab to outline card
Enter -> zoom in
Tab to editor
Type
Ctrl+S -> commit and save
Esc -> zoom out
```

## 6. Validation and Test Plan

- Keyboard-only open/edit/save path.
- Enter does not zoom while typing in textarea.
- Esc behavior is deterministic in each mode.
- Shortcut conflict table reviewed before implementation.

## 7. Acceptance Criteria

- The MVP can be used without a mouse for primary workflows.
- Text editing shortcuts do not unexpectedly trigger navigation.
- Shortcut behavior is documented in user help.

## 8. Dependencies

- RFC-010
- RFC-011
- RFC-012

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
