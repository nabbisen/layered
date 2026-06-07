<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-038: Platform Smoke Test Matrix

**Project:** layered — Layer EDitor  
**Milestone:** M8 — Cross-Platform Delivery  
**Status.** Implemented (v0.11.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define minimum validation before desktop artifacts are trusted.

## 2. Goals

- Create OS smoke checklist.
- Validate open/edit/save integrity.
- Validate keyboard shortcuts.
- Capture evidence for release sign-off.

## 3. Non-Goals

- No exhaustive QA suite.
- No full manual test book.
- No certification claim.

## 4. Design

### Smoke Test Workflow

For each supported OS:

```text
launch app
open fixture Markdown
view overview
zoom into section
edit body
save as new file
compare unrelated bytes
show raw source
close/reopen saved file
run basic keyboard navigation
```

### Release-Blocking Failures

- app fails to launch;
- open/save corrupts unrelated text;
- Ctrl/Cmd+S cannot save;
- keyboard navigation traps user;
- save failure falsely reports success.

### Evidence Format

```text
OS/version
artifact name
fixture used
steps completed
failures/screenshots/log excerpt
sign-off person/date
```

## 5. Validation and Test Plan

- Smoke checklist exists in repository.
- At least one fixture used for byte comparison.
- Release-blocker categories are enforced during sign-off.

## 6. Acceptance Criteria

- A release has platform evidence before publication.
- Data integrity is smoke-tested, not assumed.
- Manual and automated test boundaries are explicit.

## 7. Dependencies

- RFC-015
- RFC-018
- RFC-035

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
