# RFC-046: Document Statistics

**Project:** layerd — Layer EDitor
**Milestone:** Post-MVP Expansion
**Status.** Implemented (v0.12.0)
**Document type:** Detailed RFC design
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer

---

## 1. Summary

Display word count and section count for the document and the focused section
body in the status bar.

## 2. Goals

- Count words in the focused body and in the whole document.
- Count sections in the outline tree.
- Display non-intrusively in the status bar.
- Recompute only on document commits (not on every keystroke).

## 3. Non-Goals

- No character count or reading-time estimate in this RFC.
- No per-section statistics panel.
- No export of statistics.

## 4. Design

### Word Count Algorithm

Split body text on Unicode whitespace boundaries. Count non-empty chunks.
Use `str::split_whitespace` for correctness with multibyte (CJK) text is
approximate but acceptable for a writing aid; exact word segmentation is
deferred.

### Stats Model

```rust
pub struct DocumentStats {
    pub total_words: usize,
    pub focused_words: usize,   // 0 when in overview
    pub section_count: usize,
}
```

Computed via `EditorSession::stats()` in `layerd-ui`.

### Display

Added to the status bar between the dirty marker and the line-ending label:

```text
Words: 1 234 · Sections: 42
```

Displayed only when a document is open. Separator characters avoid
cluttering the bar with extra punctuation.

## 5. Validation and Test Plan

- Word count of empty string is 0.
- Word count matches manual count on fixture bodies.
- Section count equals `outline.iter().count()` minus the root.
- Stats update after a committed edit.

## 6. Acceptance Criteria

- Writers can see the approximate length of their document at a glance.
- Stats never trigger a re-index or source mutation.
- CJK text does not cause a panic (whitespace split is safe).

## 7. Dependencies

- RFC-011 (Outline and Overview UI)
- RFC-029 (Status Feedback)
