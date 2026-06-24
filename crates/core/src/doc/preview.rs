//! Markdown preview rendering (RFC-045).
//!
//! Converts source ranges to HTML for read-only display. Implemented with a
//! manual event walker over the pulldown-cmark parser so no additional
//! feature flags or C build-script dependencies are required.
//!
//! Supported elements: headings H1-H6, paragraphs, bold, italic, inline code,
//! fenced code blocks, unordered and ordered lists, block quotes, links,
//! horizontal rules, hard/soft breaks, strikethrough, tables.
//!
//! The source-preservation invariant is unaffected: this module is read-only.

use pulldown_cmark::{Alignment, Event, HeadingLevel, LinkType, Options, Parser, Tag, TagEnd};

// ── helpers ───────────────────────────────────────────────────────────────────

/// Escapes HTML special characters in text content.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn heading_tag(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

// ── rendering ─────────────────────────────────────────────────────────────────

fn render_html(markdown: &str) -> String {
    let opts = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;

    let parser = Parser::new_ext(markdown, opts);
    let mut out = String::with_capacity(markdown.len() * 2);
    // Track table column alignment for rendering <th>/<td> cells.
    let mut table_alignments: Vec<Alignment> = Vec::new();
    let mut table_col: usize = 0;
    let mut in_table_head = false;

    for event in parser {
        match event {
            // ── block elements ────────────────────────────────────────────────
            Event::Start(Tag::Paragraph) => out.push_str("<p>"),
            Event::End(TagEnd::Paragraph) => out.push_str("</p>\n"),

            Event::Start(Tag::Heading { level, .. }) => {
                let n = heading_tag(level);
                out.push_str(&format!("<h{n}>"));
            }
            Event::End(TagEnd::Heading(level)) => {
                let n = heading_tag(level);
                out.push_str(&format!("</h{n}>\n"));
            }

            Event::Start(Tag::BlockQuote(_kind)) => out.push_str("<blockquote>\n"),
            Event::End(TagEnd::BlockQuote(_)) => out.push_str("</blockquote>\n"),

            Event::Start(Tag::CodeBlock(_)) => out.push_str("<pre><code>"),
            Event::End(TagEnd::CodeBlock) => out.push_str("</code></pre>\n"),

            Event::Start(Tag::List(None)) => out.push_str("<ul>\n"),
            Event::End(TagEnd::List(false)) => out.push_str("</ul>\n"),
            Event::Start(Tag::List(Some(_start))) => out.push_str("<ol>\n"),
            Event::End(TagEnd::List(true)) => out.push_str("</ol>\n"),

            Event::Start(Tag::Item) => out.push_str("<li>"),
            Event::End(TagEnd::Item) => out.push_str("</li>\n"),

            Event::Rule => out.push_str("<hr>\n"),

            // ── tables ────────────────────────────────────────────────────────
            Event::Start(Tag::Table(alignments)) => {
                table_alignments = alignments.clone();
                out.push_str("<table>\n");
            }
            Event::End(TagEnd::Table) => out.push_str("</table>\n"),

            Event::Start(Tag::TableHead) => {
                in_table_head = true;
                out.push_str("<thead><tr>\n");
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
                out.push_str("</tr></thead>\n<tbody>\n");
                table_col = 0;
            }
            Event::Start(Tag::TableRow) => {
                out.push_str("<tr>\n");
                table_col = 0;
            }
            Event::End(TagEnd::TableRow) => out.push_str("</tr>\n"),

            Event::Start(Tag::TableCell) => {
                let tag = if in_table_head { "th" } else { "td" };
                let align = match table_alignments.get(table_col) {
                    Some(Alignment::Left) => " style=\"text-align:left\"",
                    Some(Alignment::Right) => " style=\"text-align:right\"",
                    Some(Alignment::Center) => " style=\"text-align:center\"",
                    _ => "",
                };
                out.push_str(&format!("<{tag}{align}>"));
            }
            Event::End(TagEnd::TableCell) => {
                let tag = if in_table_head { "th" } else { "td" };
                out.push_str(&format!("</{tag}>\n"));
                table_col += 1;
            }

            // ── inline elements ───────────────────────────────────────────────
            Event::Start(Tag::Strong) => out.push_str("<strong>"),
            Event::End(TagEnd::Strong) => out.push_str("</strong>"),

            Event::Start(Tag::Emphasis) => out.push_str("<em>"),
            Event::End(TagEnd::Emphasis) => out.push_str("</em>"),

            Event::Start(Tag::Strikethrough) => out.push_str("<del>"),
            Event::End(TagEnd::Strikethrough) => out.push_str("</del>"),

            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                ..
            }) => {
                let href = html_escape(&dest_url);
                let is_plain = link_type == LinkType::Autolink
                    || link_type == LinkType::Email
                    || title.is_empty();
                if is_plain {
                    out.push_str(&format!("<a href=\"{href}\">"));
                } else {
                    let t = html_escape(&title);
                    out.push_str(&format!("<a href=\"{href}\" title=\"{t}\">"));
                }
            }
            Event::End(TagEnd::Link) => out.push_str("</a>"),

            Event::Start(Tag::Image {
                dest_url, title, ..
            }) => {
                let src = html_escape(&dest_url);
                let alt = html_escape(&title);
                out.push_str(&format!("<img src=\"{src}\" alt=\"{alt}\">"));
            }
            Event::End(TagEnd::Image) => {}

            // ── text ──────────────────────────────────────────────────────────
            Event::Text(text) => out.push_str(&html_escape(&text)),
            Event::Code(code) => {
                out.push_str("<code>");
                out.push_str(&html_escape(&code));
                out.push_str("</code>");
            }
            Event::Html(raw) | Event::InlineHtml(raw) => {
                // Pass raw HTML through unchanged (user-authored HTML in Markdown).
                out.push_str(&raw);
            }
            Event::SoftBreak => out.push('\n'),
            Event::HardBreak => out.push_str("<br>\n"),

            Event::TaskListMarker(checked) => {
                if checked {
                    out.push_str("<input type=\"checkbox\" checked disabled> ");
                } else {
                    out.push_str("<input type=\"checkbox\" disabled> ");
                }
            }

            // Ignore footnote definitions in preview for simplicity.
            _ => {}
        }
    }
    out
}

