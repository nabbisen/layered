<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-006: Node Identity and Range Semantics

**Project:** layered — Layer EDitor  
**Milestone:** M1 — Core Document Engine  
**Status.** Implemented (v0.1.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define how section nodes are identified, how stable those IDs are, and how source ranges are represented.

## 2. Goals

- Allow duplicate heading titles.
- Define deterministic node identity across re-indexing where possible.
- Define range invariants for heading/body/full ranges.
- Detect stale nodes after edits.

## 3. Non-Goals

- No cross-file global identity.
- No hidden heading anchors inserted into Markdown.
- No guarantee that IDs survive arbitrary raw-source rewrite.

## 4. Design

### Node Identity Policy

M1 uses deterministic IDs derived from source-order structure, not title alone.

```rust
pub struct NodeId(u64);
```

Adopted algorithm (design review, v0.1.0):

```text
NodeId = FNV-1a hash of the node's ordinal path from the root
         (the sequence of zero-based child indices), length-prefixed
         so the root path [] cannot alias [0]
```

An earlier candidate mixed the heading byte start into the hash. It was
rejected during review: byte starts shift whenever any earlier section grows
or shrinks, which would change the IDs of every later section on ordinary
body edits — defeating the stability requirement below. The ordinal path is
invariant under body-only edits, so:

- duplicate heading titles receive distinct IDs (different paths);
- body replacement that does not change heading structure keeps every ID
  stable across re-indexing;
- IDs are deterministic for identical document structure;
- IDs are **not** guaranteed to survive edits that add, remove, or reorder
  headings. The UI must treat IDs as revision-scoped and re-resolve focus
  after structural changes (see the stale rule below).

### Range Types

```rust
pub struct SectionRanges {
    pub heading: ByteRange,
    pub body: ByteRange,
    pub full: ByteRange,
}
```

Invariants:

```text
full.start <= heading.start
heading.end <= body.start
body.end <= full.end
children full ranges are contained in parent full range
body range excludes child sections
```

### Parent/Child Invariants

- Root has no parent.
- Non-root nodes have exactly one parent.
- Children are ordered by source position.
- A node cannot be its own ancestor.

## 5. Internal Design Notes

### Stale Node Handling

When an operation receives a `NodeId`, core resolves it against the current outline. If absent, return `DocumentError::NodeNotFound` or `EditError::StaleNode`. UI should refresh focus or navigate to nearest surviving ancestor.

## 6. Validation and Test Plan

- Duplicate headings have distinct IDs.
- Body-only edits keep heading IDs stable where heading structure is unchanged.
- Invalid range construction fails.
- Tree parent/child invariant checker detects corrupt indexes.

## 7. Acceptance Criteria

- `NodeId` and `ByteRange` have explicit semantics documented in rustdoc.
- Core validates range boundaries before mutation.
- Stale node behavior is deterministic.

## 8. Dependencies

- RFC-002
- RFC-003

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
