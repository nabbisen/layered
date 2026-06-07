<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-021: Search and Result Navigation

**Project:** layerd — Layer EDitor  
**Milestone:** M4 — Navigation and Search  
**Status.** Proposed  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define search so users are not dependent on global scrolling.

## 2. Goals

- Support current-focus and whole-document search.
- Group results by section path.
- Navigate to result by focusing containing section.
- Preserve plain-text search semantics.

## 3. Non-Goals

- No semantic/AI search.
- No regex in first version unless cheap.
- No index database.

## 4. Design

### Search Scopes

```text
Current Focus: current body and immediate visible child headings
Current Subtree: current section full range
Whole Document: canonical source text
```

M4 minimum should support Current Focus and Whole Document.

### Search Results Wireframe

```text
Search: [ abstraction                    ]  Scope: [Whole Document v]

Results
  Root > Chapter 1 > Section 1.2
    ...level of abstraction should be explicit...

  Root > Chapter 3
    ...abstraction leak in the design...
```

Selecting a result:

```text
focus containing section -> highlight match if feasible -> move keyboard focus to editor/search result context
```

## 5. Internal Design Notes

### Internal Search Result

```rust
pub struct SearchMatch {
    pub range: ByteRange,
    pub containing_node: NodeId,
    pub path: Vec<OutlineItem>,
    pub preview: String,
}
```

## 6. Validation and Test Plan

- Search hidden sibling content in whole-document scope.
- Search current focus excludes unrelated siblings.
- Selecting result changes focus to containing node.
- UTF-8 match ranges are valid.

## 7. Acceptance Criteria

- Users can find content outside current zoom layer.
- Search result path makes hidden context clear.
- Search does not mutate document state.

## 8. Dependencies

- RFC-007
- RFC-012

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
