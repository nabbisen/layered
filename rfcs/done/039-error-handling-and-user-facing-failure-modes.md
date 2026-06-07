<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-039: Error Handling and User-Facing Failure Modes

**Project:** layerd — Layer EDitor  
**Milestone:** M9 — Production Readiness  
**Status.** Implemented (v0.11.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Make failures understandable, recoverable, and safe for users.

## 2. Goals

- Catalog likely failure modes.
- Define message and recovery patterns.
- Avoid exposing frightening internals unnecessarily.
- Keep logs safe and local.

## 3. Non-Goals

- No telemetry.
- No crash reporting service.
- No localization in initial version.

## 4. Design

### Failure Catalog

| Area | Failure | Recovery |
|---|---|---|
| Open | invalid UTF-8 | explain and do not mutate file |
| Parse/index | outline cannot refresh | show raw source/retry |
| Save | permission denied | save as / retry |
| Edit | stale node | refresh focus/root |
| Structural edit | invalid target | choose different target |
| Runtime | file dialog unavailable | fallback path or manual entry later |

### Message Pattern

```text
What happened.
Why it likely happened.
What the user can do now.
Technical details, collapsible if needed.
```

### Logging

Logs stay local. Avoid logging full document content by default.

## 5. Validation and Test Plan

- Each failure has user-facing message.
- Save failure never clears dirty state.
- Index error offers raw source action.
- Logs do not include document body by default.

## 6. Acceptance Criteria

- Failures do not silently lose data.
- Users can recover from common errors.
- Error wording is consistent across app.

## 7. Dependencies

- RFC-009
- RFC-015
- RFC-017

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
