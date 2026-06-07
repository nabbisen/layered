<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-017: Raw Markdown Escape Hatch

**Project:** layerd — Layer EDitor  
**Milestone:** M3 — File Lifecycle and Recovery  
**Status.** Implemented (v0.3.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define the full-source view that lets users inspect and, later, edit canonical Markdown directly.

## 2. Goals

- Provide transparent full-source inspection.
- Offer recovery path when indexing fails.
- Define read-only vs editable policy.
- Sync raw view with structured focus view.

## 3. Non-Goals

- No syntax highlighting requirement.
- No split-view preview.
- No advanced source editor features.

## 4. Design

### M3 Decision

Raw source view should be **read-only in the first stable implementation** unless the team explicitly accepts the additional edit/merge risk. Editable raw source can follow once RFC-008 and RFC-016 are mature.

### Raw Source Wireframe

```text
+--------------------------------------------------------------------------------+
| Raw Markdown Source                                      [Back to Structure]    |
+--------------------------------------------------------------------------------+
| This is the exact Markdown text currently held by layerd.                       |
|                                                                                |
| +----------------------------------------------------------------------------+ |
| | ---                                                                        | |
| | title: Example                                                             | |
| | ---                                                                        | |
| |                                                                            | |
| | # Chapter 1                                                                | |
| | Body text...                                                               | |
| +----------------------------------------------------------------------------+ |
|                                                                                |
| Status: read-only source view · Ctrl+S saves canonical text                    |
+--------------------------------------------------------------------------------+
```

### Recovery Use

If indexing fails but text is valid UTF-8, layerd should still allow raw source viewing and saving if safe.

## 5. Internal Design Notes

### Sync Lifecycle

Entering raw source view renders `Document::source()`. If the view is read-only, there is no separate buffer. If later made editable, raw edits must use a revision-checked full-range replace command and force immediate re-index.

## 6. Validation and Test Plan

- Raw view displays exact canonical source.
- Back to Structure returns to previous focus if node survives.
- Index warning can route to raw source.
- Read-only raw view cannot mutate text accidentally.

## 7. Acceptance Criteria

- Users can verify source preservation at any time.
- Raw view is available even when outline projection is partially broken.
- The source view makes no hidden formatting transformations.

## 8. Dependencies

- RFC-015
- RFC-016

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
