# Changelog

All notable changes to this project are documented in this file. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the
project adheres to [Semantic Versioning](https://semver.org/).

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
  the containing section and closes the panel. `layerd_ui::search` module
  provides `search_document` / `search_section` as pure functions over
  `Document`.
- **Command palette** (RFC-022): `Ctrl+P` opens a filterable command list
  drawn from the static `layerd_ui::commands::COMMANDS` registry. Each entry
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
- `layerd_ui::navigation` module with `SiblingInfo` and `sibling_info()`.
- `layerd_ui::search` module with `SearchMatch`, `search_document`,
  `search_section` and 5 tests including a UTF-8 range-validity check.
- `layerd_ui::commands` module with `CommandSpec`, `COMMANDS` and
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

## [0.1.0] - 2026-06-07

First milestone release (M0 "Core Document Engine" + M1 "Layered Editing MVP"
foundations, per the roadmap and RFCs 001–009, 043, 044).

### Added

- `layerd-core`: canonical-text document model with derived outline index
  over `pulldown-cmark` (ATX + Setext headings, code fences and YAML/TOML
  front matter excluded), ordinal-path `NodeId`s stable across body edits,
  byte-exact section-body replacement with optimistic revision checking,
  and bounded byte-exact undo/redo.
- Golden integration suite: 13 fixture documents (Japanese text, CRLF,
  duplicate titles, skipped heading levels, HTML blocks, front matter,
  missing trailing newline, …) verified for source preservation and
  undo/redo round-trips on every section.
- `layerd-ui`: `EditorSession` facade (content-based dirty tracking,
  focused-body commits, dead-focus pruning after structural edits),
  browser-style focus navigation history, and i18n catalogs (English,
  Japanese) with graceful fallback.
- `layerd-desktop`: Dioxus desktop shell — outline pane, focus editor with
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
  disk has a newer mtime than when it was last written by layerd, a dialog
  offers Overwrite / Save As / Cancel before touching the disk.
- **Atomic save** (RFC-015, NFR-REL-003): saves write through a temp file
  then rename, so a crash mid-write cannot corrupt the original.
- **UTF-8 BOM preservation** (RFC-018): files with a UTF-8 BOM are opened with
  the BOM stripped internally; it is re-prepended on save.
- **Line ending detection** (RFC-018): `FileTextProfile` detects LF / CRLF /
  Mixed at open time; the status bar shows the policy label.
- **`EditorSession::open_with_profile`**: desktop crate passes pre-detected
  profile on open rather than re-running detection in the session.
- `layerd_ui::file_profile` module exported as public API.
- Keyboard reference page updated with Ctrl+` shortcut.
