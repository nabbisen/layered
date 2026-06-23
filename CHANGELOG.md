# Changelog

All notable changes to this project are documented in this file. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the
project adheres to [Semantic Versioning](https://semver.org/).

## [0.13.2] - 2026-06-09

### Changed

Plain-language and clarity improvements adopted from an external UX review,
selected to fit layered's audience (Markdown-literate writers) rather than
the review's broader "non-technical user" framing. Wholesale changes that
conflicted with the product identity — hiding the word "Markdown", a
touch-first 16px/44px redesign, loading-spinner buttons for instant local
operations — were intentionally not adopted.

- **Button and control labels clarified:**
  - "Edit" (the body-commit button) → **"Done"**. The button committed the
    current draft; labelling it "Edit" while already editing was confusing.
  - "Add Child" → **"Add section"**. "Child" is tree-structure jargon.
  - The `⋯` structural toggle now shows a visible **"Arrange"** label
    beside the glyph instead of being icon-only, closing a discoverability
    gap.
  - Unsaved-changes dialog: "Discard" → **"Leave without saving"**.

- **Error messages rewritten to be calmer and recovery-oriented:**
  - Open failure now suggests trying another file.
  - Save failure now reassures that the writing is safe and points to
    Save As, rather than just stating the failure.
  - "Heading level limit reached" → "This section can't move any deeper."
  - "No adjacent sibling to merge with" → "There's no nearby section to
    merge into."

- **Accessibility:** the per-child `×` delete button gained an `aria-label`
  ("Delete section") so screen readers announce its purpose instead of just
  reading the glyph.

## [0.13.1] - 2026-06-09

### Changed

- **"Add section" button is now always visible.** Previously it was hidden
  inside the `⋯` structural toolbar, requiring two clicks before a child
  section could be created. It is now a persistent `+ Add section` button at
  the bottom of the child-sections area in focus mode — visible on every
  focused section, whether it already has children or not. The structural
  toolbar (`⋯`) retains the rearrangement and merge operations but no longer
  contains Add section.

- **Per-child delete button.** Each child section card now has a `×` button
  that navigates into the child and immediately opens the existing delete-
  confirmation dialog. This removes the two-step "zoom in, then open ⋯ and
  Delete" flow. Delete still requires confirmation; the `×` is styled as a
  subtle secondary action and uses the danger colour on hover.

- **Outline tree auto-expands on Add section.** After a child section is
  created, `OutlinePane` detects the node-count increase in `use_effect` and
  expands the parent node in the `ItemTreeView` if it was collapsed, making
  the new section immediately visible in the outline.

## [0.13.0] - 2026-06-09

Minor release consolidating the 0.12.1–0.12.7 patch series into a single
tagged version for downstream testing. The detailed entries for each patch
remain below. Highlights of the series:

- **Outline rendering** moved to `dioxus-swdir-tree` v0.9's `ItemTreeView`
  (0.12.5), retiring the hand-rolled list.
- **"Less is more" UX pass** (0.12.6): structural toolbar collapsed behind a
  toggle, status bar trimmed, welcome tutorial removed.
- **Architecture cleanup** (0.12.7): `app.rs` 573→199 ELOC and `session.rs`
  474→307 ELOC via focused module splits; dual selection models unified;
  document statistics wired into the command palette.
- **Correctness fixes**: i18n sort-order and missing-key bugs, a clippy
  lint, and several stale-documentation corrections.

### Added

- Pure-logic unit tests for the desktop shell: keyboard shortcut mapping
  (`interpret_code`) and recent-files management (`AppSettings`). 158 tests
  total. Documented in `TESTING.md` why the Dioxus component/hook/end-to-end
  testing styles are intentionally not used.

## [0.12.7] - 2026-06-09

### Changed

All three "findings to track" from the v0.12.6 audit report resolved.

**A — ELOC guideline: over-500 and over-300 files split**

- `app.rs` (573 → 199 ELOC) split into four files. `Signal<T>: Copy` lets
  every action handler take an `AppCtx` struct (one bundled argument) instead
  of eight separate signals:
  - `app_ctx.rs` — `AppCtx` bundle, `Modal` enum, `sync_draft`,
    `commit_pending`
  - `actions.rs` — all file/session action handlers (`handle_load`,
    `handle_save`, `handle_open_guarded`, `handle_new_guarded`,
    `handle_unsaved_choice`, `handle_ext_modified_choice`,
    `handle_confirm_delete`, `handle_split_choice`)
  - `dispatch.rs` — keyboard command dispatch (`dispatch_command`) and
    palette dispatch (`dispatch_palette`)
  - `app.rs` — signal declarations, `use_callback` wrappers (one line
    each), sentinel intercept, and the `rsx!` render tree

- `session.rs` (474 → 307 ELOC) split into a submodule:
  - `session/mod.rs` — struct, constructors, accessors, navigation, edit ops
  - `session/structural.rs` — structural editing façade (RFC-023..026):
    `can_promote` / `promote_focused` / `demote_focused` / `move_focused` /
    `merge_focused_up` / `split_focused` / `delete_focused` and the six
    `can_*` guards
  - `session/outline_bridge.rs` — `OutlineNode`, `outline_nodes()`,
    `build_outline_node`

**B — Dual selection models unified**

- `selected_card: Signal<usize>` removed from `OutlinePane` props and its
  sync code removed. `ItemTreeView` (dioxus-swdir-tree) manages its own
  selection and keyboard navigation internally; `selected_card` now lives
  only in `OverviewPane` where it drives the main-canvas card highlight.

**C — Stats module wired to UI**

- "Show Statistics" command (`view.stats`) added to the command palette.
  Selecting it writes word count and section count to the status bar as a
  one-time message, making `EditorSession::stats()` reachable from the UI.
  `view.stats` i18n key added to both `en` and `ja` catalogs in correct
  sorted position.

### Fixed

- `i18n/en.rs` and `i18n/ja.rs`: `struct.toolbar.toggle` was out of sort
  order (before `struct.delete` instead of after `struct.split`), breaking
  the binary-search catalog invariant. `welcome.tagline` was never inserted
  (multi-line match failed silently). Both corrected; 47 i18n tests pass.
- `preview.rs`: collapsed two identical link-rendering branches (clippy
  `clippy::if_same_then_else`).

## [0.12.6] - 2026-06-09

### Changed

Three UI simplifications applying the "less is more" design principle —
users start as immature; advanced features should be discoverable, not
front-and-centre.

- **Structural toolbar collapsed by default.** The seven structural-editing
  buttons (Promote, Demote, Move ↑, Move ↓, Merge ↑, Split, Delete) are now
  hidden behind a single `⋯` toggle button in the focus editor. One click
  reveals the toolbar; another hides it. The toolbar state is local to the
  current focus session (resets when zooming out). Expert users still have
  full access; new users are no longer confronted with seven unfamiliar
  buttons on first use.

- **Status bar trimmed.** Word count, section count, and the newline-style
  label (LF/CRLF) have been removed from the always-visible status bar.
  Remaining: live status/error messages, unsaved-changes indicator, file
  name. The RFC-046 statistics module is retained in `layered-ui` for future
  use (e.g. a command-palette "Show Statistics" action).

- **Welcome screen tutorial removed.** The five-step onboarding list was
  shown on every launch, becoming noise for returning users. Removed.
  The title, tagline, Open/New buttons, and recent-files list remain.

## [0.12.5] - 2026-06-09

### Changed

- **Outline pane now uses `dioxus-swdir-tree` v0.9's `ItemTreeView`.**
  `dioxus-swdir-tree` v0.9.0 shipped RFC-012 (generic item tree) and
  RFC-013 (item tree drag-and-drop), which implement exactly what the
  feature request described. The hand-rolled `OutlinePane` for loop is
  replaced by `ItemTreeView<String>`:
  - Expand/collapse, keyboard navigation (Up/Down/Left/Right/Enter/Home/End),
    and incremental search are handled by the widget.
  - `ItemTree::set_tree` is called on every session change; key-based
    diffing preserves expansion state across pure body edits and resets it
    when the heading structure changes (expected behaviour).
  - Drag-and-drop is disabled (not enabled via `with_drag_and_drop`);
    structural editing remains through the existing toolbar buttons.
- `EditorSession::outline_nodes() -> OutlineNode` added to `layered-ui`:
  converts the full document `Outline` into a `OutlineNode` tree (plain
  `u64` keys + `String` titles) without exposing `layered-core` types to
  the desktop crate.
- `dioxus-swdir-tree = { version = "0.9", default-features = false }` added
  to `layered-desktop` dependencies (`default-style` disabled; the existing
  `assets/style.css` themes the outline pane instead).

## [0.12.4] - 2026-06-07

### Fixed

- **Outline pane showed nothing.** The `for` loop in `OutlinePane` used a
  `{ let …; rsx! { … } }` block as its body. In Dioxus 0.7 this pattern
  silently produces no output — the inner `rsx!` call inside a Rust block
  is not forwarded to the parent element. Fixed by removing the wrapper
  block and capturing per-iteration values directly in the event-handler
  closures (`let id = item.id; move |_| { … }`), which is the correct
  Dioxus 0.7 pattern.

## [0.12.3] - 2026-06-07

### Fixed

- **"New" button on the welcome screen did nothing.** `is_welcome` was
  computed as `source().is_empty() && !is_dirty()`, which remained true
  after "New" because a freshly created empty document satisfies both
  conditions. Fixed by adding a `document_open: bool` field to
  `EditorSession`:
  - `new_empty()` sets `document_open = false` — startup placeholder,
    keeps the welcome screen.
  - New method `new_document()` sets `document_open = true` — used by all
    three "New" code paths (direct click, save-then-new, discard-then-new),
    dismisses the welcome screen and shows the editor.
  - `is_welcome` is now simply `!session.document_open()`.

## [0.12.2] - 2026-06-07

### Fixed

- **Linux build failure**: added `libjavascriptcoregtk-4.1-dev` and
  `libssl-dev` to the required system package list in `PLATFORMS.md`,
  `getting-started.md`, and the `Cargo.toml` comment. Both packages are
  required by Dioxus 0.7's desktop renderer on Linux but were missing from
  the documentation.

### Changed

- **MSRV restored and corrected to 1.87**: `rust-version = "1.87"` is now
  set in the workspace `Cargo.toml`. The previous value of 1.85 was removed
  in v0.12.1 because it could not be verified; a full scan of the 631-package
  dependency tree (`cargo metadata`) found that `wit-bindgen 0.51` (a
  transitive dep of Dioxus 0.7) requires Rust 1.87. All other transitive
  dependencies state 1.85 or lower.

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
