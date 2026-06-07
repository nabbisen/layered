<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-004: Source-Preserving Section Replacement

**Project:** layerd — Layer EDitor  
**Milestone:** M0 — Technical Spike  
**Status.** Implemented (v0.1.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Prove the key product invariant: editing one section body can update the canonical Markdown source without rewriting unrelated document bytes.

## 2. Goals

- Define `replace_section_body` for M0.
- Preserve heading line and child sections unless explicitly edited.
- Preserve unrelated bytes exactly.
- Define failure behavior for stale nodes and invalid ranges.

## 3. Non-Goals

- No promote/demote or section movement.
- No collaborative merge.
- No auto-formatting.

## 4. Design

### Operation Semantics

`replace_section_body(node_id, new_body, base_revision)` replaces only the current node's body range.

```text
heading line: preserved
body text: replaced
child sections: preserved
siblings: preserved
ancestors: preserved
```

Example:

```markdown
# A
A body

## A.1
child

# B
B body
```

Replacing body of `A` changes only `A body

` and leaves `## A.1` and `# B` untouched.

### Whitespace Policy

M0 should not auto-normalize blank lines. If the user enters text with or without trailing newline, the operation stores exactly the provided replacement inside the body range. UI may later warn if the result visually collapses into a child heading, but core must not silently rewrite.

## 5. Internal Design Notes

### API Sketch

```rust
pub fn replace_section_body(
    &mut self,
    id: NodeId,
    replacement: &str,
    base_revision: DocumentRevision,
) -> Result<EditResult, EditError>;
```

`EditResult` contains:

```rust
pub struct EditResult {
    pub old_revision: DocumentRevision,
    pub new_revision: DocumentRevision,
    pub replaced_range: ByteRange,
    pub new_range: ByteRange,
    pub reindexed: bool,
}
```

After replacement, the document must re-index before returning success. If re-index fails, M0 should retain text and return an error state that can be shown in raw source view. Later RFCs may introduce transaction rollback.

## 6. Validation and Test Plan

- Golden test: prefix and suffix outside replaced body are byte-identical.
- CRLF document replacement preserves unrelated CRLF bytes.
- Stale revision returns `RevisionMismatch`.
- Invalid node ID returns `NodeNotFound`.

## 7. Acceptance Criteria

- M0 can edit a focused section body and save a valid Markdown file.
- Byte-preservation tests pass for representative fixtures.
- No AST serialization is used to produce saved Markdown.
## 8. Dependencies

- RFC-002
- RFC-003

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
