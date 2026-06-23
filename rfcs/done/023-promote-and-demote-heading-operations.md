<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-023: Promote and Demote Heading Operations

**Project:** omriss — Omriss Editor  
**Milestone:** M5 — Structural Editing  
**Status.** Implemented (v0.9.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define structural operations that change heading depth while preserving section text.

## 2. Goals

- Allow users to raise/lower abstraction level.
- Define behavior for child headings.
- Prevent invalid H0/H7 results.
- Make operation preview/testable.

## 3. Non-Goals

- No drag-and-drop UI requirement.
- No automatic outline reorganization beyond level changes.
- No semantic rewriting of titles.

## 4. Design

### Operation Semantics

Promote decreases heading marker level by one. Demote increases heading marker level by one.

```text
# H1 cannot be promoted
###### H6 cannot be demoted
```

Child policy:

- Default: adjust the target heading only.
- Future option: adjust entire subtree.

M5 should choose target-only first because it is simpler and preserves user intent, but UI must show the effect clearly.

### Before/After

```markdown
## Section
Text
### Child
```

Promote Section:

```markdown
# Section
Text
### Child
```

This creates a skipped level; skipped levels are allowed by RFC-007.

## 5. Internal Design Notes

### Implementation

Find heading marker range within `heading_range`, replace marker text with target marker length. Re-index after edit.

## 6. Validation and Test Plan

- H1 promote rejected.
- H6 demote rejected.
- Duplicate title promote targets correct NodeId.
- Child sections are preserved exactly unless heading marker lies in edited range.

## 7. Acceptance Criteria

- User can promote/demote a section without body loss.
- Operation clearly communicates skipped-level possibility.
- Golden tests prove unrelated bytes are preserved.

## 8. Dependencies

- RFC-006
- RFC-008
- RFC-026

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
