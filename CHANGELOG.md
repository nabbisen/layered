# Changelog

All notable changes to this project are documented in this file. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the
project adheres to [Semantic Versioning](https://semver.org/).

## [0.12.1] - 2026-06-07

### Changed

- **Renamed: `layerd` → `layered` throughout the entire codebase.** The
  previous name was a typo/abbreviation that caused confusion. All crate
  names (`layered-core`, `layered-ui`, `layered-desktop`), Rust module
  paths, the app title in both i18n catalogs, the OS config directory
  (`~/.config/layered/`), and all documentation files have been updated.
  110 files changed; no functional behaviour modified.

## [0.12.0] - 2026-06-07

Post-MVP expansion — two Future RFC items promoted to implemented:

### Added

**RFC-045: Markdown Preview Pane**

- A **Preview** button appears in the editor-actions bar when a section is
  focused. Clicking it (or pressing **Ctrl+Shift+P**) commits the pending
  draft and switches to a rendered HTML view of the section body.
- The rendered preview uses a manual pulldown-cmark event walker — no
  additional C build-script features required. Supported: headings H1–H6,
  paragraphs, bold, italic, strikethrough, inline code, fenced code blocks,
  unordered and ordered lists, block quotes, links, images, tables,
  task-list checkboxes, horizontal rules, hard/soft breaks, raw HTML pass-through.
- Preview is read-only. The source-preservation invariant is unaffected:
  `section_html` and `document_html` are pure functions over the outline.
- `PreviewPane` component with `role="region"` and accessible back-to-edit
  button. Scoped `.preview-body` CSS styles the rendered Markdown.
- 7 new tests in `crates/layered-core/src/preview.rs` covering bold, headings,
  empty body, unknown node, Japanese text, code fences, and HTML escaping.

**RFC-046: Document Statistics**

- The status bar now shows word count and section count for the open document.
  When a section is focused, the focused-section word count is shown first:
  `42 words / 1 234 words · 17 sections`.
- `layered_ui::stats` module with `DocumentStats`, `word_count`, and
  `compute_stats`. Statistics recompute only on committed edits (not per
  keystroke), following the render-boundary policy from RFC-033.
- 6 new tests covering zero, normal, leading/trailing whitespace, focused
  scope, section count, and post-edit update.

### Changed

- `AppSettings::remove_recent` and `clear_recent` marked `#[allow(dead_code)]`
  (reserved for future clear-recents UI).
- Workspace version bumped to 0.12.0.



## [0.11.0] - 2026-06-07

Phase G release (M8 — Cross-Platform Delivery, RFC-035..038; M9 — Production
Readiness, RFC-039..042). All 45 design RFCs are now implemented.

### Added

**Cross-platform delivery (M8):**

- **App settings with persistent recent files** (RFC-036): `AppSettings`
  stored as TOML in the OS config directory. Recent files (up to 10) are
  loaded on startup and shown on the welcome screen. Opening a file adds it
  to the list automatically; stale paths are filtered on load. Backed by
  `dirs`, `serde`, and `toml` workspace dependencies.
- **Recent files welcome screen** (RFC-036 / RFC-041): The welcome screen now
  shows a five-step onboarding guide and the recent-files list with file name
  and directory. Clicking an item opens the file immediately.
- **Platform documentation** (RFC-035): `PLATFORMS.md` documents the support
  matrix, required Linux system packages, keyboard modifier policy, file dialog
  backend, config directory paths, and known constraints per platform.
- **Packaging and release checklist** (RFC-037 / RFC-038 / RFC-042):
  `RELEASE_CHECKLIST.md` covers pre-release gates, data-integrity tests,
  per-platform smoke test workflow, artifact matrix with checksum instructions,
  unsigned build policy, and the required sign-off form.

**Production readiness (M9):**

- **Structured open-error dialog** (RFC-039): `OpenOutcome::Failed` now carries
  a plain-language `cause` string. File-open failures show an `ErrorDialog`
  modal with the specific reason (permission denied, not valid UTF-8, file not
  found) instead of a bare status-bar message. `ErrorDialog` is a reusable
  component for future error surfaces.
- **`open_markdown_path`** (RFC-039): opens a file at a known path without
  displaying a dialog, used by the recent-files list and testable in isolation.
- **Test strategy documentation** (RFC-040): `TESTING.md` formalises the test
  pyramid, fixture catalog, regression policy (reproduce → classify →
  fix → keep test), and CI requirements.
- **4 new regression tests** (RFC-040): empty document, whitespace-only
  document, edit-last-preserves-first, and UTF-8 multibyte body edit are now
  in `tests/source_preservation.rs`.
- **Known limitations page** (RFC-041): `docs/src/known-limitations.md`
  documents read-only raw source, setext promote/demote constraint, focus-return
  WebView limitation, deferred features, and what is explicitly not limited.
- **Release policy in README** (RFC-042): public releases require explicit
  product-owner sign-off; unsigned build verification instructions added.

### Changed

- `WelcomeScreen` gains `recent_files: Signal<Vec<String>>` and
  `on_open_recent: EventHandler<String>` props; all callers updated.
- Workspace version bumped to 0.11.0.
- `SUMMARY.md` updated with Known Limitations and Architecture pages.



## [0.10.0] - 2026-06-07

Sixth + seventh milestone release (M6 — Accessibility Hardening, RFC-027..030;
M7 — Performance and Large Document Readiness, RFC-031..034).

### Added

**Accessibility (M6):**

- **Semantic landmark regions** (RFC-027): toolbar rendered as `<header
  role="toolbar">`, the outline side-panel as `<aside>`, keeping `<main>` for
  the focus editor and `<footer>` for the status bar. Interactive elements
  across every component carry explicit accessible names.
- **Keyboard focus after zoom** (RFC-028): the body editor textarea now
  receives `autofocus` when a section is entered, so keyboard-only users land
  directly in the editor without extra Tab presses.
- **Polite vs assertive live regions** (RFC-029): the status bar now uses
  `aria-live="assertive"` for error keys (anything starting with `error.`) so
  screen readers interrupt to announce failures, while save confirmations and
  status updates remain `polite`. Save-failure status now includes an inline
  **Save As** recovery affordance rendered as a button (RFC-029 error pattern).
- **Light theme via `prefers-color-scheme: light`** (RFC-030): CSS custom
  properties remap the full token set to a light palette automatically.
- **Reduced-motion support** (RFC-030): `@media (prefers-reduced-motion:
  reduce)` disables all transitions and animations site-wide.
- **Enhanced focus ring** (RFC-030): `:focus-visible` rule now applies `!important`
  to ensure visibility overrides component-level styles. Focus ring is visible
  in both light and dark themes at sufficient contrast.
- `dirty-indicator` in toolbar now carries `aria-label` for screen readers.

**Performance and large-document readiness (M7):**

- **Three new test fixtures** (RFC-034): `large-10k-words.md` (~15 000 words,
  deterministically generated), `academic-paper.md` (realistic academic
  structure with nested sections and references), `technical-rfc.md` (RFC-style
  document with code fences and tables in body ranges). All fixtures are
  version-controlled and covered by golden tests.
- **Fixture catalog** (RFC-034): 11 new tests in
  `crates/layered-core/tests/fixture_catalog.rs` verify outline shape, heading
  count, round-trip byte-preservation, and source integrity across every
  fixture.
- **Criterion benchmarks** (RFC-031): `crates/layered-core/benches/indexing.rs`
  measures parse+index, section body replacement, promote, move, and split on
  small, medium, and large fixtures. Run with `cargo bench -p layered-core`.
- **Architecture documentation** (RFC-033): `docs/src/architecture.md` records
  the render boundary contract, state ownership table, re-index lifecycle, and
  anti-patterns. Added to `SUMMARY.md` alongside a new structural-editing
  user guide page.

### Changed

- `StatusBar` gains an `on_save_as: EventHandler<()>` prop for the inline
  recovery affordance; all callers updated.
- Workspace version bumped to 1.0.0.



## [0.9.0] - 2026-06-07

Fifth milestone release (M5 — Structural Editing, per RFCs 023–026).

### Added

- **Promote / Demote heading** (RFC-023): raise or lower a section's ATX
  heading level by one step (`#`→`##` or vice-versa). Guards reject H1
  promote, H6 demote, and Setext headings (with a clear message directing
  the user to convert via raw view). Only the heading marker bytes change;
  all body text, child sections, siblings, and unrelated bytes are preserved
  exactly.
- **Move section up / down** (RFC-024): swap a section with its previous or
  next sibling. The full subtree (`full_range` — heading + body +
  descendants) is extracted and reinserted as a single source-text operation.
  Cyclic moves (into own descendants) and self-moves are rejected with typed
  errors before any mutation.
- **Delete section** (RFC-025): removes a section's `full_range`. A
  confirmation dialog (RFC-026 guard) displays the title and child count
  before the user can proceed. Fully undoable via Ctrl+Z.
- **Add child section (split)** (RFC-025): a dialog collects the new
  section title; the heading is inserted at the end of the focused body,
  splitting off a new child. Undo restores the original body.
- **Merge up** (RFC-025): removes a section's heading line, making its body
  a continuation of the previous sibling's body. Undo is byte-exact.
- **Structural edit validation framework** (RFC-026): `StructuralEditError`
  enum centralises preflight rejections: `RevisionMismatch`, `StaleNode`,
  `InvalidLevel`, `CannotMoveIntoDescendant`, `CannotDeleteRoot`,
  `NoAdjacentSibling`, `UnsupportedHeadingStyle`, `InvalidSplitOffset`.
  Every structural op rolls back automatically on re-index failure, preserving
  the pre-edit source.
- `layered_core::structural` module exposed as `pub`; `MoveTarget` and
  `StructuralEditError` re-exported from `layered_ui`.
- 23 new golden tests in `tests/structural_ops.rs` covering every operation,
  each error variant, undo round-trips, and byte-preservation invariants.
- 20 new i18n keys for structural ops, dialogs, and error messages (en + ja).



## [0.8.0] - 2026-06-07

Fourth milestone release (M4 — Navigation and Search, per RFCs 019–022).

### Added

- **Sibling and depth navigation** (RFC-020): Parent / First Child / Prev /
  Next buttons appear in the focus editor beneath the breadcrumb and title.
  Clicking any of them commits the pending draft first, then navigates.
  `EditorSession` exposes `navigate_parent`, `navigate_first_child`,
  `navigate_prev_sibling`, `navigate_next_sibling` and `sibling_info`.
- **Whole-document and section-scoped search** (RFC-021): `Ctrl+F` opens a
  slide-in search panel. Query is case-insensitive and UTF-8-safe; results are
  grouped by section path with a preview snippet. Selecting a result focuses
  the containing section and closes the panel. `layered_ui::search` module
  provides `search_document` / `search_section` as pure functions over
  `Document`.
- **Command palette** (RFC-022): `Ctrl+P` opens a filterable command list
  drawn from the static `layered_ui::commands::COMMANDS` registry. Each entry
  shows the command title and default shortcut. Selecting a command executes
  the corresponding app action. `filter_commands` is testable with a mock
  localizer.
- **Focus history with stale-node reporting** (RFC-019): `Alt+←` / `Alt+→`
  back/forward history was already implemented; this release adds non-blocking
  status feedback (`nav.stale_section`) when a history target no longer exists
  after a document edit. `EditorSession::prune_and_report` returns `true` when
  pruning occurred so the UI can surface the message.
- `Esc` now also dismisses the search panel and command palette before
  triggering zoom-out.
- `layered_ui::navigation` module with `SiblingInfo` and `sibling_info()`.
- `layered_ui::search` module with `SearchMatch`, `search_document`,
  `search_section` and 5 tests including a UTF-8 range-validity check.
- `layered_ui::commands` module with `CommandSpec`, `COMMANDS` and
  `filter_commands`; 3 unit tests.
- 15 new i18n keys in both English and Japanese catalogs (search, palette,
  navigation labels, stale-node message).

### Changed

- Workspace version bumped to 0.8.0.
- `app.rs` wired with search/palette overlay signals; all signal mutations
  continue to follow the `let mut sig = sig` shadowing pattern required by
  Dioxus 0.6 `Writable::set(&mut self)`.

## [0.2.0] - 2026-06-07

Second milestone release (M2 — Basic Desktop UX, per RFCs 010–014).

### Added

- **Desktop application shell** (RFC-010): welcome screen for new sessions,
  dirty indicator `●` in the toolbar, Save As button, document name display,
  and status bar covering ready/saved/unsaved/error states.
- **Outline and overview UI** (RFC-011): heading cards in the main canvas
  show level badges (H1…H6), child counts, and keyboard-selected highlight.
  Arrow-key navigation + Enter to zoom into any visible card; empty/headingless
  document state with hint text.
- **Focus editor** (RFC-012): breadcrumb header, section title with level
  label, textarea with `aria-label`, local-dirty indicator `●`, commit on
  blur and on the Edit button. Failed commits keep the draft text so the user
  can see and recover their unsaved work.
- **Breadcrumb navigation** (RFC-013): `<nav aria-label>` with `aria-current`
  on the current segment; long paths collapse to root › … › parent › current;
  clicking any ancestor navigates and commits pending draft.
- **Keyboard interaction** (RFC-014): Ctrl/Cmd+O/S/Shift+S, Ctrl/Cmd+Z/Y,
  Alt+←/→, Esc (commit + zoom out), Enter (zoom in from overview), ↑/↓
  (card selection) — all wired through a pure `interpret()` function and a
  global `onkeydown` handler on the app div.
- **Outline side panel** (RFC-011): left-panel `<nav role="listbox">` with
  roving tabindex; keyboard Enter/Space to zoom; Up one level button when
  focused; keyboard hint for sighted users.
- Keyboard shortcut reference page added to the mdBook user guide.

### Changed

- `app.rs` refactored to `use_callback` pattern (Dioxus 0.6 `Writable` trait
  uses `&mut self`; `let mut sig = sig` shadowing inside callbacks makes
  closures `Fn` and fully `Copy`-shareable).
- `file_dialog.rs` handles open/save I/O; `keyboard.rs` is pure and
  dependency-free from editor state.

## [0.7.0] - 2026-06-07

First milestone release (M0 "Core Document Engine" + M1 "Layered Editing MVP"
foundations, per the roadmap and RFCs 001–009, 043, 044).

### Added

- `layered-core`: canonical-text document model with derived outline index
  over `pulldown-cmark` (ATX + Setext headings, code fences and YAML/TOML
  front matter excluded), ordinal-path `NodeId`s stable across body edits,
  byte-exact section-body replacement with optimistic revision checking,
  and bounded byte-exact undo/redo.
- Golden integration suite: 13 fixture documents (Japanese text, CRLF,
  duplicate titles, skipped heading levels, HTML blocks, front matter,
  missing trailing newline, …) verified for source preservation and
  undo/redo round-trips on every section.
- `layered-ui`: `EditorSession` facade (content-based dirty tracking,
  focused-body commits, dead-focus pruning after structural edits),
  browser-style focus navigation history, and i18n catalogs (English,
  Japanese) with graceful fallback.
- `layered-desktop`: Dioxus desktop shell — outline pane, focus editor with
  breadcrumbs and subsection cards, undo/redo/back/forward toolbar, open/save
  dialogs, runtime language switching.
- Project documentation: README, mdBook user guide skeleton, 44 RFCs under
  the lifecycle policy.
## [0.3.0] - 2026-06-07

Third milestone release (M3 — File Lifecycle and Recovery, per RFCs 015–018).

### Added

- **Raw Markdown source view** (RFC-017): Ctrl+` toggles a read-only overlay
  displaying the exact canonical text with line count. Back to Structure returns
  to the previous outline or focus mode. Status bar shows a "Raw Markdown
  Source" badge when active.
- **Unsaved changes guard** (RFC-016): opening or creating a new document while
  the current document is dirty shows a three-button dialog (Save / Discard /
  Cancel). Save commits pending draft, saves to disk, then proceeds only on
  success.
- **External modification detection** (RFC-015): when saving, if the file on
  disk has a newer mtime than when it was last written by layered, a dialog
  offers Overwrite / Save As / Cancel before touching the disk.
- **Atomic save** (RFC-015, NFR-REL-003): saves write through a temp file
  then rename, so a crash mid-write cannot corrupt the original.
- **UTF-8 BOM preservation** (RFC-018): files with a UTF-8 BOM are opened with
  the BOM stripped internally; it is re-prepended on save.
- **Line ending detection** (RFC-018): `FileTextProfile` detects LF / CRLF /
  Mixed at open time; the status bar shows the policy label.
- **`EditorSession::open_with_profile`**: desktop crate passes pre-detected
  profile on open rather than re-running detection in the session.
- `layered_ui::file_profile` module exported as public API.
- Keyboard reference page updated with Ctrl+` shortcut.
