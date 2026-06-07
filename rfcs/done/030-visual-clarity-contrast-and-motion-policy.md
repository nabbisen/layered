<!--
Project: layered — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-030: Visual Clarity, Contrast, and Motion Policy

**Project:** layered — Layer EDitor  
**Milestone:** M6 — Accessibility and Usability Hardening  
**Status.** Implemented (v0.10.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define visual usability principles for hierarchy, focus, contrast, and motion.

## 2. Goals

- Set contrast and readability targets.
- Show hierarchy using multiple cues.
- Respect reduced motion.
- Keep focus mode visually calm.

## 3. Non-Goals

- No final branding system.
- No theme marketplace.
- No animated canvas requirement.

## 4. Design

### Visual Hierarchy Rules

Hierarchy must not rely on color alone. Combine:

```text
indentation
heading level label
font size/weight
borders or spacing
textual path
```

### Contrast

Target WCAG AA contrast for text and controls. Critical status/errors should exceed minimum when practical.

### Motion

Zoom animations are optional. If used:

- short duration;
- no essential information conveyed only by motion;
- disabled when reduced-motion is requested by OS/browser environment.

### Density

Initial density should favor readability over information packing. Compact density can be future setting.

## 5. Validation and Test Plan

- Hierarchy understandable in grayscale.
- Focus ring visible.
- Reduced motion disables zoom animation.
- Text contrast meets target tokens.

## 6. Acceptance Criteria

- The UI remains readable for long writing sessions.
- Current focus is visually obvious.
- Visual design does not undermine accessibility semantics.

## 7. Dependencies

- RFC-011
- RFC-012
- RFC-027

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
