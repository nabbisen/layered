# Working in Layers

## The outline is the map

The outline pane lists top-level sections. Numbers in parentheses show how
many subsections each one holds.

## Focus is the work surface

Focusing a section shows:

- the **breadcrumb** path from the document root — click any crumb to jump up;
- the section **body** in an editor (the body excludes subsections, so a
  parent's text and its children never blur together);
- **subsection cards** — click to descend one more layer;
- a **structural toolbar** for promote, demote, move, merge, split, and delete.

## Back and forward

Focus navigation keeps browser-style history. **Back** returns to where you
were; **Forward** retraces. Jumping somewhere new starts a fresh branch.

If an edit removes a section you had visited, dead history entries are
dropped automatically and the view falls back to the outline.

## Searching

Press **Ctrl+F** to open the search panel. Search runs case-insensitively
across either the focused section body or the whole document. Selecting a
result navigates directly to the matching section.

## Preview

Press **Ctrl+Shift+P** (or the **Preview** button in the editor) to see
the focused section body rendered as HTML. The view is read-only — switching
back to edit mode restores the textarea with the draft unchanged.

## Word count

The status bar shows the word count for the focused section and for the
whole document, along with the total section count. The count updates after
each committed edit, not on every keystroke.
