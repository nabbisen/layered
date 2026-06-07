# Architecture

This page documents the layerd system architecture: how state flows between
crates, where renders are triggered, and which components own which data.

---

## Crate Boundaries (RFC-001)

```
layerd-core      pure Rust, no UI dependency
    ↓
layerd-ui        pure Rust, no Dioxus dependency
    ↓
layerd-desktop   Dioxus 0.6 desktop shell
```

**layerd-core** owns the canonical document model: the source text buffer,
the derived outline index, and all edit operations including undo/redo. It
has no knowledge of windows, signals, or the file system.

**layerd-ui** owns editor session state: the current view mode (overview vs
focus), focus history, search, the command registry, and the file text
profile. It depends on `layerd-core` but not on Dioxus. Its types are
testable with `cargo test` on any host.

**layerd-desktop** owns the Dioxus component tree, file I/O, keyboard
handling, and the signal graph. It has no business logic; it maps user
gestures to `EditorSession` method calls.

---

## Render Boundary (RFC-033)

The most important performance contract in the system:

> **Typing a character in the focus editor must not cause the outline pane
> or breadcrumb to re-render.**

This is enforced by signal ownership:

| State | Signal owner | Update trigger |
|-------|-------------|----------------|
| Document source + outline | `session: Signal<EditorSession>` | commit / open / save / structural op |
| Current draft body | `draft: Signal<String>` | every keystroke |
| Status message | `status: Signal<String>` | operation result |
| Selected outline card | `selected_card: Signal<usize>` | keyboard nav |
| Search open | `search_open: Signal<bool>` | Ctrl+F |
| Palette open | `palette_open: Signal<bool>` | Ctrl+P |
| Modal state | `modal: Signal<Modal>` | operation guard |

The `draft` signal is **local** — it never enters `session` until the user
commits (blur, save, navigation). Components that only read `session` (the
outline pane, the breadcrumb, the status bar) are therefore not dirtied by
keystrokes.

The `session` signal is written **once per committed edit**, not per
keystroke. This is the main debounce boundary (RFC-032).

---

## Re-index Lifecycle (RFC-032)

```
User types in textarea
    → draft.set(new_value)          ← local signal only; no re-index

User blurs / saves / navigates
    → session.write().commit_focused_body(snapshot, draft)
        → document.replace_section_body(…)
            → apply_replacement(range, new_text)
                → re-index (full, synchronous)
                → increment revision
                → record undo entry
    → sync_draft(session, draft)    ← draft re-synced from committed source
```

Re-indexing is **synchronous and full** for M7. The document is small enough
that this is imperceptible for typical writing sessions. If profiling reveals
latency on large documents (> 10 000 words), the next step is an incremental
parser — see RFC-032 §4 for the criteria.

---

## State Ownership in Components

| Component | Reads | Writes |
|-----------|-------|--------|
| `Toolbar` | session (dirty/file), locale | via callbacks (open/save) |
| `OutlinePane` | session (children), locale, selected_card | selected_card (via parent) |
| `OverviewPane` | session (children), locale | session (via focus), draft |
| `FocusEditor` | session (snapshot), locale, draft | draft (every key), session (commit, structural) |
| `Breadcrumb` | session (path), locale, draft | session (jump) |
| `SearchPanel` | session (search), locale | session (navigate result) |
| `CommandPalette` | locale | via on_execute callback |
| `StatusBar` | session (dirty/profile/file), locale, status | via on_save_as callback |

**Anti-patterns to avoid** (RFC-033 §4):
- Passing `session.read().source()` as a prop to a component that renders per keystroke.
- Computing the full outline in a closure that runs on every `draft` change.
- Storing duplicate copies of the document tree in additional signals.
- Calling `session.write()` from inside a `draft` signal watcher.

---

## Undo/Redo Model (RFC-044)

All edit operations — including structural ones — go through
`Document::apply_replacement`. This method records an `EditRecord` capturing
the replaced byte range, the old text, and the new text. Undo restores the
old text via another `apply_replacement` call, giving byte-exact restoration.

Structural moves record a full-document replacement (range `[0, source.len())`)
so the before/after source can be diffed exactly by the undo machinery.

---

## Accessibility Architecture (RFC-027..030)

The component-to-landmark mapping:

| HTML element | Component | ARIA |
|---|---|---|
| `<header role="toolbar">` | `Toolbar` | groups file + edit controls |
| `<aside>` | `OutlinePane` | outline navigation landmark |
| `<main>` | `FocusEditor` / `OverviewPane` | primary workspace |
| `<nav>` | `Breadcrumb` | section path navigation |
| `<footer>` | `StatusBar` | live status region |

Status updates use `aria-live="polite"` for informational messages and
`aria-live="assertive"` for errors, so screen readers interrupt for failures
but not for routine saves.

The focus editor textarea receives `autofocus` when the component mounts
(RFC-028). Programmatic focus management beyond this is constrained by the
WebView platform; future work may use `use_eval` to call
`element.focus()` in JavaScript.
