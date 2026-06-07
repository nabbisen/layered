# Known Limitations

This page documents the intentional limitations and deferred features in the
current version of layered. Limitations that affect data safety are listed
first.

---

## Data Safety Limitations

### Raw Source Editing Is Read-Only

The Raw Markdown Source view (Ctrl+\`) shows the full document source but
does not currently support editing. Changes must be made through the Focus
Editor or an external text editor.

**Why:** Editing raw source requires re-indexing the entire document after
every commit, and handling the case where heading titles are changed during
a raw edit requires additional focus-remapping logic. This is deferred to a
future release.

**Workaround:** Open the file in a standard text editor (Vim, VS Code, etc.),
make your changes, then re-open the file in layered.

---

## Heading Style Limitations

### Setext Headings Cannot Be Promoted or Demoted

Setext headings (underlined with `===` or `---`) are displayed correctly in
the outline but cannot be promoted or demoted using the structural toolbar.

**Why:** ATX headings use a simple prefix change (`##` → `#`); Setext
headings require replacing the underline character on a different line, which
creates a more complex source range replacement.

**Workaround:** Use the Raw Source view to manually convert the Setext heading
to an ATX heading, then use promote/demote normally.

---

## Navigation Limitations

### Focus Does Not Return to Card After Zoom Out (WebView constraint)

When pressing Esc to zoom out, keyboard focus moves to the document body
rather than back to the specific heading card that was zoomed into.

**Why:** Programmatic focus management in the Dioxus WebView environment
requires JavaScript `element.focus()` calls that are not yet implemented in
this release.

**Workaround:** Press Tab to move focus into the outline panel, then use
arrow keys to navigate.

---

## Feature Limitations

These features are intentionally absent in this release:

| Feature | Status |
|---------|--------|
| Plugin / extension system | Deferred — see Future RFC-C |
| AI writing assistance | Deferred — see Future RFC-D |
| Real-time collaboration | Deferred — see Future RFC (not started) |
| Cloud synchronization | Out of scope by design |
| Mobile application | Out of scope by design |
| Web / browser version | Out of scope by design |
| Multi-document workspace | Deferred — see Future RFC-B |
| TUI / CLI companion | Deferred — see Future RFC-F |

---

## Platform Limitations

See `PLATFORMS.md` for the full platform support matrix. Key notes:

- **Unsigned macOS builds**: Gatekeeper will warn. Right-click → Open to bypass.
- **Linux Wayland**: Runs via XWayland; native Wayland support follows upstream.
- **File dialogs on headless Linux**: May silently return no file if no portal is available.

---

## Performance Limitations

- Documents larger than ~50 000 words may cause noticeable pause after
  committing an edit (re-indexing is synchronous and full).
- Very deep heading trees (>50 levels of nesting, which is unusual) are
  supported but not performance-optimised.

---

## What Is Not Limited

- Markdown source is **never silently modified** by layered. If you open a
  file, navigate around, and close without saving, the file is unchanged.
- All structural operations (promote, demote, move, split, delete, merge)
  are **undoable** via Ctrl+Z.
- Saved files are **standard UTF-8 Markdown**. They open correctly in any
  text editor, with no layered-specific metadata.
