<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-033: Render Boundary and State Update Policy

**Project:** layerd — Layer EDitor  
**Milestone:** M7 — Performance and Large Document Readiness  
**Status.** Proposed  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Prevent text input and navigation from causing unnecessary full UI updates.

## 2. Goals

- Define global vs local UI state.
- Avoid full outline re-render during typing.
- Use stable component props and derived snapshots.
- Identify anti-patterns.

## 3. Non-Goals

- No Dioxus internals tuning without measurement.
- No virtualized outline requirement unless needed.
- No custom renderer.

## 4. Design

### State Ownership Map

| State | Owner | Update Frequency |
|---|---|---|
| Document | app/global signal | commit/open/save |
| Current focus id | app/global signal | navigation |
| Focus body buffer | focus editor local state | every keystroke |
| Dirty status | app + editor derived | edit/save |
| Search query | search component | typing in search |

### Anti-Patterns

- storing full document text in multiple global signals;
- recomputing full outline on every input event;
- passing large source strings through many components;
- deriving breadcrumbs in every render instead of using snapshot/path.

## 5. Internal Design Notes

### Render Boundary

Focus editor should receive a `FocusSnapshot` and maintain local buffer. Outline should receive a lightweight `OutlineViewModel`. Saving commits local buffer first, then updates global document state.

## 6. Validation and Test Plan

- Typing in body editor does not update outline component.
- Navigation updates breadcrumb and focus view only.
- Large outline fixture renders within measured budget.
- State-update tests use instrumentation where feasible.

## 7. Acceptance Criteria

- UI remains responsive during text input.
- Component responsibilities are clear.
- Performance issues can be traced to known state boundaries.

## 8. Dependencies

- RFC-012
- RFC-031
- RFC-032

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
