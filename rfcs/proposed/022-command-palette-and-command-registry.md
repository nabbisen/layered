<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-022: Command Palette and Command Registry

**Project:** layerd — Layer EDitor  
**Milestone:** M4 — Navigation and Search  
**Status.** Proposed  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define a testable command registry that supports palette search, shortcuts, and mode-based availability.

## 2. Goals

- Represent UI actions as commands.
- Enable/disable commands by current mode/state.
- Expose command metadata to help and palette UI.
- Keep command execution testable.

## 3. Non-Goals

- No plugin command API.
- No user scripting.
- No customizable shortcuts yet.

## 4. Design

### Command Metadata

```rust
pub struct CommandSpec {
    pub id: CommandId,
    pub title: &'static str,
    pub description: &'static str,
    pub default_shortcut: Option<Shortcut>,
    pub availability: AvailabilityRule,
}
```

### Palette Wireframe

```text
+----------------------------------------------+
| Run command: [ save                           ]|
+----------------------------------------------+
| Save File                         Ctrl+S       |
| Save As                           Ctrl+Shift+S |
| Show Raw Markdown                              |
+----------------------------------------------+
```

### Execution Lifecycle

```text
palette/search/shortcut -> resolve command -> check availability -> execute handler -> update app state/status
```

## 5. Validation and Test Plan

- Unavailable commands cannot execute.
- Shortcut table generated from registry.
- Palette search returns commands by title/description.
- Command execution can be tested with mocked app state.

## 6. Acceptance Criteria

- Keyboard help and palette share one source of command metadata.
- Mode-specific command availability is visible to users.
- Future shortcuts can be added without scattered handlers.

## 7. Dependencies

- RFC-014
- RFC-019
- RFC-021

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
