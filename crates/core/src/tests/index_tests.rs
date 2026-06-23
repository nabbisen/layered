//! RFC-003/006/007: heading detection, tree construction, root semantics,
//! and node identity determinism/stability.

use crate::{Document, HeadingLevel, NodeId, SectionNode};

fn doc(source: &str) -> Document {
    Document::parse(source.to_string()).expect("parse")
}

fn titles_in_source_order(document: &Document) -> Vec<String> {
    document
        .outline()
        .iter()
        .skip(1)
        .map(|n| n.title.clone())
        .collect()
}

#[test]
fn atx_headings_build_nested_tree() {
    let document = doc("# A\nbody a\n\n## A1\nbody a1\n\n### A1a\ndeep\n\n## A2\n\n# B\n");
    let outline = document.outline();
    let root_children = outline.children(outline.root_id()).unwrap();
    assert_eq!(
        root_children
            .iter()
            .map(|n| n.title.as_str())
            .collect::<Vec<_>>(),
        ["A", "B"]
    );
    let a = root_children[0];
    let a_children = outline.children(a.id).unwrap();
    assert_eq!(
        a_children
            .iter()
            .map(|n| n.title.as_str())
            .collect::<Vec<_>>(),
        ["A1", "A2"]
    );
    let a1_children = outline.children(a_children[0].id).unwrap();
    assert_eq!(a1_children[0].title, "A1a");
    assert_eq!(a1_children[0].level, Some(HeadingLevel::H3));
}

#[test]
fn headings_inside_code_fences_are_not_indexed() {
    let document = doc("# Real\n\n```\n# not a heading\n## also not\n```\n\n# Also Real\n");
    assert_eq!(titles_in_source_order(&document), ["Real", "Also Real"]);
}

#[test]
fn duplicate_titles_get_distinct_ids() {
    let document = doc("# Same\nfirst\n\n# Same\nsecond\n");
    let ids: Vec<NodeId> = document.outline().root().children.clone();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
    assert_eq!(document.section_body(ids[0]).unwrap(), "first\n\n");
    assert_eq!(document.section_body(ids[1]).unwrap(), "second\n");
}

#[test]
fn skipped_levels_attach_to_nearest_shallower_heading_without_synthetic_nodes() {
    let document = doc("# A\n### B\nbody\n## C\n");
    let outline = document.outline();
    let a = outline.children(outline.root_id()).unwrap()[0];
    let a_children = outline.children(a.id).unwrap();
    // B (H3) and C (H2) are both direct children of A; no synthetic H2 exists.
    assert_eq!(
        a_children
            .iter()
            .map(|n| n.title.as_str())
            .collect::<Vec<_>>(),
        ["B", "C"]
    );
    assert_eq!(a_children[0].level, Some(HeadingLevel::H3));
    assert_eq!(a_children[1].level, Some(HeadingLevel::H2));
    assert_eq!(outline.len(), 4); // root + A + B + C
}

#[test]
fn root_holds_content_before_first_heading() {
    let document = doc("preface line\n\n# First\nbody\n");
    let root = document.outline().root();
    assert!(root.is_root());
    assert_eq!(document.section_body(root.id).unwrap(), "preface line\n\n");
}

#[test]
fn document_without_headings_is_root_only() {
    let document = doc("just text\nno headings here\n");
    assert!(document.outline().is_empty());
    let root = document.outline().root();
    assert_eq!(
        document.section_body(root.id).unwrap(),
        "just text\nno headings here\n"
    );
}

#[test]
fn yaml_front_matter_is_not_misread_as_setext_heading() {
    // Without metadata-block recognition, "title: x" + "---" would parse as a
    // Setext H2. The indexer must treat front matter as root-level content.
    let document = doc("---\ntitle: Example\n---\n\n# Real\nbody\n");
    assert_eq!(titles_in_source_order(&document), ["Real"]);
    let root_body = document.section_body(document.outline().root_id()).unwrap();
    assert!(root_body.contains("title: Example"));
}

#[test]
fn toml_front_matter_is_not_indexed() {
    let document = doc("+++\ntitle = \"Example\"\n+++\n\n# Real\n");
    assert_eq!(titles_in_source_order(&document), ["Real"]);
}

#[test]
fn setext_headings_are_indexed_with_correct_ranges() {
    let source = "Title\n=====\n\nbody\n\nSub\n---\nmore\n";
    let document = doc(source);
    let outline = document.outline();
    let title = outline.children(outline.root_id()).unwrap()[0];
    assert_eq!(title.level, Some(HeadingLevel::H1));
    assert_eq!(title.title, "Title");
    // Heading range spans text line + underline + newline.
    assert_eq!(&source[title.heading_range.as_range()], "Title\n=====\n");
    let sub = outline.children(title.id).unwrap()[0];
    assert_eq!(sub.level, Some(HeadingLevel::H2));
    assert_eq!(document.section_body(sub.id).unwrap(), "more\n");
}

#[test]
fn crlf_documents_index_with_exact_byte_ranges() {
    let source = "# A\r\nbody a\r\n# B\r\nbody b\r\n";
    let document = doc(source);
    let ids = document.outline().root().children.clone();
    assert_eq!(document.section_body(ids[0]).unwrap(), "body a\r\n");
    assert_eq!(document.section_body(ids[1]).unwrap(), "body b\r\n");
}

#[test]
fn inline_markup_in_titles_is_flattened() {
    let document = doc("# Hello *world* `code`\n");
    assert_eq!(titles_in_source_order(&document), ["Hello world code"]);
}

#[test]
fn node_ids_are_deterministic_for_identical_structure() {
    let a = doc("# A\nx\n## B\ny\n");
    let b = doc("# A\nx\n## B\ny\n");
    let ids = |d: &Document| d.outline().iter().map(|n| n.id).collect::<Vec<_>>();
    assert_eq!(ids(&a), ids(&b));
}

#[test]
fn node_ids_depend_on_position_not_title() {
    // Same structure with different titles → same IDs (ordinal-path hash).
    let a = doc("# X\n## Y\n");
    let b = doc("# P\n## Q\n");
    let ids = |d: &Document| d.outline().iter().map(|n| n.id).collect::<Vec<_>>();
    assert_eq!(ids(&a), ids(&b));
}

#[test]
fn body_range_excludes_child_sections() {
    let source = "# A\nown body\n## Child\nchild body\n";
    let document = doc(source);
    let a = document
        .outline()
        .children(document.outline().root_id())
        .unwrap()[0];
    assert_eq!(document.section_body(a.id).unwrap(), "own body\n");
    // Full range covers the child too.
    assert_eq!(&source[a.full_range.as_range()], source);
}

#[test]
fn range_invariants_hold_for_every_node() {
    let document = doc("# A\nbody\n## B\n### C\ndeep\n## D\n# E\n");
    let check = |n: &SectionNode| {
        assert!(n.full_range.start <= n.heading_range.start);
        assert!(n.heading_range.end <= n.body_range.start);
        assert!(n.body_range.end <= n.full_range.end);
    };
    document.outline().iter().for_each(check);
}

#[test]
fn heading_at_eof_without_trailing_newline() {
    let document = doc("# A\nbody\n# B");
    let ids = document.outline().root().children.clone();
    assert_eq!(document.section_body(ids[1]).unwrap(), "");
}
