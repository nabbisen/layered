<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-041: User Documentation and Onboarding

**Project:** layerd — Layer EDitor  
**Milestone:** M9 — Production Readiness  
**Status.** Proposed  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Help users understand the zoom paradigm, source trust model, and limitations.

## 2. Goals

- Explain layerd in one minute.
- Teach zoom/focus workflows.
- Document source preservation.
- Document limitations honestly.

## 3. Non-Goals

- No marketing website requirement.
- No tutorial video requirement.
- No AI assistant documentation.

## 4. Design

### First-Run Onboarding

```text
Welcome to layerd
1. Open a Markdown file
2. Zoom into headings
3. Edit one layer at a time
4. Save clean Markdown
5. Use Raw Source to verify everything
```

### User Guide Outline

```text
What layerd is
Why zoom editing helps
Opening and saving files
Overview mode
Focus mode
Breadcrumbs
Raw Markdown view
Keyboard shortcuts
Search
Known limitations
Data safety model
```

### Example Document

Ship a small example Markdown document that demonstrates H1/H2/H3 hierarchy, child sections, and raw source view.

## 5. Validation and Test Plan

- Onboarding can be completed with keyboard.
- Shortcut reference matches command registry.
- Known limitations page exists before public release.
- Example document opens and indexes correctly.

## 6. Acceptance Criteria

- A new user can understand the main workflow without external explanation.
- Documentation does not overpromise rich editing features.
- Trust model is explicit and plain-language.

## 7. Dependencies

- RFC-010
- RFC-014
- RFC-017
- RFC-022

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
