<!-- fixture: technical-rfc.md
     Purpose: an RFC-style document with numbered sections, sub-sections,
     tables, and code fences. Tests that structural content (code, tables)
     inside bodies does not affect section boundary detection.
-->
# RFC 0099: Section Range Calculation Policy

**Status:** Proposed
**Milestone:** Fixtures

---

## 1. Summary

Define how the byte range of a section is computed, including the boundary
between adjacent sections and the treatment of trailing blank lines.

## 2. Goals

- Establish a deterministic rule for section full-range boundaries.
- Handle edge cases: last section in document, sections with no body, nested
  sections, setext headings.
- Keep the rule simple enough to unit-test exhaustively.

## 3. Non-Goals

- No semantic meaning is assigned to blank lines between sections.
- No preservation guarantee for blank lines added by editors.

## 4. Design

### 4.1 Range Policy

The *full range* of a section node spans from the first byte of its heading
line to the last byte of its last descendant (inclusive), or to the first
byte of the next sibling's heading, exclusive.

```text
│ ## Section A     ← heading_range.start
│ body A
│
│ ### Child A.1    ← nested heading
│ body A.1
│                  ← full_range.end = heading of B, exclusive
│ ## Section B
```

### 4.2 Body Range Policy

The *body range* spans from the byte immediately following the heading line's
newline to the first byte of the first child heading (exclusive), or to
`full_range.end` if there are no children.

| Case | body_range.end |
|---|---|
| No children | full_range.end |
| Has children | first child heading_range.start |
| Empty body | heading_range.end (zero-length range) |

### 4.3 Trailing Blank Lines

Trailing blank lines between a section's body and the next heading belong to
the body range of the preceding section. This ensures that body edits do not
accidentally consume bytes belonging to the next section.

## 5. Validation and Test Plan

Each of the following must have a golden test fixture:

- Adjacent ATX sections with one blank line between them.
- Adjacent ATX sections with no blank lines.
- Nested sections (parent / child / grandchild).
- Section with no body and an immediate child.
- Final section in document (no following sibling).

## 6. Acceptance Criteria

- Range calculation is deterministic given the source string.
- All golden tests pass after any change to the indexer.
- Ranges in the tree never overlap, never leave gaps relative to full_range.

## 7. Dependencies

- RFC-003: Markdown Parsing Strategy
- RFC-006: Section Identity Model
