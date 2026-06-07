<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-040: Test Strategy and Regression Policy

**Project:** layered — Layer EDitor  
**Milestone:** M9 — Production Readiness  
**Status.** Implemented (v0.11.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define the test pyramid and regression process for layered.

## 2. Goals

- Protect source-preservation invariant.
- Define unit/golden/UI/manual tests.
- Create regression workflow.
- Use fixtures consistently.

## 3. Non-Goals

- No perfect formal verification.
- No expensive E2E-only strategy.
- No flaky UI tests as sole release gate.

## 4. Design

### Test Pyramid

```text
Many: layered-core unit tests
Many: golden Markdown fixture tests
Some: UI component/command tests
Some: desktop smoke tests
Few: manual release checks
```

### Non-Negotiable Invariant

```text
A section edit must not rewrite unrelated bytes.
```

Every edit/structural RFC must add tests around this invariant.

### Regression Policy

When a bug is fixed:

1. add minimal fixture or unit test reproducing it;
2. fix implementation;
3. keep test permanently unless superseded;
4. classify severity.

## 5. Validation and Test Plan

- CI runs core and fixture golden tests.
- Regression template exists.
- Structural edit tests include byte preservation.
- UI shortcut tests cover primary workflows.

## 6. Acceptance Criteria

- No release without passing data-integrity test suite.
- Fixtures are owned and documented.
- Bugs that affect data preservation become release blockers.

## 7. Dependencies

- RFC-034
- RFC-038
- RFC-042

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
