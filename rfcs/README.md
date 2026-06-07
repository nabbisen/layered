# layerd RFCs

Design documents for **layerd — Layer EDitor**, governed by the lifecycle
policy in [RFC 000](./done/000-rfc-lifecycle-policy.md): an RFC's folder is
the source of truth for its state, numbers are permanent, and completed
RFCs are never deleted.

Core invariant protected by every RFC:

> The canonical document is the raw Markdown source text. The outline tree
> is a derived navigational index. Editing one section must not rewrite
> unrelated source bytes.

## Implemented

| ID | Title | Shipped in |
|----|-------|------------|
| 000 | [RFC lifecycle policy](./done/000-rfc-lifecycle-policy.md) | —(policy) |
| 001 | [Project Architecture and Crate Boundaries](./done/001-project-architecture-and-crate-boundaries.md) | v0.7.0 |
| 002 | [Canonical Markdown Text Model](./done/002-canonical-markdown-text-model.md) | v0.7.0 |
| 003 | [Heading Indexing Spike](./done/003-heading-indexing-spike.md) | v0.7.0 |
| 004 | [Source-Preserving Section Replacement](./done/004-source-preserving-section-replacement.md) | v0.7.0 |
| 005 | [Core Document API](./done/005-core-document-api.md) | v0.7.0 |
| 006 | [Node Identity and Range Semantics](./done/006-node-identity-and-range-semantics.md) | v0.7.0 |
| 007 | [Markdown Heading Tree Construction](./done/007-markdown-heading-tree-construction.md) | v0.7.0 |
| 008 | [Core Edit Operation Model](./done/008-core-edit-operation-model.md) | v0.7.0 |
| 009 | [Core Error and Validation Model](./done/009-core-error-and-validation-model.md) | v0.7.0 |
| 043 | [GUI Internationalization and Locale Policy](./done/043-gui-internationalization-and-locale-policy.md) | v0.7.0 |
| 044 | [Undo and Redo for Body Edit Operations](./done/044-undo-and-redo-for-body-edit-operations.md) | v0.7.0 |

## Proposed

Grouped by roadmap milestone. Review before implementation; any behavior
touching source preservation, file lifecycle, or structural editing must be
converted into tests before its RFC moves to `done/`.

| ID | Title | Milestone |
|----|-------|-----------|
| 010 | [Desktop Application Shell](./proposed/010-desktop-application-shell.md) | M2 — Basic Desktop UX |
| 011 | [Outline and Overview UI](./proposed/011-outline-and-overview-ui.md) | M2 — Basic Desktop UX |
| 012 | [Focus Editor UI](./proposed/012-focus-editor-ui.md) | M2 — Basic Desktop UX |
| 013 | [Breadcrumb and Hierarchical Navigation](./proposed/013-breadcrumb-and-hierarchical-navigation.md) | M2 — Basic Desktop UX |
| 014 | [Basic Keyboard Interaction](./proposed/014-basic-keyboard-interaction.md) | M2 — Basic Desktop UX |
| 015 | [File Open and Save Lifecycle](./proposed/015-file-open-and-save-lifecycle.md) | M3 — File Lifecycle and Recovery |
| 016 | [Dirty State and Unsaved Change Protection](./proposed/016-dirty-state-and-unsaved-change-protection.md) | M3 — File Lifecycle and Recovery |
| 017 | [Raw Markdown Escape Hatch](./proposed/017-raw-markdown-escape-hatch.md) | M3 — File Lifecycle and Recovery |
| 018 | [Line Endings, Encoding, and File Integrity](./proposed/018-line-endings-encoding-and-file-integrity.md) | M3 — File Lifecycle and Recovery |
| 019 | [Focus History and Back/Forward Navigation](./proposed/019-focus-history-and-back-forward-navigation.md) | M4 — Navigation and Search |
| 020 | [Sibling and Depth Navigation](./proposed/020-sibling-and-depth-navigation.md) | M4 — Navigation and Search |
| 021 | [Search and Result Navigation](./proposed/021-search-and-result-navigation.md) | M4 — Navigation and Search |
| 022 | [Command Palette and Command Registry](./proposed/022-command-palette-and-command-registry.md) | M4 — Navigation and Search |
| 023 | [Promote and Demote Heading Operations](./proposed/023-promote-and-demote-heading-operations.md) | M5 — Structural Editing |
| 024 | [Move Section Operations](./proposed/024-move-section-operations.md) | M5 — Structural Editing |
| 025 | [Split, Merge, and Delete Section Operations](./proposed/025-split-merge-and-delete-section-operations.md) | M5 — Structural Editing |
| 026 | [Structural Edit Validation and Conflict Rules](./proposed/026-structural-edit-validation-and-conflict-rules.md) | M5 — Structural Editing |
| 027 | [Semantic HTML and ARIA Model](./proposed/027-semantic-html-and-aria-model.md) | M6 — Accessibility and Usability Hardening |
| 028 | [Keyboard Focus and Navigation Accessibility](./proposed/028-keyboard-focus-and-navigation-accessibility.md) | M6 — Accessibility and Usability Hardening |
| 029 | [Accessible Editor Status and Error Feedback](./proposed/029-accessible-editor-status-and-error-feedback.md) | M6 — Accessibility and Usability Hardening |
| 030 | [Visual Clarity, Contrast, and Motion Policy](./proposed/030-visual-clarity-contrast-and-motion-policy.md) | M6 — Accessibility and Usability Hardening |
| 031 | [Performance Targets and Measurement Plan](./proposed/031-performance-targets-and-measurement-plan.md) | M7 — Performance and Large Document Readiness |
| 032 | [Re-index Strategy and Debounce Lifecycle](./proposed/032-re-index-strategy-and-debounce-lifecycle.md) | M7 — Performance and Large Document Readiness |
| 033 | [Render Boundary and State Update Policy](./proposed/033-render-boundary-and-state-update-policy.md) | M7 — Performance and Large Document Readiness |
| 034 | [Large Document Test Fixtures](./proposed/034-large-document-test-fixtures.md) | M7 — Performance and Large Document Readiness |
| 035 | [Cross-Platform Desktop Runtime Policy](./proposed/035-cross-platform-desktop-runtime-policy.md) | M8 — Cross-Platform Delivery |
| 036 | [Settings and Recent Files Storage](./proposed/036-settings-and-recent-files-storage.md) | M8 — Cross-Platform Delivery |
| 037 | [Packaging and Release Artifacts](./proposed/037-packaging-and-release-artifacts.md) | M8 — Cross-Platform Delivery |
| 038 | [Platform Smoke Test Matrix](./proposed/038-platform-smoke-test-matrix.md) | M8 — Cross-Platform Delivery |
| 039 | [Error Handling and User-Facing Failure Modes](./proposed/039-error-handling-and-user-facing-failure-modes.md) | M9 — Production Readiness |
| 040 | [Test Strategy and Regression Policy](./proposed/040-test-strategy-and-regression-policy.md) | M9 — Production Readiness |
| 041 | [User Documentation and Onboarding](./proposed/041-user-documentation-and-onboarding.md) | M9 — Production Readiness |
| 042 | [Release Readiness Criteria](./proposed/042-release-readiness-criteria.md) | M9 — Production Readiness |

## Archive

No RFCs have been withdrawn or superseded yet.

## Conventions

- Filenames: `NNN-slug.md`; numbers assigned at creation and never reused.
- Status field mirrors the folder and is updated in the same commit as a move.
- New RFCs start in `proposed/`; the next free number is 045.

Run `scripts/check-rfcs.sh` from the repository root to verify these
invariants.
