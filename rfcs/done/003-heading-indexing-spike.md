<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-003: Heading Indexing Spike

**Project:** layered — Layer EDitor  
**Milestone:** M0 — Technical Spike  
**Status.** Implemented (v0.1.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Validate heading extraction and hierarchy construction from the canonical Markdown source. This RFC is intentionally a spike: it proves feasibility before finalizing the production indexing engine.

## 2. Goals

- Extract headings from Markdown source without treating headings in code fences as real headings.
- Build a navigable tree with a synthetic root node.
- Handle duplicate titles and skipped heading levels.
- Create fixtures for Markdown edge cases.

## 3. Non-Goals

- No incremental parser.
- No complete CommonMark conformance certification.
- No structural edit operations.

## 4. Design

### Heading Detection

The indexer must detect Markdown headings through parser-supported heading events rather than line-regex alone. This prevents headings inside fenced code blocks from becoming section nodes.

Supported in M0:

- ATX headings: `#`, `##`, ..., `######`;
- duplicate titles;
- nested headings;
- root-level content before first heading.

Candidate support to validate:

- Setext headings;
- front matter boundaries;
- HTML blocks around heading-like text.

### Synthetic Root

Every document has a synthetic root node that does not correspond to a heading line.

```text
Root
  H1 Introduction
  H1 Body
    H2 Detail
```

Root body range covers document content before the first real heading, excluding front matter if the file lifecycle RFC adopts front matter recognition.

### Skipped Levels

Skipped levels are allowed and represented literally.

```markdown
# A
### B
```

`B` becomes a child of `A`; no synthetic H2 is inserted.

## 5. Internal Design Notes

### Stack Algorithm

```text
initialize stack = [root]
for each heading event in source order:
  pop stack until top.level < heading.level
  parent = stack.top
  create node
  append node to parent.children
  push node
calculate body/full ranges after all heading starts are known
```

Full range of a node begins at its heading start and ends before the next heading whose level is less than or equal to the node's level, or end-of-document.

Body range begins after heading line/end and ends at first child heading start or full range end.

## 6. Validation and Test Plan

- `#` inside fenced code is ignored.
- Duplicate headings produce distinct nodes.
- H1 > H3 skipped level produces direct parent/child relation.
- Root preface content is addressable.

## 7. Acceptance Criteria

- M0 prototype can render an outline from real Markdown source.
- Indexer produces deterministic node order for identical source.
- Known edge cases are captured as fixtures even if some remain deferred.
## 8. Dependencies

- RFC-001
- RFC-002

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
