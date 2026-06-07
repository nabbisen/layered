<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-018: Line Endings, Encoding, and File Integrity

**Project:** layerd — Layer EDitor  
**Milestone:** M3 — File Lifecycle and Recovery  
**Status.** Implemented (v0.3.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define file-level integrity policies for encoding, BOM, line endings, and trailing newlines.

## 2. Goals

- Preserve common Markdown file characteristics.
- Require safe UTF-8 handling.
- Avoid unintended newline normalization.
- Define golden integrity tests.

## 3. Non-Goals

- No arbitrary legacy encoding editing.
- No automatic document formatting.
- No content linting.

## 4. Design

### Encoding Policy

MVP supports UTF-8. Files with UTF-8 BOM may be opened and saved with the BOM preserved if detected.

Invalid UTF-8 behavior:

```text
show error -> do not open as editable document -> suggest external conversion
```

### Line Ending Policy

Detect dominant line ending on open:

```text
LF
CRLF
Mixed
```

MVP preservation rule:

- unrelated bytes are preserved exactly;
- replacement text from UI uses the current platform/editor newline normalized to document dominant newline before commit, unless this would violate explicit raw editing later;
- mixed files preserve unaffected regions and warn that new edited body uses dominant newline.

### Trailing Newline Policy

Preserve the file's existing trailing newline state unless the edited range includes end-of-document and user input changes it.

## 5. Internal Design Notes

### Metadata

```rust
pub struct FileTextProfile {
    pub newline: NewlinePolicy,
    pub had_utf8_bom: bool,
    pub had_trailing_newline: bool,
}
```

This profile belongs to app/file lifecycle state, while core may store newline policy if it affects replacement normalization.

## 6. Validation and Test Plan

- LF fixture preserves LF outside edit.
- CRLF fixture preserves CRLF outside edit.
- UTF-8 BOM fixture round-trips.
- Invalid UTF-8 rejected safely.
- Trailing newline fixture unchanged outside edited EOF range.

## 7. Acceptance Criteria

- Opening and saving an unedited file does not change bytes.
- Editing one body preserves unrelated bytes.
- Encoding and newline behavior is documented to users.

## 8. Dependencies

- RFC-002
- RFC-004
- RFC-015

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
