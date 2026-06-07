<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-042: Release Readiness Criteria

**Project:** layerd — Layer EDitor  
**Milestone:** M9 — Production Readiness  
**Status.** Implemented (v0.11.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define what must be true before layerd is released publicly.

## 2. Goals

- Set release blocker criteria.
- Define supported platform matrix.
- Require data integrity tests.
- Create sign-off process.
- Document acceptable limitations.

## 3. Non-Goals

- No commercial launch plan.
- No long-term support policy.
- No auto-update guarantee.

## 4. Design

### Release Checklist

```text
[ ] Core source-preservation tests pass
[ ] File open/save lifecycle tests pass
[ ] Platform smoke tests completed
[ ] Known limitations documented
[ ] Release notes written
[ ] Checksums produced
[ ] No release-blocking bugs open
```

### Release Blockers

- known data corruption;
- save success reported incorrectly;
- inability to open ordinary UTF-8 Markdown;
- keyboard-only primary workflow impossible;
- app crash during common open/edit/save path;
- undocumented severe platform limitation.

### Acceptable Limitations for Early Release

Allowed if documented:

```text
limited raw source editing
no plugins
no collaboration
no AI features
no rich preview
manual installation warnings for unsigned builds
```

## 5. Internal Design Notes

### Sign-Off

Release sign-off should include product owner and technical owner. If the user has a rule that public release requires explicit owner confirmation, that rule must be documented here and honored by release workflow.

## 6. Validation and Test Plan

- Release checklist template exists.
- Blocker labels exist in issue tracker or equivalent.
- Smoke evidence attached before release.
- Known limitations included in release notes.

## 7. Acceptance Criteria

- The team has a concrete go/no-go standard.
- Data integrity is the top release criterion.
- Early release limitations are transparent.

## 8. Dependencies

- RFC-037
- RFC-038
- RFC-040
- RFC-041

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
