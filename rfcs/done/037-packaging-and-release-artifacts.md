<!--
Project: layerd — Layer EDitor
Document Set: RFC detailed design bundle
Generated for architecture/design review
Language: English
-->
# RFC-037: Packaging and Release Artifacts

**Project:** layerd — Layer EDitor  
**Milestone:** M8 — Cross-Platform Delivery  
**Status.** Implemented (v0.11.0)  
**Document type:** Detailed RFC design  
**Primary audience:** Architect, Rust developer, UI/UX designer, QA engineer  

---

## 1. Summary

Define release artifact types, versioning, and distribution expectations.

## 2. Goals

- Create artifact matrix by OS.
- Define versioning and release notes.
- Document unsigned build limitations if applicable.
- Support reproducible-ish build process where feasible.

## 3. Non-Goals

- No paid code-signing requirement in early planning.
- No app store distribution requirement.
- No auto-update system.

## 4. Design

### Artifact Matrix

| OS | Candidate Artifacts |
|---|---|
| Linux | tar.gz, AppImage or distro package later |
| macOS | app bundle zip/dmg later |
| Windows | zip/msi later |

### Versioning

Use semantic versioning for public releases. Pre-1.0 versions may introduce breaking changes but must document data/file safety expectations.

### Release Notes Template

```text
Version
Highlights
Data integrity notes
Known limitations
Platform notes
Checksums
```

### Unsigned Build Policy

If builds are unsigned, documentation must clearly explain expected OS warnings and safe verification steps such as checksums.

## 5. Validation and Test Plan

- CI produces expected artifacts.
- Artifact launches smoke app where possible.
- Checksums generated.
- Release notes include known limitations.

## 6. Acceptance Criteria

- Users know which artifact to download.
- Release process does not imply unsupported trust guarantees.
- Packaging does not alter core file-preservation behavior.

## 7. Dependencies

- RFC-035
- RFC-038
- RFC-042

---

## Implementation Reminder

This RFC must preserve the project-wide invariant: editing one section must not rewrite unrelated Markdown source bytes unless the RFC explicitly describes and justifies a structural source transformation.
