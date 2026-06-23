# Structural Editing

omriss lets you reorganise your document's heading hierarchy without touching
the body text. The **Arrange** button below the editor actions reveals
Promote, Demote, Move ↑, Move ↓, Merge Up, and Delete for the focused
section. **Add section** is always visible at the bottom of the child-sections
area.

---

## Promote and Demote

**Promote** raises the heading level by one step (for example H3 → H2),
making the section a higher-level concept in the hierarchy. **Demote** lowers
it (H2 → H3). Only the heading marker changes; the body text and all child
sections are untouched.

Buttons are disabled when the operation would be invalid:

- You cannot promote an H1 heading (it is already at the top level).
- You cannot demote an H6 heading.
- Setext-style headings (`Title\n======`) are not supported; use the raw
  Markdown view to convert them to ATX headings first.

> **Child headings are not adjusted.** If you promote a section from H2 to
> H1, any H3 children become one step further removed in the hierarchy. Use
> additional promote/demote operations to bring them into alignment.

---

## Move Up / Move Down

These swap the focused section with the adjacent sibling in source order,
keeping the full subtree (all children and descendants) intact. The buttons
are disabled when the section is already at the top or bottom of its siblings.

---

## Merge Up

Removes the focused section's heading line and makes its body a continuation
of the previous sibling's body. The section's children become children of the
previous sibling.

This operation cannot be undone by clicking Merge Up again — use **Ctrl+Z**
(Undo) to restore the heading.

---

## Add section

Opens a dialog where you enter a title. A new child heading is appended at
the end of the focused section's body at the next depth level. You can then
move or redistribute body text by editing normally.

---

## Delete Section

Opens a confirmation dialog showing the section title and the number of child
sections that will also be removed. Click **Delete** to proceed or **Cancel**
to abort. The deletion is reversible with **Ctrl+Z** (Undo).

---

## Undo

All structural operations are recorded in the same undo history as body
edits. **Ctrl+Z** reverses the most recent committed operation; **Ctrl+Y**
re-applies it. Structural edits are byte-exact: undo restores the source
character-for-character.
