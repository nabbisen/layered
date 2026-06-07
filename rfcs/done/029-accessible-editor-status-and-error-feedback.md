<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-029: Accessible Editor Status and Error Feedback

**Project:** layerd — Layer EDitor  
**Milestone:** M6 — Accessibility and Usability Hardening  
**Status.** Implemented (v0.10.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Make save status, dirty state, validation errors, and indexing problems perceivable.

## 2. Goals

- Define status and live-region policy.
- Avoid color-only state communication.
- Tie errors to controls where possible.
- Make long operations perceivable.

## 3. Non-Goals

- No localization framework.
- No telemetry reporting.
- No voice-specific UX.

## 4. Design

### Status Types

```text
Saved
Unsaved changes
Saving...
Save failed
Outline refreshed
Index warning
Command unavailable
```

### Live Region Policy

- Save success: polite status.
- Save failure: assertive or dialog depending severity.
- Dirty state: visible text marker; not repeatedly announced on every keystroke.
- Parse/index warning: persistent visible status with action to show details/raw source.

### Error Pattern

```text
Problem: Save failed.
Cause: Permission denied.
Action: Choose another location or check file permissions.
[Save As] [Dismiss]
```

## 5. Validation and Test Plan

- Dirty state visible without color.
- Save success announced once.
- Save failure includes recovery action.
- Validation errors reference related control.

## 6. Acceptance Criteria

- Users can perceive important state changes without sight.
- Error messages are actionable and non-technical where possible.
- Status channel does not spam screen-reader users during typing.

## 7. Dependencies

- RFC-009
- RFC-016
- RFC-027

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
