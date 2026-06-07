# How layerd Treats Your Text

The raw Markdown text **is** the document. The outline you navigate is a
derived index, rebuilt after each committed edit — it is never saved, and
the document is never regenerated from it.

When you commit a body edit, layerd splices exactly that byte range of the
file. Everything else is preserved bit-for-bit:

- spacing quirks, blank-line conventions, missing trailing newlines;
- CRLF and LF line endings, exactly as found;
- HTML comments and inline/block HTML;
- code fences (a `#` inside a fence is code, not a heading);
- YAML/TOML front matter;
- headings that share the same title (each keeps its own identity).

This is verified continuously by a golden test suite that edits **a section
in each fixture document** and asserts the surrounding bytes are untouched
and that undo restores the original file exactly.
