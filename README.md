# layerd

**Layer EDitor** — a next-generation text editor that helps you clarify ideas
and refine them, consideration by consideration, **layer by layer**.

layerd treats a Markdown document as a stack of layers: the document outline
is the map, and each section is a layer you can focus on in isolation. You
zoom into one section, refine just that thought, and zoom back out — without
the rest of the document getting in the way, and without the editor ever
rewriting a byte you didn't touch.

## Design principles

1. **The raw Markdown text is the canonical document.** The outline tree is a
   derived index, rebuilt after every committed edit. Saving always writes the
   canonical text back verbatim — never a serialization of an AST.
2. **Editing one section never rewrites unrelated source bytes.** Replacing a
   section body splices exactly that byte range; every byte outside it is
   preserved bit-for-bit (whitespace quirks, HTML comments, code fences,
   CRLF line endings, missing trailing newlines — all of it).
3. **Conflicts are impossible by construction.** Every edit carries the
   document revision it was composed against; stale edits are rejected before
   any mutation (optimistic concurrency, RFC-002/RFC-008).
4. **Undo/redo are first-class, byte-exact mutations** with bounded history
   (RFC-044).
5. **The GUI speaks your language.** UI strings are catalog-based with
   English fallback; English and Japanese ship in the MVP (RFC-043).

## Workspace layout

| Crate | Role |
| --- | --- |
| `crates/layerd-core` | Document engine: canonical text, outline index over `pulldown-cmark`, section-body edits, undo/redo. No GUI dependencies. |
| `crates/layerd-ui` | Renderer-independent GUI logic: editor session, focus navigation with back/forward history, i18n catalogs. |
| `crates/layerd-desktop` | Desktop shell: Dioxus components on the system WebView, file dialogs via `rfd`. |

Design documents live in [`rfcs/`](rfcs/) (see the
[RFC index](rfcs/README.md) and the lifecycle policy in
[`rfcs/done/000-rfc-lifecycle-policy.md`](rfcs/done/000-rfc-lifecycle-policy.md)).
The user guide sources live in [`docs/`](docs/) as an mdBook.

## Building

Requires Rust 1.85+ (edition 2024). Core and UI logic build everywhere:

```sh
cargo build            # builds layerd-core and layerd-ui (default members)
cargo test             # runs the full unit + golden integration suite
```

### Desktop GUI

`layerd-desktop` links the platform WebView, so it is excluded from the
default members. On Linux (Debian/Ubuntu) install the native packages first:

```sh
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev
cargo run -p layerd-desktop
```

Windows (WebView2) and macOS (WKWebView) need no extra packages:

```sh
cargo run -p layerd-desktop
```

## Using layerd

* The left pane lists the top-level sections of the open document.
* Click a section to **focus** it: you see its breadcrumb path, its body in
  an editor, and its direct subsections as cards.
* Edit the body and commit; only that section's bytes change.
* **Back / Forward** retrace your focus history like a browser.
* **Undo / Redo** restore the document text byte-exactly.
* Switch the GUI language (English / 日本語) from the toolbar at any time.

## Documentation

```sh
cargo install mdbook
mdbook serve docs
```

## License

Apache-2.0 — see [LICENSE](LICENSE). Copyright (c) nabbisen.
