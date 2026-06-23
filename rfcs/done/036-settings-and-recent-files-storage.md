<!--
Project: omriss — Omriss Editor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-036: Settings and Recent Files Storage

**Project:** omriss — Omriss Editor  
**Milestone:** M8 — Cross-Platform Delivery  
**Status.** Implemented (v0.11.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define local app settings without compromising Markdown file portability.

## 2. Goals

- Store app preferences outside Markdown documents.
- Define early settings.
- Define recent files model.
- Protect privacy and allow clearing.

## 3. Non-Goals

- No document-required sidecar metadata.
- No cloud preferences.
- No sensitive analytics.

## 4. Design

### Principle

Settings must never be required to understand or recover a Markdown document.

### Initial Settings

```toml
[editor]
font_size = 14
line_wrap = true

[ui]
last_mode = "overview"
show_recent_files = true

[files]
recent = []
```

### Storage Path

Use OS-appropriate config/data directories through a standard Rust directory crate or platform API. Exact crate choice is implementation detail.

### Recent Files

Recent files store local paths only. Provide clear action to remove a file from recent list or clear all recents.

## 5. Validation and Test Plan

- Settings file creation does not touch Markdown file.
- Recent file missing path handled gracefully.
- Clear recent files removes paths.
- Invalid settings fall back to defaults with warning.

## 6. Acceptance Criteria

- omriss can run with no settings file.
- Settings migration policy exists before schema changes.
- Privacy implications of recent file paths are documented.

## 7. Dependencies

- RFC-015
- RFC-035

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
