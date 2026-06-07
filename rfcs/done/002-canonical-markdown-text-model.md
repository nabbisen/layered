<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-002: Canonical Markdown Text Model

**Project:** layered — Layer EDitor  
**Milestone:** M0 — Technical Spike  
**Status.** Implemented (v0.1.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define the core document model where the raw Markdown source is the canonical truth and every outline, focus view, and section body is a derived projection.

## 2. Goals

- Make raw Markdown text the only canonical document representation.
- Define range units and UTF-8 safety rules.
- Define revision tracking for stale edit detection.
- Leave room to replace `String` with `Rope` after M0 if measured need appears.

## 3. Non-Goals

- No AST-as-canonical model.
- No lossy Markdown serialization.
- No hidden metadata in Markdown files.

## 4. Design

### Canonical Principle

```text
Document source text = canonical
Outline tree = derived index
Focused editor text = temporary local edit buffer
Saved file = canonical source text written back to disk
```

### M0 Buffer Decision

M0 may use `String` as the backing buffer to reduce complexity. The public core API should avoid exposing this choice so that a later `Rope` migration does not rewrite the UI.

### Range Unit Policy

All persisted core ranges are byte ranges into valid UTF-8 source text.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    pub start: usize,
    pub end: usize,
}
```

Rules:

- `start <= end`.
- both boundaries must be valid UTF-8 character boundaries;
- ranges are half-open: `[start, end)`;
- parser/index offsets must be normalized to `ByteRange`;
- UI selection offsets must be converted before core mutation.

### Revision Policy

Every successful mutation increments `DocumentRevision`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocumentRevision(pub u64);
```

UI edit buffers remember the revision they were created from. A commit against an older revision fails with `EditError::RevisionMismatch` unless the UI explicitly rebase/reloads the buffer.

## 5. Internal Design Notes

### Internal Struct Sketch

```rust
pub struct Document {
    text: TextBuffer,
    outline: Outline,
    revision: DocumentRevision,
    newline_policy: NewlinePolicy,
    encoding_policy: EncodingPolicy,
}

enum TextBuffer {
    String(String),
    // Future: Rope(ropey::Rope),
}
```

`TextBuffer` must provide:

```rust
fn as_str(&self) -> &str;
fn slice(&self, range: ByteRange) -> Result<&str, RangeError>;
fn replace(&mut self, range: ByteRange, replacement: &str) -> Result<(), EditError>;
```

## 6. Validation and Test Plan

- Byte ranges reject non-character-boundary positions.
- Japanese, emoji, and mixed ASCII/UTF-8 documents remain valid after replacement.
- Revision increments exactly once per successful core edit.

## 7. Acceptance Criteria

- No API exposes a mutable raw string without validation.
- All core edits operate on canonical text and produce a new revision.
- The outline can be dropped and rebuilt without losing document content.
## 8. Dependencies

- RFC-001

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
