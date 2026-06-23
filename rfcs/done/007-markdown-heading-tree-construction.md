<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-007: Markdown Heading Tree Construction

**Project:** omriss — Omriss Editor  
**Milestone:** M1 — Core Document Engine  
**Status.** Implemented (v0.1.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Promote the M0 heading-indexing spike into production behavior for constructing the omriss outline tree.

## 2. Goals

- Define full heading tree construction algorithm.
- Represent root and pre-heading content.
- Handle skipped heading levels intentionally.
- Document supported Markdown heading forms.

## 3. Non-Goals

- No outline generated from non-heading Markdown blocks.
- No automatic repair of invalid user structure.
- No semantic interpretation of document meaning.

## 4. Design

### Tree Construction Contract

The outline is built from heading events in source order. Parent selection is based on heading level:

```text
new heading parent = nearest previous heading with lower level, else root
```

### Root Design

Root represents the document itself. It has:

```text
level: Root
heading_range: empty at byte 0
body_range: content before first heading
full_range: whole document
```

### Skipped Heading Levels

Skipped levels are allowed because Markdown allows them. omriss should not insert invisible missing headings.

### Front Matter Policy

If the document starts with YAML/TOML front matter, it is treated as root-level source content and excluded from heading detection unless the parser already ignores it. omriss must preserve it exactly.

### Malformed Documents

If Markdown can still be parsed as text, the document should open. Index warnings are allowed; save should not be blocked solely because heading hierarchy is unusual.

## 5. Internal Design Notes

### Node Storage

Use an insertion-order collection for deterministic traversal.

Candidate:

```rust
pub struct Outline {
    root_id: NodeId,
    nodes: IndexMap<NodeId, SectionNode>,
    source_order: Vec<NodeId>,
}
```

`source_order` supports search result grouping, next/previous traversal, and stable UI rendering.

## 6. Validation and Test Plan

- ATX headings tree.
- Setext fixture accepted or explicitly marked unsupported for M1.
- Front matter preserved.
- Root body before first heading accessible.
- Skipped levels produce no synthetic node.

## 7. Acceptance Criteria

- Production tree builder replaces M0 spike code.
- Outline exposes root, children, source order, parent lookup, and path lookup.
- Tree builder has golden fixtures for edge cases.

## 8. Dependencies

- RFC-003
- RFC-006

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
