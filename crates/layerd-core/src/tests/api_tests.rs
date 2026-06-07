//! RFC-005/009: public API behavior — focus snapshots, projections, and
//! structured error paths without panics.

use crate::{Document, DocumentError, HeadingLevel, NodeId};

fn doc(source: &str) -> Document {
    Document::parse(source.to_string()).expect("parse")
}

#[test]
fn focus_snapshot_carries_title_body_children_and_path() {
    let document =
        doc("# Chapter\nintro\n\n## Section\nsection body\n\n### Sub\ndeep\n\n## Other\n");
    let outline = document.outline();
    let chapter = outline.children(outline.root_id()).unwrap()[0];
    let section = outline.children(chapter.id).unwrap()[0];

    let snapshot = document.focus_snapshot(section.id).unwrap();
    assert_eq!(snapshot.title, "Section");
    assert_eq!(snapshot.level, Some(HeadingLevel::H2));
    assert_eq!(snapshot.body, "section body\n\n");
    assert_eq!(snapshot.children.len(), 1);
    assert_eq!(snapshot.children[0].title, "Sub");
    assert_eq!(snapshot.children[0].child_count, 0);
    // Breadcrumb path: root → Chapter → Section.
    let path_titles: Vec<&str> = snapshot.path.iter().map(|i| i.title.as_str()).collect();
    assert_eq!(path_titles, ["", "Chapter", "Section"]);
    assert_eq!(snapshot.revision, document.revision());
}

#[test]
fn focus_snapshot_of_root_has_no_level() {
    let document = doc("preface\n\n# A\n");
    let snapshot = document
        .focus_snapshot(document.outline().root_id())
        .unwrap();
    assert_eq!(snapshot.level, None);
    assert_eq!(snapshot.body, "preface\n\n");
    assert_eq!(snapshot.path.len(), 1);
}

#[test]
fn unknown_node_lookups_return_structured_errors() {
    let document = doc("# A\n");
    let missing = NodeId(0xdead_beef);
    assert_eq!(
        document.section_body(missing).unwrap_err(),
        DocumentError::NodeNotFound(missing)
    );
    assert_eq!(
        document.focus_snapshot(missing).unwrap_err(),
        DocumentError::NodeNotFound(missing)
    );
}

#[test]
fn from_bytes_rejects_invalid_utf8_without_panicking() {
    let err = Document::from_bytes(vec![0x23, 0x20, 0xff, 0xfe]).unwrap_err();
    assert_eq!(err, DocumentError::InvalidUtf8);
}

#[test]
fn from_bytes_accepts_valid_utf8() {
    let document = Document::from_bytes("# こんにちは\n".as_bytes().to_vec()).unwrap();
    assert_eq!(document.outline().root().children.len(), 1);
}

#[test]
fn source_is_exactly_what_was_parsed() {
    let source = "---\nfm: yes\n---\n\n# A\r\nbody  \nwith trailing spaces\n\n<!-- comment -->\n";
    let document = doc(source);
    assert_eq!(document.source(), source);
}

#[test]
fn outline_path_and_children_traversal_are_consistent() {
    let document = doc("# A\n## B\n### C\n");
    let outline = document.outline();
    let a = outline.children(outline.root_id()).unwrap()[0].id;
    let b = outline.children(a).unwrap()[0].id;
    let c = outline.children(b).unwrap()[0].id;
    let path: Vec<NodeId> = outline.path(c).unwrap().iter().map(|n| n.id).collect();
    assert_eq!(path, [outline.root_id(), a, b, c]);
}

#[test]
fn empty_document_parses_to_root_only() {
    let document = doc("");
    assert!(document.outline().is_empty());
    assert_eq!(
        document.section_body(document.outline().root_id()).unwrap(),
        ""
    );
}
