<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-032: Re-index Strategy and Debounce Lifecycle

**Project:** omriss — Omriss Editor  
**Milestone:** M7 — Performance and Large Document Readiness  
**Status.** Implemented (v0.10.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define when heading indexes are rebuilt and when incremental indexing should be considered.

## 2. Goals

- Use simple full re-index initially.
- Avoid re-index on every keystroke.
- Define debounce/commit timing.
- Set criteria for future incremental parser.

## 3. Non-Goals

- No incremental parser in first implementation.
- No background worker requirement unless measured.
- No speculative caching complexity.

## 4. Design

### Lifecycle

```text
User types -> local buffer only
Pause/blur/focus change/save -> commit body
Commit -> replace source range
After replacement -> full re-index
UI refresh -> new snapshot
```

### Debounce

Debounce should be a UI policy, not core policy. Core operations remain synchronous and deterministic.

### Reconsider Incremental Indexing When

```text
full re-index exceeds agreed threshold on target fixtures
or UI commit/save becomes noticeably blocking
or large document fixtures become central target use cases
```

### Failure Behavior

If re-index after commit fails, the canonical text must not be silently discarded. Show raw source and error details.

## 5. Validation and Test Plan

- Typing does not trigger core re-index per keystroke.
- Commit triggers exactly one re-index.
- Re-index failure surfaces recoverable state.
- Debounce can be disabled in tests.

## 6. Acceptance Criteria

- MVP has simple and understandable indexing behavior.
- Optimization path is documented but deferred.
- Core remains deterministic and easy to test.

## 7. Dependencies

- RFC-004
- RFC-008
- RFC-031

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