// ── public API ────────────────────────────────────────────────────────────────

/// Returns the focused section body rendered as HTML, or `None` when the
/// node does not exist in the outline.
pub fn section_html(doc: &crate::Document, id: crate::NodeId) -> Option<String> {
    let node = doc.outline().node(id)?;
    let body = doc.source().get(node.body_range.as_range())?;
    Some(render_html(body))
}

/// Returns the full document rendered as a single HTML string, headings
/// included. Useful for whole-document export or reference.
pub fn document_html(doc: &crate::Document) -> String {
    render_html(doc.source())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Document;

    fn doc(md: &str) -> Document {
        Document::parse(md.to_string()).unwrap()
    }

    #[test]
    fn section_html_renders_bold() {
        let d = doc("# A\nHello **world**\n");
        let id = d.outline().root().children[0];
        let html = section_html(&d, id).unwrap();
        assert!(html.contains("<strong>world</strong>"), "{html}");
    }

    #[test]
    fn document_html_includes_heading() {
        let d = doc("# Title\nbody\n");
        let html = document_html(&d);
        assert!(html.contains("<h1>Title</h1>"), "{html}");
    }

    #[test]
    fn empty_body_yields_empty_or_minimal_html() {
        let d = doc("# A\n");
        let id = d.outline().root().children[0];
        let html = section_html(&d, id).unwrap();
        assert!(html.trim().is_empty() || html.contains("<p>"), "{html}");
    }

    #[test]
    fn unknown_node_returns_none() {
        let d = doc("# A\n");
        assert!(section_html(&d, crate::NodeId(999_999_999)).is_none());
    }

    #[test]
    fn japanese_body_does_not_panic() {
        let d = doc("# 日本語\n東京は日本の首都です。\n");
        let id = d.outline().root().children[0];
        let html = section_html(&d, id).unwrap();
        assert!(html.contains("東京"), "{html}");
    }

    #[test]
    fn code_fence_renders_pre_code() {
        let d = doc("# A\n```rust\nfn main() {}\n```\n");
        let id = d.outline().root().children[0];
        let html = section_html(&d, id).unwrap();
        assert!(html.contains("<pre><code>"), "{html}");
    }

    #[test]
    fn html_escapes_angle_brackets() {
        let d = doc("# A\n1 < 2 > 0\n");
        let id = d.outline().root().children[0];
        let html = section_html(&d, id).unwrap();
        assert!(html.contains("&lt;"), "{html}");
        assert!(html.contains("&gt;"), "{html}");
        assert!(!html.contains("<2"), "{html}");
    }
}
