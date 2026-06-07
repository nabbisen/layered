<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-034: Large Document Test Fixtures

**Project:** layered — Layer EDitor  
**Milestone:** M7 — Performance and Large Document Readiness  
**Status.** Implemented (v0.10.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Create realistic documents for correctness, performance, and UX regression tests.

## 2. Goals

- Define fixture categories.
- Include Markdown edge cases.
- Support generated and hand-written fixtures.
- Use fixtures across core/UI/performance tests.

## 3. Non-Goals

- No private copyrighted fixture corpus.
- No massive binary test data.
- No benchmark-only documents without correctness assertions.

## 4. Design

### Fixture Catalog

```text
academic-paper.md
scenario-outline.md
technical-rfc.md
heading-edge-cases.md
utf8-japanese.md
crlf-document.md
frontmatter-comments.md
large-10k-words.md
large-100k-words.md (optional/performance only)
```

### Golden Expectations

Each fixture should define expected:

```text
heading count
tree shape
selected body ranges
source preservation after sample edit
line ending profile
```

### Generated Fixtures

Generated documents must be deterministic and documented. Avoid random output unless seed is fixed.

## 5. Validation and Test Plan

- Fixture metadata matches indexer output.
- Golden body replacement tests run against all relevant fixtures.
- Performance fixtures are excluded from slow CI unless enabled.
- Edge-case fixtures contain comments explaining purpose.

## 6. Acceptance Criteria

- New parser/edit changes are tested against realistic documents.
- Performance claims can be reproduced.
- Fixture set represents target users, not only synthetic Markdown.

## 7. Dependencies

- RFC-003
- RFC-018
- RFC-031

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
