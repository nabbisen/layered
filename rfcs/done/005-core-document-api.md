<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-005: Core Document API

**Project:** omriss — Omriss Editor  
**Milestone:** M1 — Core Document Engine  
**Status.** Implemented (v0.1.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Stabilize the minimum public API of `omriss` so UI and desktop crates can rely on a predictable document engine.

## 2. Goals

- Define read-only document accessors.
- Define mutation methods for body edits.
- Define error-returning API rather than panic-driven behavior.
- Keep implementation details private.

## 3. Non-Goals

- No structural editing API beyond body replacement.
- No plugin API.
- No persistence API inside core except text import/export.

## 4. Design

### Public API Surface

```rust
pub struct Document;
pub struct Outline;
pub struct SectionNode;
pub struct FocusSnapshot;

impl Document {
    pub fn parse(markdown: String) -> Result<Self, DocumentError>;
    pub fn source(&self) -> &str;
    pub fn revision(&self) -> DocumentRevision;
    pub fn outline(&self) -> &Outline;
    pub fn focus_snapshot(&self, id: NodeId) -> Result<FocusSnapshot, DocumentError>;
    pub fn replace_section_body(&mut self, cmd: ReplaceSectionBody) -> Result<EditResult, EditError>;
}
```

### FocusSnapshot

The UI should not assemble focus mode by directly traversing internal maps. Core returns a stable projection:

```rust
pub struct FocusSnapshot {
    pub node_id: NodeId,
    pub title: String,
    pub level: HeadingLevel,
    pub body: String,
    pub children: Vec<OutlineItem>,
    pub path: Vec<OutlineItem>,
    pub revision: DocumentRevision,
}
```

This provides the data needed for breadcrumbs, focus editor, and direct child cards.

## 5. Internal Design Notes

### Visibility Rule

Keep fields private and use constructors/accessors. This allows replacement of internal storage from `Vec`/`HashMap` to arena/indexmap/rope without breaking the UI crate.

### Mutation Rule

All public mutations use command structs:

```rust
pub struct ReplaceSectionBody {
    pub node_id: NodeId,
    pub base_revision: DocumentRevision,
    pub new_body: String,
}
```

## 6. Validation and Test Plan

- API-level tests for parse, focus snapshot, replacement, and error paths.
- Snapshot body matches exact source body range.
- Outline ordering is source-order deterministic.

## 7. Acceptance Criteria

- `omriss-ui` can implement M2 screens using only public core API.
- No UI code accesses core internals through public fields.
- Every mutation reports an `EditResult`.

## 8. Dependencies

- RFC-001
- RFC-002
- RFC-003
- RFC-004

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
